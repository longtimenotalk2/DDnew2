// use super::super::unit::Unit;
// use crate::wyrand::Dice;
// use std::fmt::Write;
use super::choose_skill::Skill;

use super::*;

use std::collections::HashMap;

impl Team {
  pub fn ai_unit(&self, ids : &[u32]) -> u32 {
    ids[0]
  }

  pub fn ai_skill(&self, id : u32, skls : Vec<Skill>) -> Skill {
    // 先整理出目标位置与技能的HashMap
    let mut map : HashMap<i32, Vec<Skill>> = HashMap::new();
    let mut list_self = vec!();
    for skl in skls {
      if let Some(p) = skill_pos(&skl) {
        map.entry(p).or_insert(vec![]).push(skl);
      } else {
        list_self.push(skl)
      }
    }
    // 继续压制
    if list_self.contains(&Skill::CtnCtrl) {
      return Skill::CtnCtrl
    }
    // 对目标操作，选择最近的
    let mut pn : Option<i32> = None;
    let p = self.id_pos(id);
    for pt in map.keys() {
      if let Some(pnr) = pn {
        if (p-pt).abs() < (p-pnr).abs() {
          pn = Some(*pt);
        }
      } else {
        pn = Some(*pt);
      }
    }
    // 压制＞进攻＞解绑＞解绑自身＞挣脱大于空过
    if let Some(pt) = pn {
      let skls = map.get(&pt).unwrap();
      if skls.contains(&Skill::Ctrl(pt)) {
        return Skill::Ctrl(pt);
      } else if skls.contains(&Skill::Punch(pt)) {
        return Skill::Punch(pt);
      } else if skls.contains(&Skill::Kick(pt)) {
        return Skill::Kick(pt);
      } else if skls.contains(&Skill::Untie(pt)) {
        return Skill::Untie(pt);
      }
    } else {
      if list_self.contains(&Skill::UntieSelf) {
        return Skill::UntieSelf;
      } else if list_self.contains(&Skill::Escape) {
        return Skill::Escape;
      } else if list_self.contains(&Skill::Pass) {
        return Skill::Pass;
      }
    }
    panic!()
  }
}

fn skill_pos(skl : &Skill) -> Option<i32> {
  match skl {
    Skill::Ctrl(p) => Some(*p),
    Skill::Punch(p) => Some(*p),
    Skill::Kick(p) => Some(*p),
    Skill::Untie(p) => Some(*p),
    _ => None,
  }
}