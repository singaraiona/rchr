use chr::constraints::Constraint;

#[derive(Clone)]
pub struct Store {
    pub store: Vec<Constraint>,
}

impl Store {
    pub fn new() -> Self {
        Store { store: Vec::new() }
    }

    pub fn push(&mut self, c: Constraint) {
        self.store.push(c);
    }
}