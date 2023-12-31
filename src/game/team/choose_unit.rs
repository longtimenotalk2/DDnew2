// use super::super::unit::Unit;
// use crate::wyrand::Dice;
// use std::fmt::Write;

use super::*;

impl Team {
  // 返回接下来的行动方，行动角色列表，和是否允许等待
  pub fn get_choose_unit(&mut self) -> Option<(u8, Vec<u32>, bool)> {
    // 主要
    let next = self.decide_next(self.spd_now, &self.wait_ids, self.next_team);
    if let Some((mut spd, team, mut ids)) = next {
      // 如果此时选择等待，再次行动的依然是我方，则自动选择等待
      let mut wait_ids = self.wait_ids.clone();
      loop {
        for id in &ids {
          if !wait_ids.contains(id) {
            wait_ids.push(*id);
          }
        }
        if let Some((spd2, team2, ids2)) = self.decide_next(Some(spd), &wait_ids, 1 - team) {
          if team2 == team {
            spd = spd2;
            ids = ids2;
          }else{
            break;
          }
        }else {
          break;
        }
      }

      // 更新team的当前速度
      self.spd_now = Some(spd);
      self.next_team = 1 - team;
      let can_wait = self.can_wait(&ids, &self.wait_ids, team);
      // 在返回前，计算一下下一波要动的角色
      let mut wait_ids = self.wait_ids.clone();
      for id in &ids {
        if !wait_ids.contains(id) {
          wait_ids.push(*id);
        }
      }

      let next2 = self.decide_next(Some(spd), &wait_ids, 1 - team);
      self.next_ids.clear();
      if let Some((_, _, ids)) = next2 {
        self.next_ids = ids;
      }
      Some((team, ids, can_wait))
    } else {
      None
    }
  }
  
  fn can_wait(&self, ids : &[u32], wait_ids : &[u32], team : u8) -> bool {
    // 如果全员可动，获取当前队伍可动角色的最低速度，以及对方可动角色中，速度最低的以及等待状态
    let mut spd_act = None;
    let mut spd_tar = None;
    let mut wait_tar = None;
    for pw in &self.board {
      if pw.unit.can_select() && pw.team == team {
        if ids.contains(&pw.id) {
          spd_act = spd_act.or(Some(pw.unit.spd())).and_then(|spd| Some(spd.min(pw.unit.spd())));
        }else{
          return true
        }
      } else if pw.unit.can_select() && pw.team != team {
        if pw.unit.spd() < spd_tar.unwrap_or(999) {
          spd_tar = Some(pw.unit.spd());
          wait_tar = Some(wait_ids.contains(&pw.id));
        }
      }
    }

    if let Some(spd_act) = spd_act {
      if let Some(spd_tar) = spd_tar {
        if spd_act < spd_tar {
          false
        } else if spd_act == spd_tar {
          !wait_tar.unwrap_or(false)
        } else {
          true
        }
      } else {
        false
      }
    } else {
      false
    }
  }
  
  fn decide_next(&self, spd_now : Option<i32>, wait_ids : &[u32], next_team : u8) -> Option<(i32, u8, Vec<u32>)> {
    // 先从所有可选且不等待的角色中，获取最大速度
    let mut spd = None;
    for pw in &self.board {
      if pw.unit.can_select() && !wait_ids.contains(&pw.id) {
        if pw.unit.spd() > spd.unwrap_or(-1) {
          spd = Some(pw.unit.spd());
        }
      }
    }
    // 如果不存在，则直接返回空
    // 如果存在，则和当前速度做比较取最小值
    let spd = match spd {
      None => {return None;},
      Some(spd) => {
        match spd_now {
          None => Some(spd),
          Some(s) => Some(s.min(spd)),
        }
      },
    };
    
    if let Some(spd) = spd {
      // 存在当前速度，则寻找所有大于等于该速度的非等待角色
      let mut ids = [vec!(), vec!()];
      let next_team : usize = next_team as usize;
      for pw in &self.board {
        if pw.unit.can_select() && !wait_ids.contains(&pw.id) && pw.unit.spd() >= spd {
          let team = pw.team as usize;
          ids[team].push(pw.id);
        }
      }
      // 优先下一方，考虑所有超速角色
      if ids[next_team].len() > 0 {
        let mut l = vec!();
        for pw in &self.board {
          if pw.unit.can_select() &&  pw.unit.spd() >= spd && pw.team == next_team as u8 {
            l.push(pw.id);
          }
        }
        return Some((spd, next_team as u8, l))
      } else if ids[1 - next_team].len() > 0 {
        let mut l = vec!();
        for pw in &self.board {
          if pw.unit.can_select() &&  pw.unit.spd() >= spd && pw.team != next_team as u8 {
            l.push(pw.id);
          }
        }
        return Some((spd, 1 - next_team as u8, l))
      }
    }
    None
  }
}
