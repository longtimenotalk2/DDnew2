pub mod wyrand;
pub mod game;
use crate::wyrand::Dice;
use game::unit::Unit;
use game::solo::Solo;


fn main() {
    println!("Hello, world!");
    let dice = Dice::new(114514);
    let unit0 = Unit::new(14, 12, 10);
    let unit1 = Unit::new(11, 15, 16);

    let mut solo = Solo::new(unit0, unit1, dice);
    for i in 0..5 {
        solo.show();
        solo.punch(1);
        solo.show();
        solo.punch(0);
    }
    
    
}