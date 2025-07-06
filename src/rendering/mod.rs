use std::sync::Arc;

use bytemuck::cast_slice;
use log::debug;
use wgpu::{
    BindGroup, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BlendState, Buffer,
    BufferBindingType, BufferUsages, ColorTargetState, ColorWrites, CommandEncoder, Device,
    DeviceDescriptor, Face, Features, FragmentState, FrontFace, Instance, InstanceDescriptor,
    Limits, MultisampleState, PipelineCompilationOptions, PipelineLayoutDescriptor, PrimitiveState,
    Queue, RenderPassColorAttachment, RenderPipeline, RenderPipelineDescriptor, ShaderStages,
    Surface, SurfaceConfiguration, SurfaceError, SurfaceTexture, TextureView, Trace, VertexState,
    util::{BufferInitDescriptor, DeviceExt},
};
use winit::window::Window;

use crate::{
    ecs::{
        component::{camera::CameraComponent, transform::TransformComponent},
        entity::scene::Scene,
    },
    rendering::{camera::CameraUniform, vertex::Vertex},
};

mod camera;
mod vertex;

// Hardcoded vertices for a triangle
// arramged om counter-clockwise order from top to bottom left to bottom right
// since our render pipeline is configured to use counter-clockwise winding order
const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];

pub struct RenderingService {
    window: Arc<Window>,
    surface: Surface<'static>,
    surface_configuration: SurfaceConfiguration,
    device: Device,
    queue: Queue,
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    is_surface_configured: bool,
    camera_uniform: CameraUniform,
    camera_buffer: Buffer,
    camera_bind_group: BindGroup,
}

impl RenderingService {
    pub async fn new(
        window: Arc<Window>,
        main_camera_component: &CameraComponent,
        main_transform_component: &TransformComponent,
    ) -> anyhow::Result<Self> {
        let window_size = window.inner_size();

        // The instance manages WebGPU resources and provides access to the GPU.
        let instance_descriptor: InstanceDescriptor = InstanceDescriptor {
            #[cfg(target_os = "windows")]
            backends: wgpu::Backends::DX12,
            #[cfg(target_os = "macos")]
            backends: wgpu::Backends::METAL,
            #[cfg(target_os = "linux")]
            backends: wgpu::Backends::VULKAN,
            ..Default::default()
        };
        let instance: Instance = Instance::new(&instance_descriptor);

        // The surface is attached to a window and is used for rendering.
        let surface: Surface<'static> = instance
            .create_surface(window.clone())
            .expect("Failed to create surface");

        // The adapter is the interface to the GPU and provides access to its capabilities.
        let request_adapter_options = wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        };
        let adapter = instance
            .request_adapter(&request_adapter_options)
            .await
            .expect("Failed to request adapter");

        debug!("Using GPU adapter: {}", adapter.get_info().name);

        // The device is the logicl handle to the GPU, and the queue is used to submit commands to the GPU.
        let device_descriptor = DeviceDescriptor {
            label: None,
            required_features: Features::empty(),
            required_limits: Limits::default(),
            memory_hints: Default::default(),
            trace: Trace::Off,
        };
        let (device, queue) = adapter
            .request_device(&device_descriptor)
            .await
            .expect("Failed to request device and queue");

