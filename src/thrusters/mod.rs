use super::input_buffer::InputBuffer;

pub enum NetThrust {
    Down,
    Left,
    Right,
    DownLeft,
    DownRight,
    None,
}

pub struct Thrusters {
    pub acceleration_due_to_thrusters: f64,
}

impl Thrusters {
    pub fn new(acceleration_due_to_thrusters: f64) -> Thrusters {
        Thrusters {
            acceleration_due_to_thrusters,
        }
    }
    pub fn net_thrust(&self, input: &InputBuffer) -> NetThrust {
        match input {
            InputBuffer {
                down: true,
                left: false,
                right: false,
            } => NetThrust::Down,
            InputBuffer {
                down: true,
                left: true,
                right: true,
            } => NetThrust::Down,
            InputBuffer {
                down: false,
                left: true,
                right: false,
            } => NetThrust::Left,
            InputBuffer {
                down: false,
                left: false,
                right: true,
            } => NetThrust::Right,
            InputBuffer {
                down: true,
                left: true,
                right: false,
            } => NetThrust::DownLeft,
            InputBuffer {
                down: true,
                left: false,
                right: true,
            } => NetThrust::DownRight,
            _ => NetThrust::None,
        }
    }
}
