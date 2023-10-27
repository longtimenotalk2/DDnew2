pub mod wyrand;
pub mod game;
use crate::wyrand::Dice;
use game::unit::Unit;
use game::solo::Solo;

fn test1() {
    let dice = Dice::new(114514);
    let unit0 = Unit::new(15, 13, 13);
    let unit1 = Unit::new(13, 13, 15);

    let mut solo = Solo::new(unit0, unit1, dice);
    for _ in 0..4 {
        let (s, i) = solo.main_loop();
        print!("{}", s);
        println!("{}", i);
        solo.reset();
    }
}

fn test2() {
    let dice = Dice::new(114514);
    let unit0 = Unit::new(13, 15, 13);
    let unit1 = Unit::new(13, 13, 15);

    let times = 400;
    let mut solo = Solo::new(unit0, unit1, dice);
    let list =solo.main_loop_times(times);
    let mut count = 0;
    for i in 0..times {
        count += list[i];
        print!("{}", list[i]);
    }
    println!("\n{}", count);
}


fn main() {
    println!("Hello, world!");
    test1();

    
    
}