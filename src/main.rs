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
    let dice = Dice::new(114516);

    let noel = Unit::new("诺艾尔", 10, 10, 10);
    let yelin = Unit::new("叶琳", 16, 14, 12);
    let alyssa = Unit::new("艾丽莎", 11, 15, 16);
    let elis = Unit::new("伊莉丝", 12, 18, 12);

    let enemy0 = Unit::new("大姐", 18, 18, 16);
    let enemy1 = Unit::new("二姐", 17, 12, 10);
    let enemy2 = Unit::new("三姐", 12, 12, 12);
    let enemy3 = Unit::new("幺妹", 8, 12, 14);

    let mut team = Team::new(vec!(noel, elis, alyssa, yelin), vec!(enemy0, enemy1, enemy2, enemy3), dice);

    team.play();
  
}

fn test7() {
    let noel = Unit::new("诺艾尔", 10, 10, 10);
    let mut yelin = Unit::new("叶琳", 16, 14, 12);
    // yelin.take_stun(2);
    let e1 = Unit::new("杂鱼甲", 9,9,12);
    let e2 = Unit::new("杂鱼乙", 9,12,9);
    let e3 = Unit::new("杂鱼丙", 12,9,9);
    let e4 = Unit::new("杂鱼丁", 12,9,9);
    let dice = Dice::new(114514);

    let mut team = Team::new(vec!( yelin), vec!(e1, e2, e3), dice);

    team.play()
}

fn roll_name(names : &mut Vec<&str>, dice : &mut Dice) -> String {
    let n = dice.d(names.len() as i32) - 1;
    return names.remove(n as usize).to_string();
}

fn roll_attr(a : i32, dice : &mut Dice ) -> Vec<i32> {
    let mut attrs = vec![5, 5, 5];
    for _ in 0..a {
        let n = dice.d(attrs.len() as i32) - 1;
        attrs[n as usize] += 1;
    }
    attrs
}

fn test8() {
    let mut names = vec!["星", "三月七", "艾丝妲", "娜塔莎", "希尔", "布洛妮", "佩拉", "姬子", "黑塔", "克拉拉", "虎克", "御空", "停云", "符玄", "青雀", "素裳", "桂乃芬", "寒鸦", "希露瓦", "可利亚", "玲可", "阮梅", "雪衣", "藿藿", "静流", "托帕", "卡芙卡", "银狼", "白露"];

    let a = 30;
    let b = 25;
    let c = 20;
    let d = 15;
    let df1 = 4;
    let n0 = [d, c, b, a];
    let n1 = [a+df1, b+df1, c+df1, d+df1];

    let seed = 114525;
    let mut di = Dice::new(seed);

    let mut t0 = vec!();
    for a in n0 {
        let attr = roll_attr(a, &mut di);
        t0.push(Unit::new(&roll_name(&mut names, &mut di), attr[0], attr[1], attr[2]));
    }

    let mut t1 = vec!();
    for a in n1 {
        let attr = roll_attr(a, &mut di);
        t1.push(Unit::new(&roll_name(&mut names, &mut di), attr[0], attr[1], attr[2]));
    }

    let mut team = Team::new(t0, t1, Dice::new(seed+233));

    team.play();
    
}


fn test0() {
    let mut a1 = Unit::new("佩拉", 18,12,18);
    let mut a2 = Unit::new("静流", 5, 5, 6);
    let mut b1 = Unit::new("艾丝妲", 5, 5, 12);
    let mut b2 = Unit::new("姬子", 5, 5, 17);
    a1.take_master(3);
    b1.take_ctrl(2);
    b1.take_bound();
    b1.take_bound();
    let mut team = Team::new(vec!(a2, a1), vec!(b1, b2), Dice::new(114514));
    team.play();
}


fn main() {
    println!("Hello, world!");
    test8();
}