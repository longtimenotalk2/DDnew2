pub mod wyrand;
pub mod game;
use crate::wyrand::Dice;
use game::unit::Unit;
use game::team::Team;



fn test3() {
    let dice = Dice::new(114516);
    
    let mut yelin = Unit::new("叶  琳", 16, 14, 12);
    let alyssa = Unit::new("艾丽莎", 11, 15, 16);
    let elis = Unit::new("伊莉丝", 12, 18, 12);

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

fn test5() {
    let mut female = Unit::new("女斗士", 16, 12, 18);
    female.take_stun(2);
    let male = Unit::new("男盗贼", 14, 12, 13);
    let mut team = Team::new(vec!(female), vec!(male), Dice::new(114519));
    // println!("{}", team.main_loop(50).1);

    team.win_rate(20, 200);
}

fn test6() {
    let dice = Dice::new(114514);

    let noel = Unit::new("诺艾尔", 20, 10, 10);
    let yelin = Unit::new("叶  琳", 16, 14, 12);
    let alyssa = Unit::new("艾丽莎", 11, 15, 16);
    let elis = Unit::new("伊莉丝", 12, 18, 12);

    let enemy0 = Unit::new("大  姐", 18, 18, 16);
    let enemy1 = Unit::new("二  姐", 17, 12, 11);
    let enemy2 = Unit::new("三  姐", 12, 12, 12);
    let enemy3 = Unit::new("幺  妹", 8, 12, 14);

    let mut team = Team::new(vec!(noel, elis, alyssa, yelin), vec!(enemy0, enemy1, enemy2), dice);

    print!("{}", team.main_loop(20).1);

    // team.win_rate(20, 200);
}

fn test7() {
  let str1 = 18;
  let skl1 = 13;
  let spd1 = 13;
  let str2 = 13;
  let skl2 = 18;
  let spd2 = 13;
  let dice = Dice::new(114518);
  let a1 = Unit::new("我方 A", str1, skl1, spd1);
  let a2 = Unit::new("我方 B", str1, skl1, spd1);
  let a3 = Unit::new("我方 C", str1, skl1, spd1);
  let a4 = Unit::new("我方 D", str1, skl1, spd1);
  let b1 = Unit::new("敌方 A", str2, skl2, spd2);
  let b2 = Unit::new("敌方 B", str2, skl2, spd2);
  let b3 = Unit::new("敌方 C", str2, skl2, spd2);
  let b4 = Unit::new("敌方 D", str2, skl2, spd2);
  let mut team = Team::new(vec!(a1, a2, a3, a4), vec!(b1, b2, b3, b4), dice);

  // print!("{}", team.main_loop(5).1);

  team.win_rate(20, 200);
}

fn main() {
    println!("Hello, world!");
    test7(); 


    
    
}