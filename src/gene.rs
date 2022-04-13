#[derive(Debug, Clone, Copy)]
pub struct Gene(pub bool);

impl Gene {
    pub fn new(value: bool) -> Self {
        Self(value)
    }

    pub fn mutate(&mut self) {
        self.0 = !self.0;
    }
}
