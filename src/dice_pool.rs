use std::fmt;
use super::die::Die;

pub enum Operation {
    KeepHighest(u32),
    KeepLowest(u32),
}

pub struct DicePool {
    pub die: Die,
    pub count: u32,
    pub op: Option<Operation>
}

impl DicePool {
    #[must_use]
    pub const fn new(count: u32, sides: u32) -> Self {
        assert!(sides > 0);
        assert!(count > 0);
        
        Self {
            die: Die::new(sides),
            count,
            op: None,
        }
    }

    #[must_use]
    pub const fn new_with_op(count: u32, sides: u32, op: Operation) -> Self {
        assert!(sides > 0);
        assert!(count > 0);
        match op {
            Operation::KeepHighest(keep) | Operation::KeepLowest(keep) => {
                assert!(keep > 0);
                assert!(keep <= count);
            }
        }

        Self {
            die: Die::new(sides),
            count,
            op: Some(op),
        }
    }

    #[must_use]
    pub fn roll_all(&self) -> Vec<u32> {
        (0..self.count).map(|_| self.die.roll()).collect()
    }
}

impl fmt::Display for DicePool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self.count {
            1 => write!(f, "{}", self.die)?,
            _ => write!(f, "{}{}", self.count, self.die)?,
        }
        
        if let Some(op) = &self.op {
            match op {
                Operation::KeepHighest(n) => write!(f, "kh{n}")?,
                Operation::KeepLowest(n) => write!(f, "kl{n}")?,
            }
        }
        
        Ok(())
    }
}