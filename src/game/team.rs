use super::unit::Unit;
use crate::wyrand::Dice;
use std::fmt::Write;

#[derive(Clone)]
struct Pawn {
  pub unit : Unit,
  pub team : u8,
  pub id : u32,
}

enum Skill {
  Ctrl(i32),
  Punch(i32),
  Kick(i32),
  Untie(i32),
  UntieSelf,
  Escape,
  CtnCtrl,
}

pub struct Team {
  board: Vec<Pawn>, 
  board_init : Vec<Pawn>,
  dice: Dice,
  turn : i32,
  next_team : u8,
}

impl Team {
  pub fn state(&self) -> String {
  let mut s = String::new();
  writeln!(s, "第{:^3}回合, 力 技 速(伤,  状态   束缚情况)", self.turn).unwrap();
  for p in &self.board {
    writeln!(s, "{}", p.unit.state()).unwrap();
  }
  s
  }
  
  pub fn new(a : Vec<Unit>, b : Vec<Unit>, dice : Dice) -> Team {
  let mut board = vec!();
  let mut id : u32 = 0;
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
  let board_init = board.clone();
  Self {
    board,
    board_init,
    dice,
    turn : 0,
    next_team : 0,
  }
  }

  pub fn win_rate(&mut self, n : i32, iter : i32) {
  let mut win = 0;
  let mut lose = 0;
  for _ in 0..iter {
    self.reset();
    if let Some(df) = self.main_loop(n).0 {
    if df == 0 {
      lose += 1;
    }else{
      win += 1;
    }
    }
  }
  println!("总计 {iter} 局，胜 {win} 局，负 {lose} 局");
  }

  fn reset(&mut self) {
  self.board = self.board_init.clone();
  self.turn = 0;
  self.next_team = 0;
  }

  pub fn main_loop(&mut self, n : i32) -> (Option<u8>, String) {
  // 随机先手方
  if self.dice.d(2) == 2 {
    self.next_team = 1;
  }else{
    self.next_team = 0;
  }
  let mut s = String::new();
  for _ in 0..n {
    s += self.turn().as_str();
    if let Some(df) = self.is_end() {
    if df == 0 {
      s += "我方全员被锁死，我方战败\n"
    } else {
      s += "敌方全员被锁死, 我方胜利\n"
    }
    return (Some(df), s)
    }
  }
  (None, s)
  } 

  pub fn turn(&mut self) -> String {
  let mut s = String::new();
  self.turn += 1;
  writeln!(s, "=============第 {} 回合开始==============", {self.turn}).unwrap();

  // 恢复
  for pw in &mut self.board {
    pw.unit.recover();
  }

  // 尝试挣扎
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

  s += &self.state();
  
  // 行动阶段
  while let Some(pos) = self.get_next_actor() {
    let team = self.pos_pawn(pos).unwrap().team;
    self.next_team = 1 - team;
    s += self.action(pos).as_str();
    s += &self.state();
  }

  // 捆绑阶段
  for i in 0..self.board.len() {
    let i : usize = i.try_into().unwrap();
    let u = &self.board[i].unit;
    let p = self.id_pos(u.id);
    if let Some(it) = u.mastered_id() {
    let pt = self.id_pos(it);
    let ut = &self.pos_pawn(pt).unwrap().unit;
    let mut point = u.skl_lv();
    if ut.antibound_lv() > 0 {
      point -= 0.max(ut.antibound_lv() + 2 - u.str_lv());
    }
    point = 1.max(point);
    writeln!(s, "{} 对 {} 进行了捆绑", u.name, ut.name).unwrap();
    let txt = self.pos_pawn_mut(pt).unwrap().unit.take_bounds(point);
    writeln!(s, "依次捆绑了 {}部位", txt).unwrap();
    }
  }
  s
  }

  fn action(&mut self, p : i32) -> String {
  let mut s = String::new();
  // 维持控制
  let u = &self.pos_pawn(p).unwrap().unit;
  if let Some(it) = u.mastered_id() {
    if self.id_pawn(it).unit.defeated() == false {
    return self.take_choice(p, Skill::CtnCtrl);
    }
  }
  for pt in self.find_target(p) {
    if let Some(skill) = self.make_choice(p, pt) {
    return self.take_choice(p, skill);
    }
  }
    
  s += "无可执行行动，跳过回合\n";
  self.pos_pawn_mut(p).unwrap().unit.finish();
  s
  }

