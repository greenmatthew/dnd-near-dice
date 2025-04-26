use std::fmt;
use super::die::Die;

pub struct DicePool {
    pub die: Die,
    pub count: u32,
}

impl DicePool {
    #[must_use]
    pub const fn new(sides: u32, count: u32) -> Self {
        assert!(sides > 0);
        assert!(count > 0);
        Self {
            die: Die::new(sides),
            count,
        }
    }

    #[must_use]
    pub fn roll_all(&self) -> Vec<u32> {
        (0..self.count).map(|_| self.die.roll()).collect()
    }
}

impl fmt::Display for DicePool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.count, self.die)
    }
}