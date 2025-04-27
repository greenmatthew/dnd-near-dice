use std::fmt;
use crate::die::Die;
use crate::dice_pool::DicePool;
use crate::result::RollResult;

pub enum DiceExpressionComponent {
    DicePool(DicePool),
    Constant(i32),
    // GroupLabel(String), // For future implementation
}

pub struct DiceExpression {
    tokens: Vec<DiceExpressionComponent>,
}

impl DiceExpression {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            tokens: Vec::new(),
        }
    }

    pub fn add_die(&mut self, die: Die) -> &mut Self {
        self.tokens.push(DiceExpressionComponent::DicePool(DicePool::new(die.sides, 1)));
        self
    }

    pub fn add_die_by_sides(&mut self, sides: u32) -> &mut Self {
        self.tokens.push(DiceExpressionComponent::DicePool(DicePool::new(sides, 1)));
        self
    }

    pub fn add_pool(&mut self, pool: DicePool) -> &mut Self {
        self.tokens.push(DiceExpressionComponent::DicePool(pool));
        self
    }

    pub fn add_pool_by_values(&mut self, count: u32, sides: u32) -> &mut Self {
        self.tokens.push(DiceExpressionComponent::DicePool(DicePool::new(sides, count)));
        self
    }

    pub fn add_constant(&mut self, value: i32) -> &mut Self {
        self.tokens.push(DiceExpressionComponent::Constant(value));
        self
    }

    // pub fn add_group(&mut self, label: String) -> &mut Self {
    //     self.tokens.push(DiceExpressionComponent::GroupLabel(label));
    //     self
    // }

    pub fn roll(&self) -> RollResult {
        todo!("Dice expression rolling not implemented yet!")
    }
}

impl fmt::Display for DiceExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.tokens.is_empty() {
            return write!(f, "0");
        }

        let mut first = true;
        
        for token in &self.tokens {
            match token {
                DiceExpressionComponent::DicePool(pool) => {
                    if first {
                        write!(f, "{}", pool)?;
                        first = false;
                    } else {
                        write!(f, "+{}", pool)?;
                    }
                },
                DiceExpressionComponent::Constant(value) => {
                    if *value >= 0 {
                        write!(f, "+{}", value)?;
                    } else {
                        write!(f, "-{}", value.abs())?;
                    }
                    first = false;
                }
            }
        }
        
        Ok(())
    }
}