  fn take_choice(&mut self, p : i32, skill : Skill) -> String {
  let mut s = String::new();
  self.pos_pawn_mut(p).unwrap().unit.finish();
  let u = &self.pos_pawn(p).unwrap().unit;
  match skill {
    // 压制
    Skill::Ctrl(pt) => {
    let ut = &self.pos_pawn(pt).unwrap().unit;
    writeln!(s, "\n>>>>  {} 压制并开始捆绑 {} !\n", u.name, ut.name).unwrap();
    let id_ctrl = ut.id;
    let id_master = u.id;
    // 取消自身和对手的压制
    self.cancel_ctrl(p);
    self.cancel_ctrl(pt);
    // 压制对手
    self.pos_pawn_mut(p).unwrap().unit.take_master(id_ctrl);
    self.pos_pawn_mut(pt).unwrap().unit.take_ctrl(id_master);
    // 位移
    self.dash_to(p, pt);
    },
    // 挥拳
    Skill::Punch(pt) => {
      s += &self.attack(p, pt, Attack::Punch);
    },
    // 踢腿
    Skill::Kick(pt) => {
      s += &self.attack(p, pt, Attack::Kick);
    },
    // 解绑
    Skill::Untie(pt) => {
    let point = u.skl_lv();
    self.cancel_ctrl(p);
    let txt = self.pos_pawn_mut(pt).unwrap().unit.take_unties(point);
    let u = &self.pos_pawn(p).unwrap().unit;
    let ut = &self.pos_pawn(pt).unwrap().unit;
    writeln!(s, "\n{} 解绑 {}, 依次解绑了 {}部位\n", u.name, ut.name, txt).unwrap();
    self.dash_to(p, pt);
    },
    // 解绑自身
    Skill::UntieSelf => {
    let txt = self.pos_pawn_mut(p).unwrap().unit.take_unties(1);
    let u = &self.pos_pawn_mut(p).unwrap().unit;
    writeln!(s, "\n{} 解绑 自身, 依次解绑了 {}部位\n", u.name, txt).unwrap();
    },
    // 挣脱
    Skill::Escape => {
    let rate = u.str_lv() * 10;
    let dice = self.dice.d(100);
    let u = &self.pos_pawn_mut(p).unwrap().unit;
    let is_escape = dice <= rate;
    writeln!(s, "\n{} 尝试挣脱, 成功率 {}%", u.name, rate).unwrap();
    write!(s, "掷骰 {dice}, ").unwrap();
    if is_escape {
      let bd = self.pos_pawn_mut(p).unwrap().unit.take_untie();
      writeln!(s, "挣脱成功, 解除 {} 束缚\n", bd).unwrap();
    }else{
      s += "挣脱失败\n\n";
    }
    },
    // 维持压制
    Skill::CtnCtrl => {
    writeln!(s, "\n>>>>  {} 维持压制，继续捆绑\n", u.name).unwrap();
    },
  }
  s
  }

  fn attack(&mut self, 
    p : i32, 
    pt : i32,
    tp : Attack,
  ) -> String {
    let mut s = String::new();
    let name = match &tp {
      Attack::Punch => "挥拳",
      Attack::Kick => "踢腿",
    };
    let u = &self.pos_pawn(p).unwrap().unit;
    let ut = &self.pos_pawn(pt).unwrap().unit;
    let ana = attack_analyse(u, ut,tp);
    let hit_rate = ana.hit;
    let cri_rate = ana.cri;
    let can_pierce = ana.pierce;
    let can_def = !can_pierce;
    let def_txt = if can_def {
      "会被格挡"
    } else {
      "无法格挡"
    };
    let mut vtxt = String::new();
    let mut dtxt = String::new();
    write!(vtxt, ">>>>  {} {name} {} : ", u.name, ut.name).unwrap();
    writeln!(dtxt, "命中率{}%, {}, 暴击率{}%", hit_rate, def_txt, cri_rate).unwrap();
    let dice = self.dice.d(100);
    let u = &mut self.pos_pawn_mut(p).unwrap().unit;
    let ut = &self.pos_pawn(pt).unwrap().unit;
    let is_hit = dice <= hit_rate;
    let is_cri = dice <= cri_rate;
    let mut txt = (if is_hit {"命中"} else {"落空"}).to_string();
    let dmg = if is_hit {
      if can_def {
      txt += ", 被格挡";
      vtxt += "(挡)";
      ana.dmg_normal
      }else{
      txt += ", 直击";
      if is_cri {
        txt += ", 暴击!";
        ana.dmg_cri
      }else{
        txt += ", 未暴击";
        ana.dmg_normal
      }
      }
    } else {
      0
    };

    let stun = if is_cri {
      let dmg_lv = get_lv(dmg);
      0.max(dmg_lv + 1 -  ut.str_lv())
      
    }else{
      0
    };
    
    writeln!(dtxt, "掷骰 {dice}, {txt}").unwrap();
    if is_hit {
      write!(dtxt, "造成 {} 点伤害", dmg).unwrap();
      write!(vtxt, " {}", dmg).unwrap();
      if is_cri {
        vtxt += "!";
      }
      if stun > 0 {
      write!(dtxt, ", 并击晕 {} 回合", stun).unwrap();
      write!(vtxt, " 晕 {} !", stun).unwrap();
      }
    } else {
      vtxt += "miss"
    }
    s += "\n";
    s += &vtxt;
    s += "\n\n";
    s += &dtxt;
    s += "\n";
    // 结算
    self.cancel_ctrl(p);
    let ut = &mut self.pos_pawn_mut(pt).unwrap().unit;
    ut.take_dmg(dmg);
    if stun > 0 {
      ut.take_stun(stun);
      self.cancel_ctrl(pt);
    }
    self.dash_to(p, pt);
    s
  }

