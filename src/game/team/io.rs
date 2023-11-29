// use super::super::unit::Unit;
// use crate::wyrand::Dice;
use std::fmt::Write;
use super::choose_skill::Skill; 
use super::ai::skill_pos;

use std::collections::HashMap;

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
      print!("{} : {}", id, u.name);
      // 行动提示：控制目标，攻击穿透目标
      println!("({})", self.skill_hint(*id));
    }

    if can_wait {
      print!("0 : 等待");
      // 行动提示
      println!("({})", self.enemy_skill_hint());
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
    let ua = &self.pos_pawn(self.id_pos(id)).unwrap().unit;
    for (i, skl) in skls.iter().enumerate() {
      match skl {
        Skill::Punch(p) => {
          let u = &self.pos_pawn(*p).unwrap().unit;
          let ana = attack_analyse(ua, u, Attack::Punch);
          let pir = if ana.pierce {"√"} else {"×"};
          println!("{} : 挥拳 -> {} (命{}% 穿{pir} 爆{} 伤{})", i, u.name, ana.hit, ana.cri, ana.dmg_normal);
        },
        Skill::Kick(p) => {
          let u = &self.pos_pawn(*p).unwrap().unit;
          let ana = attack_analyse(ua, u, Attack::Kick);
          let pir = if ana.pierce {"√"} else {"×"};
          println!("{} : 踢腿 -> {} (命{}% 穿{pir} 爆{} 伤{})", i, u.name, ana.hit, ana.cri, ana.dmg_normal);
        },
        Skill::Ctrl(p) => {
          let u = &self.id2pw(id).unit;
          let ut = &self.p2pw(*p).unit;
          let point = u.tie_point(ut);
          println!("{} : 压制并捆绑 -> {} : {}层", i, ut.name, point);
        },
        Skill::Untie(p) => {
          let u = &self.pos_pawn(*p).unwrap().unit;
          println!("{} : 解绑 -> {}", i, u.name);
        },
        Skill::UntieSelf => {
          println!("{} : 解绑自己", i);
        },
        Skill::Escape => {
          println!("{} : 挣脱束缚", i);
        },
        Skill::CtnCtrl => {
          let u = &self.id2pw(id).unit;
          let ut = &self.id2pw(u.mastered_id().unwrap()).unit;
          let point = u.tie_point(ut);
          println!("{} : 维持压制 -> {} : {}层", i, ut.name, point);
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

  fn skill_hint(&self, id : u32) -> String {
    let mut s = String::new();
    let skls = self.get_choose_skill (self.id_pos(id));
    // 先整理出目标位置与技能的HashMap
    let mut map : HashMap<i32, Vec<Skill>> = HashMap::new();
    for skl in skls {
      if let Some(p) = skill_pos(&skl) {
        map.entry(p).or_insert(vec![]).push(skl);
      }
    }
    // 针对各目标逐一给出提示
    for (pt, skls) in map {
      let idt = self.pos_pawn(pt).unwrap().id;
      let namet = &self.id2pw(idt).unit.name;
      write!(s, "{namet}:").unwrap();
      if skls.contains(&Skill::Ctrl(pt)) {
        let u = &self.id2pw(id).unit;
        let ut = &self.id2pw(idt).unit;
        let point = u.tie_point(ut);
        s += "控";
        s += &format!("{}", point);
      }
      let u = &self.pos_pawn(self.id_pos(id)).unwrap().unit;
      let ut = &self.pos_pawn(self.id_pos(idt)).unwrap().unit;
      if skls.contains(&Skill::Punch(pt)) {
        let ana = attack_analyse(u, ut, Attack::Punch);
        if ana.pierce {
          s += "√";
        }
      }
      if skls.contains(&Skill::Kick(pt)) {
        let ana = attack_analyse(u, ut, Attack::Kick);
        if ana.pierce {
          s += "√";
        }
      }
      s += ","
    }
    if s.len() >= 1 {
      s = s[..s.len()-1].to_string();
    }
    s
  }

  fn enemy_skill_hint(&self) -> String {
    // 针对所有能动的敌人
    let mut s = String::new();
    for &id in &self.next_ids {
      let sf = self.skill_hint(id);
      let name = &self.id2pw(id).unit.name;
      write!(s, "{name}->{sf};").unwrap();
    }
    if s.len() >= 1 {
      s = s[..s.len()-1].to_string();
    }
    s
  }
}


