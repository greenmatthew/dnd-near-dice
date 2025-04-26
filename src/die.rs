use std::fmt;
use rand::rng;
use rand::Rng;

pub struct Die {
    pub sides: u32
}

impl Die {
    pub fn new(sides: u32) -> Self {
        Self {
            sides
        }
    }

    pub fn roll(&self) -> u32 {
        let mut rng = rng();
        rng.random_range(1..=self.sides)
    }
}

impl fmt::Display for Die {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "d{}", self.sides)
    }
}