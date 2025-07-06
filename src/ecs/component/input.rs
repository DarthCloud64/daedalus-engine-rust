#[derive(Debug, Default, Clone, Copy)]
pub struct InputComponent {
    pub moving_forward: bool,
    pub moving_backward: bool,
    pub moving_left: bool,
    pub moving_right: bool,
}
