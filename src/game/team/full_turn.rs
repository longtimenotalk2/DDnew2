// use super::super::unit::Unit;
// use crate::wyrand::Dice;
// use std::fmt::Write;
// use super::choose_skill::Skill;

use super::*;

impl Team {
  pub fn full_turn(&mut self, o : bool) {
    self.pre();
    loop {
      if self.sub_turn(o) == false {
        return
      }
    }
  }

  fn pre(&mut self) {
    for pw in &mut self.board {
      pw.unit.refresh_action();
    }
  }
}