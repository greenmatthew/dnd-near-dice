use std::fmt;
use rand::rng;
use rand::Rng;

pub struct Die {
    pub sides: u32
}

impl Die {
    #[must_use]
    pub const fn new(sides: u32) -> Self {
        Self {
            sides
        }
    }

    #[must_use]
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