  fn cancel_ctrl(&mut self, p : i32) {
  let u = &self.pos_pawn(p).unwrap().unit;
  if let Some(it) = u.mastered_id()
  {
    self.pos_pawn_mut(p).unwrap().unit.cancel_ctrl();
    self.pos_pawn_mut(self.id_pos(it)).unwrap().unit.cancel_ctrl();
  }
  }

  fn make_choice(&self, p : i32, pt : i32) -> Option<Skill> {

  // 对它人
  let pw = self.pos_pawn(p).unwrap();
  let pwt = self.pos_pawn(pt).unwrap();
  let u = &pw.unit;
  let ut = &pwt.unit;
  if pw.team != pwt.team {
    // 对敌
    
    // 控制
    let ci = if !u.can_ctrl() {
    false
    }else if ut.defeated() {
    false
    } else if ut.is_stun(){
    true
    } else if ut.restrain() {
    true
    } else if ut.struggle_lv() == 0 || u.str_lv() - ut.struggle_lv() >= 2 {
    if ut.can_stand() == false || u.skl_lv() >= ut.spd_lv(){
      true
    } else {
      false
    }
    } else {
    false
    };
    if ci {
    return Some(Skill::Ctrl(pt))
    }
    // 攻击
    if !ut.defeated() && !ut.is_stun() {
    // if u.can_punch() &&  ((ut.can_def() && u.skl_lv() == ut.skl_lv() + 1) || u.skl_lv() < ut.evd_lv()) {
    //   return Some(Skill::Punch(pt))
    // } else if u.can_kick() {
    //   return Some(Skill::Kick(pt))
    // } 
      if u.can_punch() {
        if u.can_kick() {
          let exp1 = exp_dmg(&attack_analyse(u, ut, Attack::Punch));
          let exp2 = exp_dmg(&attack_analyse(u, ut, Attack::Kick));
          if exp1 >= exp2 {
            return Some(Skill::Punch(pt))
          }else {
            return Some(Skill::Kick(pt))
          } 
        }else {
          return Some(Skill::Punch(pt))
        }
      } else if u.can_kick() {
        return Some(Skill::Kick(pt))
      }
    }
  } else {
    // 对友
    if ut.have_bound() && u.can_untie() {
    return Some(Skill::Untie(pt))
    }
  }
  // 对自身
  // 解绑
  if u.have_bound() {
    if u.can_untie_self() {
    return Some(Skill::UntieSelf)
    } else {
    return Some(Skill::Escape)
    }
  }
  None
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


  fn get_next_actor(&self) -> Option<i32> {
    let mut spd_f = None;
    let mut p_list = vec!();
    // 先找所有最高速的
    for (i, p) in self.board.iter().enumerate() {
      let i = i as i32;
      let u = &p.unit;
      if u.action() {
        if u.spd() > spd_f.unwrap_or(-1) {
          spd_f = Some(u.spd());
          p_list.clear();
          p_list.push(i);
        }  else {
          if u.spd() == spd_f.unwrap_or(-1) {
            p_list.push(i);
          }
        }
      }
    }
    // 优先选符合该队伍的
    for p in &p_list {
      if self.pos_pawn(*p).unwrap().team == self.next_team {
        return Some(*p)
      }
    }
    // 如果没有符合,则选择第一个
    if p_list.len() > 0 {
      return Some(p_list[0])
    }
    None
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

  fn id_pawn(&self, id : u32) -> &Pawn {
  for p in &self.board {
    if p.id == id {
    return &p;
    }
  }
  panic!("id not found");
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

fn attack_analyse(act : &Unit, tar : &Unit, tp : Attack) -> AttackData {
  let base_hit = match &tp {
    Attack::Punch => 100,
    Attack::Kick => 75,
  };
  let base_cri = match &tp {
    Attack::Punch => 20,
    Attack::Kick => 20,
  };
  let base_atk = match &tp {
    Attack::Punch => 0,
    Attack::Kick => 5,
  };
  let atk = base_atk + act.str();
  let def = tar.str() / 2;
  let mut pierce = match &tp {
    Attack::Punch => act.skl_lv() - tar.skl_lv() >= 1,
    Attack::Kick => act.skl_lv() - tar.skl_lv() >= 2,
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