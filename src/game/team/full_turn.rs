// use super::super::unit::Unit;
// use crate::wyrand::Dice;
// use std::fmt::Write;
// use super::choose_skill::Skill;

use super::*;

impl Team {
  pub fn play(&mut self) {
    let is_load = if let Ok(t) = Self::load_from_file() {
      *self = t;
      true
    } else {
      false
    };
    let r = self.loop_turn(100, true, true, is_load);
    match r {
      Some(0) => println!("你输了,游戏结束"),
      Some(1) => println!("你赢了,游戏结束"),
      _ => println!("超时,游戏结束"),

    }
    Self::delete_file().unwrap();
  }
  
  pub fn loop_turn(&mut self, n: i32, o : bool, ai_1 : bool, mut is_load : bool) -> Option<u8> {
    for _ in 0..n {
      self.full_turn(o, ai_1, is_load);
      is_load = false;
      if let Some(i) = self.is_end() {
        return Some(i)
      }
    }
    None
  }
  
  pub fn full_turn(&mut self, o : bool, ai_1 : bool, is_load : bool) {
    if is_load == false {
      self.turn += 1;
      self.spd_now = None;
      self.pre(o);
    }
    loop {
      if self.sub_turn(o, ai_1) == false {
        self.end(o);
        return
      }
    }
  }

  fn end(&mut self, o : bool) {
    let mut s = String::new();
    // 捆绑阶段
    s += "\n";
    for i in 0..self.board.len() {
      let i : usize = i.try_into().unwrap();
      let u = &self.board[i].unit;
      if let Some(it) = u.mastered_id() {
      let pt = self.id_pos(it);
      let ut = &self.pos_pawn(pt).unwrap().unit;
      let point = if u.skl() == 0 {
        0
      } else if u.skl() >= 10 && (ut.stun() > 0 || ut.antibound_lv() + 2 - u.str_lv() <= 0) {
        2
      } else {
        1
      };
      if point == 0 {
        s += "技术为0，无法捆绑\n";
      } else {
        writeln!(s, "{} 对 {} 进行了捆绑", u.name, ut.name).unwrap();
        let txt = self.pos_pawn_mut(pt).unwrap().unit.take_bounds(point);
        writeln!(s, "依次捆绑了 {}部位", txt).unwrap();
        }
      }
    }
    if o {print!("{}", s);}
  }

  fn pre(&mut self, o : bool) {
    let mut s = String::new();
    // 恢复
    for pw in &mut self.board {
      pw.unit.recover();
    }

    // 尝试挣扎
    s += "\n";
    for i in 0..self.board.len() {
      let i : usize = i.try_into().unwrap();
      let u = &self.board[i].unit;
      let p = self.id_pos(u.id);
      if let Some(it) = u.mastered_id() {
        let pt = self.id_pos(it);
        let ut = &self.pos_pawn(pt).unwrap().unit;
        if ut.is_stun() {
          writeln!(s, "{} 昏迷中, 无法挣扎", ut.name).unwrap();
        }else if ut.defeated() {
          writeln!(s, "{} 已被锁死, 无法挣扎", ut.name).unwrap();
        }else if ut.struggle_lv() == 0 {
          writeln!(s, "{} 挣扎力为 0, 无法挣扎", ut.name).unwrap();
        }else  {
          if u.str_lv() <= ut.struggle_lv() {
          writeln!(s, "{} 挣脱了 {} 的压制", ut.name, u.name).unwrap();
          self.cancel_ctrl(p);
          } else if u.str_lv() == ut.struggle_lv() + 1 {
          let dice = self.dice.d(100);
          let u = &self.board[i].unit;
          let ut = &self.pos_pawn(pt).unwrap().unit;
          write!(s, "掷骰 {dice}, ").unwrap();
          if dice <= 50 {
            writeln!(s, "{} 挣脱了 {} 的压制", ut.name, u.name).unwrap();
            self.cancel_ctrl(p);
          } else {
            writeln!(s, "{} 未能挣脱 {} 的压制", ut.name, u.name).unwrap();
          }
          } else {
          writeln!(s, "{} 挣扎力远低于 {} 的压制力, 无法挣扎", ut.name, u.name).unwrap();
          }
        }
      }
    }
    // 获取行动
    for pw in &mut self.board {
      pw.unit.refresh_action();
    }

    if o {print!("{}", s);}
  }
}