use dnd_near_dice::die::Die;
use dnd_near_dice::dice_pool::{DicePool, Operation};
use dnd_near_dice::expression::DiceExpression;

fn main() {
    let d20 = Die::new(20);
    println!("Rolling a {}: {}", d20, d20.roll());
    
    let d6 = Die::new(6);
    println!("Rolling a {}: {}", d6, d6.roll());

    let p2d6 = DicePool::new(6, 2);
    println!("Rolling {p2d6}: {:?}", p2d6.roll_all());

    let mut exp = DiceExpression::new();
    exp.add_pool_by_values_with_op(2, 20, Operation::KeepHighest(1))
        .add_constant(7);
    
    println!("{exp}");

    let mut exp = DiceExpression::new();
    exp.add_pool_by_values(2, 6)
        .add_constant(4);
    
    println!("{exp}");

    let mut exp = DiceExpression::new();
    exp.add_pool_by_values(1, 8);
    
    println!("{exp}");

    let mut exp = DiceExpression::new();
    exp.add_pool_by_values(2, 6)
        .add_constant(4)
        .add_die_by_sides(8);
    
    println!("{exp}");
}