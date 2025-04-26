use dnd_near_dice::die::Die;

fn main() {
    let d20 = Die::new(20);
    println!("Rolling a {}: {}", d20, d20.roll());
    
    let d6 = Die::new(6);
    println!("Rolling a {}: {}", d6, d6.roll());
}