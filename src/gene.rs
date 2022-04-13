pub trait GeneTrait: Copy + Clone {}

impl GeneTrait for u8 {}
impl GeneTrait for bool {}

#[derive(Debug, Clone, Copy)]
pub struct Gene<T: GeneTrait>(pub T);

//impl<T: GeneTrait> Gene<T> {
//pub fn new(value: T) -> Self {
//Self(value)
//}
//}

impl Gene<bool> {
    pub fn new(value: bool) -> Self {
        Self(value)
    }

    pub fn mutate(&mut self) {
        self.0 = !self.0;
    }
}

impl Gene<u8> {
    pub fn new(value: u8) -> Self {
        Self(value)
    }

    pub fn mutate(&mut self) {
        self.0 += 1;
    }
}
