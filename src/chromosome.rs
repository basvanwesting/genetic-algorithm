#[derive(Debug)]
pub struct Chromosome {
    pub genes: Vec<bool>,
    pub fitness: Option<f32>,
}

impl Chromosome {
    pub fn new(genes: Vec<bool>) -> Self {
        Self {
            genes: genes,
            fitness: None,
        }
    }
}
