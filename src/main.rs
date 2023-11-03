pub mod wyrand;
pub mod game;
use crate::wyrand::Dice;
use game::unit::Unit;
use game::team::Team;

// fn test1() {
//     let dice = Dice::new(114517);
//     let unit0 = Unit::new(18, 11, 13);
//     let unit1 = Unit::new(11, 15, 16);

//     let mut solo = Solo::new(unit0, unit1, dice);
//     for _ in 0..4 {
//         let (s, i) = solo.main_loop();
//         print!("{}", s);
//         println!("{}", i);
//         solo.reset();
//     }
// }

// fn test2() {
//     let dice = Dice::new(114514);
//     let unit0 = Unit::new(18, 11, 13);
//     let unit1 = Unit::new(11, 15, 16);

//     let times = 400;
//     let mut solo = Solo::new(unit0, unit1, dice);
//     let list =solo.main_loop_times(times);
//     let mut count = 0;
//     let mut win = 0;
//     for i in 0..times {
//         if list[i] <= 1 {
//             count += 1;
//             win += list[i];
//             print!("{}", list[i]);
//         }
//     }
//     let r = 100. * win as f64 / count as f64;
//     println!("\n{win}/{count}, {r}%");
// }

fn test3() {
    let dice = Dice::new(114516);
    
    let mut yelin = Unit::new("叶  琳", 16, 14, 12);
    let alyssa = Unit::new("艾丽莎", 11, 15, 16);
    let elis = Unit::new("伊莉丝", 12, 18, 12);

    // yelin.take_bound();
    // yelin.take_bound();
    // yelin.take_bound();
    // yelin.take_stun(1);
    // yelin.take_ctrl(1);

    let enemy0 = Unit::new("头  目", 16, 18, 20);
    let enemy1 = Unit::new("杂  鱼", 9, 10, 12);

    let mut team = Team::new(vec!(elis, alyssa, yelin), vec!(enemy0, enemy1), dice);

    // print!("{}", team.main_loop(20).1);

    team.win_rate(20, 200);
}

fn test4() {
    let mut yelin = Unit::new("叶小琳", 16, 14, 12);
    println!("{}", yelin.state());
    yelin.take_bound();
    yelin.take_bound();
    yelin.take_bound();
    yelin.take_stun(1);
    yelin.take_ctrl(1);
    println!("{}", yelin.state());
}



fn main() {
    println!("Hello, world!");
    test3();


    
    
}