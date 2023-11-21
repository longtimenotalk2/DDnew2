pub mod wyrand;
pub mod game;
use crate::wyrand::Dice;
use game::unit::Unit;
use game::team::Team;


fn test5() {
    let mut female = Unit::new("女斗士", 16, 12, 18);
    female.take_stun(2);
    let mut female2 = Unit::new("女斗士", 16, 12, 18);
    female2.take_stun(99);
    let male = Unit::new("男盗贼", 14, 12, 13);
    let mut team = Team::new(vec!(male), vec!(female, female2), Dice::new(114519));
  

    team.play();
}

fn test6() {
    let dice = Dice::new(114514);

    let noel = Unit::new("诺艾尔", 20, 10, 10);
    let yelin = Unit::new("叶  琳", 16, 14, 12);
    let alyssa = Unit::new("艾丽莎", 11, 15, 16);
    let elis = Unit::new("伊莉丝", 12, 18, 12);

    let enemy0 = Unit::new("大  姐", 18, 18, 16);
    let enemy1 = Unit::new("二  姐", 17, 12, 10);
    let enemy2 = Unit::new("三  姐", 12, 12, 12);
    let enemy3 = Unit::new("幺  妹", 8, 12, 14);

    let mut team = Team::new(vec!(noel, elis, alyssa, yelin), vec!(enemy0, enemy1, enemy2, enemy3), dice);

    // print!("{}", team.main_loop(20).1);

    // team.win_rate(20, 200);

    team.loop_turn(100, true, true);
    println!("End");
  
}


fn main() {
    println!("Hello, world!");
    test5(); 


    
    
}