        // The surface capabilities define how the surface can be used for rendering.
        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_capabilities.formats[0]);
        let surface_configuration = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT, // Textures will be writen to the screen
            format: surface_format,
            width: window_size.width,
            height: window_size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        // Configure shaders
        let shader_module_descriptor = wgpu::ShaderModuleDescriptor {
            label: Some("Primary Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shader.wgsl").into()),
        };
        let shader = device.create_shader_module(shader_module_descriptor);

        // Setup the uniform buffer for the camera
        // uniform buffers are used across every invocation of the shaders
        let mut camera_uniform = CameraUniform::new();
        camera_uniform
            .update_view_projection_matrix(&main_camera_component, &main_transform_component);

        debug!("Camera uniform: {:?}", camera_uniform);

        let camera_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Camera Uniform Buffer"),
            contents: cast_slice(&[camera_uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout_descriptor = BindGroupLayoutDescriptor {
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("Camera bind group layout"),
        };
        let camera_bind_group_layout =
            device.create_bind_group_layout(&camera_bind_group_layout_descriptor);
        let camera_bind_group_descriptor = wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("Camera bind group"),
        };
        let camera_bind_group = device.create_bind_group(&camera_bind_group_descriptor);

        // Configure the rendering pipeline
        let render_pipeline_layout_descriptor = PipelineLayoutDescriptor {
            label: Some("Primary Render Pipeline Layout"),
            bind_group_layouts: &[&camera_bind_group_layout],
            push_constant_ranges: &[],
        };
        let render_pipeline_layout =
            device.create_pipeline_layout(&render_pipeline_layout_descriptor);
        let color_target_state = Some(ColorTargetState {
            format: surface_configuration.format, // use the surface format since the fragments will be output there
            blend: Some(BlendState::REPLACE),     // Replaces color instead of blending
            write_mask: ColorWrites::ALL,         // Write to all color channels
        });
        let targets = &[color_target_state];
        let render_pipeline_descriptor = RenderPipelineDescriptor {
            label: Some("Primary Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                buffers: &[Vertex::describe_vertex_buffer_layout()],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                targets: targets,
            }),
            // Configure how the vertices are interpreted
            primitive: PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // Every three vertices form a triangle
                strip_index_format: None,
                front_face: FrontFace::Ccw, // Triangle is facing forward (counter-clockwise)
                cull_mode: Some(Face::Back), // Cull (remove) back-facing triangles
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None, // Not using depth yet
            multisample: MultisampleState {
                count: 1,                         // Use 1 sample per pixel
                mask: !0,                         // Use all samples
                alpha_to_coverage_enabled: false, // disable anti-aliasing
            },
            multiview: None,
            cache: None, // Don't cache shader compilation results
        };
        let render_pipeline = device.create_render_pipeline(&render_pipeline_descriptor);

        // Create a vertex buffer for the hardcoded triangle vertices
        let vertex_buffer_init_descriptor = BufferInitDescriptor {
            label: Some("Hardcoded Triangle Vertex Buffer"),
            contents: cast_slice(VERTICES), // bytemuck is used to cast complex struct types to bytes
            usage: BufferUsages::VERTEX,    // This buffer is used for vertex data
        };
        let vertex_buffer = device.create_buffer_init(&vertex_buffer_init_descriptor);

        Ok(RenderingService {
            window: window.clone(),
            surface: surface,
            surface_configuration: surface_configuration,
            device: device,
            queue: queue,
            render_pipeline: render_pipeline,
            vertex_buffer: vertex_buffer,
            is_surface_configured: false,
            camera_uniform: camera_uniform,
            camera_buffer: camera_buffer,
            camera_bind_group: camera_bind_group,
        })
    }

    pub fn update_camera_uniform(&mut self, scene: &Scene) {
        let main_camera_component = scene.camera_components.get(&1).unwrap();
        let main_transform_component = scene.transform_components.get(&1).unwrap();

        debug!(
            "Updating camera uniform with camera: {:?} and transform: {:?}",
            main_camera_component, main_transform_component
        );

        self.camera_uniform
            .update_view_projection_matrix(main_camera_component, main_transform_component);

        // In order for the shader to use the updated camera uniform,
        // we need to write the updated data to the camera buffer.
        // Offset 0 says to overwrite the entire buffer.
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
    }

    pub fn resize_surface(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.surface_configuration.width = width;
            self.surface_configuration.height = height;
            self.surface
                .configure(&self.device, &self.surface_configuration);
            self.is_surface_configured = true;
        }
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        if !self.is_surface_configured {
            return Ok(());
        }

        // Request a surface texture to render to from the surface.
        let surface_texture_to_render_to: SurfaceTexture = self.surface.get_current_texture()?;

        // Create a texture view for the surface texture.
        let texture_view: TextureView = surface_texture_to_render_to
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Create a command encoder that acts as a buffer that will record rendering commands
        // which will be submitted to the GPU.
        // Think of this as a tape recorder
        let command_encoder_descriptor = wgpu::CommandEncoderDescriptor {
            label: Some("Render Command Encoder"),
        };
        let mut encoder: CommandEncoder = self
            .device
            .create_command_encoder(&command_encoder_descriptor);

        // Begin a render pass, which groups rendering commands together.
        // This is like starting a new recording session on the tape recorder.
        // Scope is required to ensure the render pass is dropped before the encoder is finished.
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Clear Screen Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.25,
                            g: 0.5,
                            b: 1.0,
                            a: 1.0, // Clear to black with full opacity
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            // Set the pipeline for the render pass.
            render_pass.set_pipeline(&self.render_pipeline);

            // Set the bind group for uniform buffers
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

            // Set the vertex buffer to use for rendering.
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

            // Draw a triangle using the render pipeline.
            render_pass.draw(0..VERTICES.len() as u32, 0..1);
        }

        // Submit the recorded commands to the GPU.
        // This is like pressing play on the tape recorder to execute the recorded commands.
        self.queue.submit(std::iter::once(encoder.finish()));

        // Present the rendered frame to the surface.
        surface_texture_to_render_to.present();

        Ok(())
    }
}
