const RED : &str = "\u{1b}[31m";
const GREEN : &str = "\u{1b}[32m";
const YELLOW : &str = "\u{1b}[33m";
const BLUE : &str = "\u{1b}[34m";
const RESET : &str = "\u{1b}[m";

use super::unit::Unit;
use super::team::Pawn;

pub enum Color {
  None,
  Red,
  Blue,
  Yellow,
  Green,
}

pub fn draw_board(pawns : &[Pawn], active_ids : &[u32], next_ids : &[u32]) -> Vec<String> {
  let mut list : Vec<Vec<String>> = vec!();
  // 左边框
  let mut zuo = vec!();
  zuo.push(c("力", &Color::Red));
  zuo.push(c("技", &Color::Blue));
  zuo.push(c("速", &Color::Green));
  zuo.push(c("伤", &Color::None));
  zuo.push(c("破", &Color::None));
  zuo.push(format!(" {}", zbfg(0, 22)));
  for _ in 0..7 {
    zuo.push(format!(" {}", zbfz(1, 2)));
  }
  zuo.push(format!(" {}", zbfg(2, 22)));

  list.push(zuo);

  // 主体
  for (i, p) in pawns.iter(). enumerate() {
    let id = p.id;
    let team = p.team;
    let active = active_ids.contains(&id);
    let next = next_ids.contains(&id);
    let ctrl_lr = if let Some(master_id) = p.unit.ctrled_id() {
      let mut im = 0;
      for (i, p) in pawns.iter(). enumerate() {
        if p.id == master_id {
          im = i;
        }
      }
      Some(im > i)
    } else {
      None
    };
    list.push(draw_unit(
      &p.unit,
      team,
      active,
      next,
      ctrl_lr,
    ))
  }

  // 右边框
  let mut zuo = vec!();
  zuo.push(c("力", &Color::Red));
  zuo.push(c("技", &Color::Blue));
  zuo.push(c("速", &Color::Green));
  zuo.push(c("伤", &Color::None));
  zuo.push(c("破", &Color::None));
  zuo.push(format!("{} ", zbfg(1, 22)));
  for _ in 0..7 {
    zuo.push(format!("{} ", zbfz(1, 2)));
  }
  zuo.push(format!("{} ", zbfg(3, 22)));

  list.push(zuo);
  
  // 重组
  vvs2vs(list)
}

pub fn draw_unit(
  u : &Unit, 
  team : u8,
  active : bool,
  next : bool,
  ctrl_lr : Option<bool>,
) -> Vec<String> {
  let mut line1 = String::new();
  let mut line2 = String::new();
  let mut line3 = String::new();
  let mut line4 = String::new();
  let mut line5 = String::new();

  // 边框颜色
  let bc = Color::None;
  // 边框样式
  let sh = if active {3} else if next {2} else if u.action() {1} else {0};
  let ss = if active {1} else if next {1} else if u.action() {1} else {0};
  let bd1 = if u.arm {"≈"} else if u.wrist {"~"} else {""};
  let bd2 = if u.lock {"≈"} else if u.leg {"~"} else {""};
  // 左侧
  if u.ctrled() || u.stun() > 0 || u.defeated() {
    line1 += " ";
    line2 += &c(if ctrl_lr == Some(false) {"╮"} else {" "}, &bc);
    line3 += &c(if bd1 == "" {" "} else {bd1}, &bc);
    line4 += &c(if bd2 == "" {" "} else {bd2}, &bc);
    line5 += " ";
  } else {
    line1 += &c(zbfg(0,sh), &bc);
    line2 += &c(if ctrl_lr == Some(false) {"╮"} else {zbfz(1,ss)}, &bc);
    line3 += &c(if bd1 == "" {zbfz(1,ss)} else {bd1}, &bc);
    line4 += &c(if bd2 == "" {zbfz(1,ss)} else {bd2}, &bc);
    line5 += &c(zbfg(2,sh), &bc);
  }

  // 中间边框
  if u.ctrled() || u.stun() > 0 || u.defeated() {
    let stun = u.stun();
    if stun > 0 {line1 += &format!("@{}", stun)} else {line1 += "  "};
    line5 += "  ";
  } else {
    line1 += &c(zbfz(0,sh), &bc);
    line1 += &c(zbfz(0,sh), &bc);
    line5 += &c(zbfz(0,sh), &bc);
    line5 += &c(zbfz(0,sh), &bc);
  }

  // 中间内容
  let tc = if u.defeated() {Color::None} else if team == 0 {Color::Blue} else {Color::Red};
  let name = &u.name;
  match name.len() {
    3 => {
      line2 += "  "; 
      line3 += &c(name, &tc);
      line4 += "  ";
    },
    6 => {
      line2 += &c(&name[0..3], &tc);
      line3 += "  ";
      line4 += &c(&name[3..6], &tc);
    },
    9 => {
      line2 += &c(&name[0..3], &tc);
      line3 += &c(&name[3..6], &tc);
      line4 += &c(&name[6..9], &tc);
    },
    _ => panic!(),
  }

  // 右侧边框
  if u.ctrled() || u.stun() > 0 || u.defeated() {
    line1 += " ";
    line2 += &c(if ctrl_lr == Some(true) {"╭"} else {" "}, &bc);
    line3 += &c(if bd1 == "" {" "} else {bd1}, &bc);
    line4 += &c(if bd2 == "" {" "} else {bd2}, &bc);
    line5 += " ";
  } else {
    line1 += &c(zbfg(1,sh), &bc);
    line2 += &c(if ctrl_lr == Some(true) {"╭"} else {zbfz(1,ss)}, &bc);
    line3 += &c(if bd1 == "" {zbfz(1,ss)} else {bd1}, &bc);
    line4 += &c(if bd2 == "" {zbfz(1,ss)} else {bd2}, &bc);
    line5 += &c(zbfg(3,sh), &bc);
  }

  let mut def = -3;
  if u.can_def() {
    def = u.skl_lv() - u.broke() - if u.mastered() {1} else {0};
  }
  let line_def = star(def).to_string();
  
  let line_pir = if (active || next) && u.can_kick() {
    star(u.skl_lv()).to_string()
  } else {
    "    ".to_string()
  };

  let str = c(&format!("{:^4}", u.str()), &Color::Red);
  let skl = c(&format!("{:^4}", u.skl()), &Color::Blue);
  let spd = c(&format!("{:^4}", u.spd()), &Color::Green);
  let hurt = if u.hurt() > 0 {format!("{:^4}", u.hurt())} else {"    ".to_string()};
  let broke = if u.broke() > 0 {format!("{:^4}", u.broke())} else {"    ".to_string()};

  let border = zbfz(0, 2).repeat(4).to_string();
  

  if u.block() {
    vec![str, skl, spd, hurt, broke, border.clone(), line_pir, line1, line2, line3, line4, line5, line_def, border]
  } else {
    vec![str, skl, spd, hurt, broke, border.clone(), line_pir, "    ".to_string(), line1, line2, line3, line4, line_def, border]
  }
  
}

