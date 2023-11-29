use super::unit::Unit;
use crate::wyrand::Dice;
use std::fmt::Write;

mod choose_unit;
mod choose_skill;
mod sub_turn;
mod full_turn;
mod io;
mod ai;

use crate::game::art::draw_board;
use super::file::*;
use std::fs::File;
use std::io::Write as OtherWrite;
use std::io::Read;

#[derive(Clone)]
pub struct Pawn {
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
  wait_ids : Vec<u32>,
  next_ids : Vec<u32>,
}

impl Team {

  pub fn delete_file() -> std::io::Result<()> {
    let path = &"save0.did";
    std::fs::remove_file(path)?;
    Ok(())
  }

  pub fn load_from_file() -> std::io::Result<Self> {
    let path = &"save0.did";
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(Self::load(s))
  }

  pub fn save_to_file(&self) -> std::io::Result<()> {
    let path = &"save0.did";
    let f = File::create(path);
    let mut f = f.unwrap_or(File::open(path).unwrap());
    let s = self.save();
    f.write_all(&s.as_bytes())?;
    Ok(())
  }
  
  pub fn load(s: String) -> Team {
    let mut s : Vec<&str> = s.split("\n").collect();
    let dice = Dice::load(s.remove(0).to_string());
    let turn = load_i32(s.remove(0).to_string());
    let next_team = load_u8(s.remove(0).to_string());
    let spd_now = load_option_i32(s.remove(0).to_string());
    let wait_ids = load_vec_u32(s.remove(0).to_string());
    let next_ids = load_vec_u32(s.remove(0).to_string());
    let l = load_u32(s.remove(0).to_string());
    let mut board = Vec::new();
    let c = 15;
    for _ in 0..l {
      let su = s[0..c].join("\n");
      let unit = Unit::load(su);
      for _ in 0..c {s.remove(0);}
      let team = load_u8(s.remove(0).to_string());
      let id = load_u32(s.remove(0).to_string());
      let p = Pawn {
        unit,
        team,
        id,
      };
      board.push(p);
    }
    
    Self {
      board,
      dice,
      next_team,
      wait_ids,
      next_ids,
      turn,
      spd_now,
    }
  }
  
  pub fn save(&self) -> String {
    let mut s = String::new();
    s += &self.dice.save();
    s += &save_i32(self.turn);
    s += &save_u8(self.next_team);
    s += &save_option_i32(self.spd_now);
    s += &save_vec_u32(self.wait_ids.clone());
    s += &save_vec_u32(self.next_ids.clone());
    let l = self.board.len();
    s += &save_u32(l as u32);
    for pw in &self.board {
      s += &pw.unit.save();
      s += &save_u8(pw.team);
      s += &save_u32(pw.id);
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
      wait_ids : vec!(),
      next_ids : vec!(),
    }
  }

  pub fn draw(&self, active_ids : &[u32]) {
    let lines = draw_board(&self.board, active_ids, &self.next_ids, &self.spd_now);
    for l in lines {
      println!("{}", l);
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

  fn p2pw(&self, pos : i32) -> &Pawn {
    self.pos_pawn(pos).unwrap()
  }

  fn id2pw(&self, id : u32) -> &Pawn {
    self.pos_pawn(self.id_pos(id)).unwrap()
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

fn attack_analyse(act : &Unit, tar : &Unit, tp : Attack) -> AttackData {
  let mut back_fix = if tar.mastered() {1} else {0};
  back_fix += tar.broke();
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
  let hit = 0.max(100.min(base_hit + (act.spd_lv() - tar.spd_lv()) * 25));
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