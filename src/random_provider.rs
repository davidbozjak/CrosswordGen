use rand::Rng;


pub trait RandomProvider {
    fn next_u32(& mut self) -> u32;
}

pub struct DefaultRandomProvider {
    rng: rand::rngs::SmallRng,
}

impl DefaultRandomProvider {
    pub fn new() -> Self {
        Self { rng: rand::make_rng() }
    }
}

impl RandomProvider for DefaultRandomProvider {
    fn next_u32(& mut self) -> u32 {
        self.rng.next_u32()
    }
}

pub struct FakeRandomProvider {
    values: Vec<u32>,
    index: usize,
}

impl FakeRandomProvider {
    pub fn new(values: Vec<u32>) -> Self {
        Self { values, index: 0 }
    }
}

impl RandomProvider for FakeRandomProvider {
    fn next_u32(& mut self) -> u32 {
        let v = self.values[self.index];
        self.index += 1;
        v
    }
}
