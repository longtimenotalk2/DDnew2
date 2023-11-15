// use super::super::unit::Unit;
// use crate::wyrand::Dice;
// use std::fmt::Write;

use super::*;

#[derive(PartialEq, Eq)]
pub enum Skill {
  Ctrl(i32),
  Punch(i32),
  Kick(i32),
  Untie(i32),
  UntieSelf,
  Escape,
  CtnCtrl,
  Pass,
}

impl Team {
  pub fn get_choose_skill(&self, p : i32) -> Vec<Skill> {
    let mut skls = Vec::new();
    // 略过
    skls.push(Skill::Pass);
    let pw = self.pos_pawn(p).unwrap();
    let u = &self.pos_pawn(p).unwrap().unit;
    // 维持控制
    if u.mastered() {
      skls.push(Skill::CtnCtrl);
    }
    // 挣脱束缚
    if u.have_bound(){
      skls.push(Skill::Escape);
    }
    // 自我解绑
    if u.have_bound() && u.can_untie_self(){
      skls.push(Skill::Escape);
    }
    // 对他人
    for pt in self.find_target_w_pos(p) {
      let pwt = self.pos_pawn(pt).unwrap();
      let ut = &pwt.unit;
      if pw.team == pwt.team {
        // 对友
        // 解绑
        if ut.have_bound() && u.can_untie() {
          skls.push(Skill::Untie(pt));
        }
      }else {
        // 对敌
        // 控制
        if u.can_ctrl_w(ut) {
          skls.push(Skill::Ctrl(pt));
        }
        // 攻击
        if u.can_punch() {
          skls.push(Skill::Punch(pt));
        }
        if u.can_kick() {
          skls.push(Skill::Kick(pt));
        }
      }
    }
    skls
  }

  fn find_target_w_pos(&self, p : i32) -> Vec<i32> {
    let mut list = vec!();
    let team = self.pos_pawn(p).unwrap().team;
    let mut stop = [false, false];
    for i in 1..self.board.len() {
      for s in 0..2 {
        let i = i as i32;
        if stop[s] == false {
          let pt = p + [-1, 1][s] * i;
          if let Some(pw) = self.pos_pawn(pt) {
            if pw.unit.can_target() {
              list.push(pt);
            }
            if pw.team != team && pw.unit.block() {
              stop[s] = true;
            }
          }
        }
      }
    }
    list
  }
}