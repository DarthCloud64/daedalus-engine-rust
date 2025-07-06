use bytemuck::{Pod, Zeroable};
use wgpu::{BufferAddress, VertexAttribute, VertexBufferLayout, VertexFormat};

const ATTRIBUTES: &[VertexAttribute] = &[
    VertexAttribute {
        offset: 0, // Start from the beginning of the struct at the position field
        shader_location: 0,
        format: VertexFormat::Float32x3, // Position is a 32 bit float vector of 3 components
    },
    VertexAttribute {
        offset: 12, // Skip over the position field and start at the color field
        shader_location: 1,
        format: VertexFormat::Float32x3, // Color is also a 32 bit float vector of 3 components
    },
];

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    pub fn describe_vertex_buffer_layout() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as BufferAddress, // How wide a single Vertex is in memory
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: ATTRIBUTES,
        }
    }
}
