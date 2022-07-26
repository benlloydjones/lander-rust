pub struct Gravity {
    pub acceleration_due_to_gravity: f64,
}

impl Gravity {
    pub fn new(acceleration_due_to_gravity: f64) -> Gravity {
        Gravity {
            acceleration_due_to_gravity,
        }
    }
}
