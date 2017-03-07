

#[derive(Clone, Copy, Debug)]
pub struct Constraint {
    pub vtype: u8,
    pub value: u8,
    pub activated: bool,
}

impl Constraint {
    pub fn new(t: u8) -> Self {
        Constraint {
            vtype: t,
            value: 0,
            activated: false,
        }
    }
}