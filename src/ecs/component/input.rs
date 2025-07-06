#[derive(Debug, Default, Clone, Copy)]
pub struct InputComponent {
    pub moving_up: bool,
    pub moving_down: bool,
    pub moving_left: bool,
    pub moving_right: bool,
}
