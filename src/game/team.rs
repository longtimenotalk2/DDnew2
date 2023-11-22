use super::unit::Unit;
use crate::wyrand::Dice;
use std::fmt::Write;

mod choose_unit;
mod choose_skill;
mod sub_turn;
mod full_turn;
mod io;
mod ai;

#[derive(Clone)]
struct Pawn {
  pub unit : Unit,
  pub team : u8,
  pub id : u32,
}

pub struct Team {
  board: Vec<Pawn>,
  dice: Dice,
  turn : i32,
  pub next_team : u8,
  spd_now : Option<i32>,
  wait_ids : Vec<u32>
}

impl Team {
  pub fn state(&self) -> String {
    let mut s = String::new();
    writeln!(s, "第{:^3}回合, 力 技 速(伤,  状态   束缚情况)", self.turn).unwrap();
    for p in &self.board {
      let sh = if p.unit.action() {
        if p.unit.can_select() {
          if self.wait_ids.contains(&p.id) {
            "w"
          } else if p.unit.spd() >= self.spd_now.unwrap_or(-1) {
            "|"
          } else {
            "·"
          }
        } else {
          "x"
        }
          
      } else {
        " "
      };
      writeln!(s, "{sh}{}", p.unit.state()).unwrap();
    }
    s
  }
  
  pub fn new(a : Vec<Unit>, b : Vec<Unit>, dice : Dice) -> Team {
    let mut board = vec!();
    let mut id : u32 = 1;
    for mut u in a {
      u.change_id(id);
      board.push(Pawn {unit : u, team : 0, id});
      id += 1;
    }
    for mut u in b {
      u.change_id(id);
      board.push(Pawn {unit : u, team : 1, id});
      id += 1;
    }

    Self {
      board,
      dice,
      turn : 0,
      next_team : 0,
      spd_now : None,
      wait_ids : vec!()
    }
  }



  fn cancel_ctrl(&mut self, p : i32) {
  let u = &self.pos_pawn(p).unwrap().unit;
  if let Some(it) = u.mastered_id()
  {
    self.pos_pawn_mut(p).unwrap().unit.cancel_ctrl();
    self.pos_pawn_mut(self.id_pos(it)).unwrap().unit.cancel_ctrl();
  }
  }

  pub fn find_target(&self, p : i32) -> Vec<i32> {
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

  fn dash_to(&mut self, pos : i32, tar : i32) {
  let ctrl = self.pos_pawn(tar).unwrap().unit.mastered_id();
  let pos : usize = pos.try_into().unwrap();
  let tar : usize = tar.try_into().unwrap();
  if pos > tar {
    let t = self.board.remove(pos);
    self.board.insert(tar+1, t);
  } else {
    let t = self.board.remove(pos);
    self.board.insert(tar-1, t);
  }
  // 被控制角色拉回
  if let Some(id) = ctrl {
    let pos : usize = self.id_pos(id).try_into().unwrap();
    if pos > tar {
    let t = self.board.remove(pos);
    self.board.insert(tar+1, t);
    } else {
    let t = self.board.remove(pos);
    self.board.insert(tar-1, t);
    }
  }
  }

  fn is_end(&self) -> Option<u8> {
  let mut df_0 = true;
  let mut df_1 = true;
  for pw in &self.board {
    if !pw.unit.defeated() {
    if pw.team == 0 {
      df_0 = false;
    } else {
      df_1 = false;
    }
    }
  }
  if !df_0 && !df_1 {
    None
  } else if df_0 {
    Some(0)
  } else if df_1 {
    Some(1)
  } else {
    panic!();
  }
  }

  fn pos_pawn(&self, pos : i32) -> Option<&Pawn> {
  let rs : Result<usize, _> = pos.try_into() ;
  if let Ok(i) = rs {
    self.board.get(i)
  }else {
    None
  }
  }

  fn pos_pawn_mut(&mut self, pos : i32) -> Option<&mut Pawn> {
  let rs : Result<usize, _> = pos.try_into() ;
  if let Ok(i) = rs {
    self.board.get_mut(i)
  }else {
    None
  }
  }

  fn id_pos(&self, id : u32) -> i32 {
  for (i, p) in self.board.iter().enumerate() {
    if p.id == id {
    return i as i32;
    }
  }
  panic!("id not found");
  }
}

fn get_lv(i : i32) -> i32 {
  if i <= 0 {
  0
  } else {
  i / 5 + 1
  }
}

enum Attack {
  Punch,
  Kick,
}

struct AttackData {
  pub hit : i32,
  pub pierce : bool,
  pub cri : i32,
  pub dmg_normal : i32,
  pub dmg_cri : i32,
}

fn attack_analyse(act : &Unit, tar : &Unit, tp : Attack, back : bool) -> AttackData {
  let back_fix = if back {1} else {0};
  let base_hit = match &tp {
    Attack::Punch => 100 ,
    Attack::Kick => 75,
  } + 25 * back_fix;
  let base_cri = match &tp {
    Attack::Punch => 20,
    Attack::Kick => 20,
  } + 20 * back_fix;
  let base_atk = match &tp {
    Attack::Punch => 0,
    Attack::Kick => 5,
  };
  let atk = base_atk + act.str();
  let def = tar.str() / 2;
  let mut pierce = match &tp {
    Attack::Punch => act.skl_lv() - tar.skl_lv() + back_fix >= 1,
    Attack::Kick => act.skl_lv() - tar.skl_lv() + back_fix >= 2,
  };
  if !tar.can_def() {
    pierce = true;
  }
  let hit = 0.max(100.min(base_hit + (act.skl_lv() - tar.spd_lv()) * 25));
  let mut cri = 0.max(100.min(base_cri + (act.skl_lv() - tar.spd_lv()) * 10));
  if !pierce {
    cri = 0;
  }
  let mut dmg_normal = 1.max(atk - def);
  if !pierce {
    dmg_normal = 1.max(dmg_normal / 2);
  }
  let dmg_cri = atk;
  
  AttackData {hit, pierce, cri, dmg_normal, dmg_cri}
}

fn exp_dmg(ana : &AttackData) -> i32 {
  let cri_part = ana.cri * ana.dmg_cri;
  let normal_part = ana.dmg_normal * (ana.hit - ana.cri);
  cri_part + normal_part
}