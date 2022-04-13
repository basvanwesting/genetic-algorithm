#[derive(Debug, Clone, Copy)]
pub struct Gene {
    pub value: bool,
}

impl Gene {
    pub fn new(value: bool) -> Self {
        Self { value: value }
    }

    pub fn mutate(&mut self) {
        self.value = !self.value;
    }
}
