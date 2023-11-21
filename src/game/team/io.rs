// use super::super::unit::Unit;
// use crate::wyrand::Dice;
// use std::fmt::Write;
use super::choose_skill::Skill;

use super::*;
use std::io;

impl Team {
  pub fn io_unit(&self, ids : &[u32], can_wait : bool) -> Option<u32> {
    if ids.len() == 1 && !can_wait {
      return Some(ids[0])
    }
    println!("请选择希望行动的角色：") ;
    for (_, id) in ids.iter().enumerate() {
      let u = &self.pos_pawn(self.id_pos(*id)).unwrap().unit;
      println!("{} : {}", id, u.name);
    }

    if can_wait {
      println!("0 : 等待");
    }

    
    loop {
      let mut ops = String::new();
      io::stdin().read_line(&mut ops).expect("failed to read line");
      if &ops == "\n" {
        return Some(self.ai_unit(ids))
      }
      if let Ok(op) = ops.trim().parse::<u32>() {
        if ids.contains(&op) {
          return Some(op);
        } else if can_wait && op == 0 {
          return None;
        } else {
          println!("输入错误,请输入所给选项前面的数字");
        }
      }else {
        println!("输入错误,请输入一个自然数");
      }
    }
  }

  pub fn io_skill(&self, id : u32, mut skls : Vec<Skill>) -> Skill {
    for (i, skl) in skls.iter().enumerate() {
      match skl {
        Skill::Punch(p) => {
          let u = &self.pos_pawn(*p).unwrap().unit;
          println!("{} : 挥拳 -> {}{}", i, u.name, u.id);
        },
        Skill::Kick(p) => {
          let u = &self.pos_pawn(*p).unwrap().unit;
          println!("{} : 踢腿 -> {}{}", i, u.name, u.id);
        },
        Skill::Ctrl(p) => {
          let u = &self.pos_pawn(*p).unwrap().unit;
          println!("{} : 压制并捆绑 -> {}{}", i, u.name, u.id);
        },
        Skill::Untie(p) => {
          let u = &self.pos_pawn(*p).unwrap().unit;
          println!("{} : 解绑 -> {}{}", i, u.name, u.id);
        },
        Skill::UntieSelf => {
          println!("{} : 解绑自己", i);
        },
        Skill::Escape => {
          println!("{} : 挣脱束缚", i);
        },
        Skill::CtnCtrl => {
          println!("{} : 维持压制", i);
        },
        Skill::Pass => {
          println!("{} : 放弃行动", i);
        },
      }
    }
    
    loop {
      let mut ops = String::new();
      io::stdin().read_line(&mut ops).expect("failed to read line");
      if &ops == "\n" {
        return self.ai_skill(id, skls);
      }
      if let Ok(op) = ops.trim().parse::<usize>() {
        if op < skls.len() {
          return skls.remove(op);
        } else {
          println!("输入错误,请输入所给选项前面的数字");
        }
      }else {
        println!("输入错误,请输入一个自然数");
      }
    }
  }
}