fn vvs2vs(vvs : Vec<Vec<String>>) -> Vec<String> {
  let mut lines = vec!();
  for i in 0..vvs[0].len() {
    let mut line = String::new();
    for j in 0..vvs.len() {
      line += &vvs[j][i];
    }
    lines.push(line);
  }
  lines
}

fn star(n : i32) -> &'static str {
  match n {
    1 => " .  ",
    2 => " .. ",
    3 => " :. ",
    4 => " :: ",
    _ => "    ",
  }
}

fn d2cd(d : i32) -> String {
  if d == 0 {"X ".to_string()} else {
    let big = d / 5;
    let small = d - big * 5;
    let c = match big {
      0 => "D",
      1 => "C",
      2 => "B",
      3 => "A",
      4 => "S",
      _ => "?",
    };
    if small > 0 {
      format!("{c}{small}")
    } else {
      format!("{c} ")
    }
    
  }
}

fn c(s : &str, color : &Color) -> String {
  let mut r = String::new();
  match color {
    Color::None => return s.to_string(),
    Color::Red => r += RED,
    Color::Blue => r += BLUE,
    Color::Yellow => r += YELLOW,
    Color::Green => r += GREEN,
  }
  r += s;
  r += RESET;
  r
}

/*
─
━
│
┃
╌
╍╎╏┄┅┆┇┈┉┊
┋┌
┍
┎┏┐
┑
┒┓└
┕
┖┗┘
┙
┚┛├┝┞┟┠┡┢┣┤┥┦┧┨┩┪┫┬┭┮┯┰┱┲┳┴┵┶┷┸┹┺┻┼┽┾┿╀╁╂╃╄╅╆╇╈╉╊╋╪╫╬═
║
╒
╓╔
╕
╖╗╘
╙╚╛
╜╝╞╟╠╡╢╣╤╥╦╧╨╩╔╗
╝
╚
╬
═╓╩┠┨┯┷┏┓┗┛┳⊥﹃﹄┌╮
╭
╯╰╳
.
:.
::
*/

fn zbfg(wz : u8, sh : u8) -> &'static str {
  match wz {
    0 => {
      match sh {
        0 => "┌",
        1 => "┌",
        2 => "╒",
        3 => "┍",
        22 => "╔",
        _ => panic!(),
      }
    },
    1 => {
      match sh {
        0 => "┐",
        1 => "┐",
        2 => "╕",
        3 => "┑",
        22 => "╗",
        _ => panic!(),
      }
    },
    2 => {
      match sh {
        0 => "└",
        1 => "└",
        2 => "╘",
        3 => "┕",
        22 => "╚",
        _ => panic!(),
      }
    },
    3 => {
      match sh {
        0 => "┘",
        1 => "┘",
        2 => "╛",
        3 => "┙",
        22 => "╝",
        _ => panic!(),
      }
    },
    _ => panic!(),
  }
}

fn zbfz(hs : u8, st : u8) -> &'static str {
  match hs {
    0 => {
      match st {
        0 => " ",
        1 => "─",
        2 => "═",
        3 => "━",
        _ => panic!(),
      }
    },
    1 => {
      match st {
        0 => " ",
        1 => "│",
        2 => "║",
        3 => "┃",
        _ => panic!(),
      }
    },
    _ => panic!(),
  }
}