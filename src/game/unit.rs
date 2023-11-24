use std::fmt::Write;

#[derive(Clone)]
pub struct Unit {
  pub name : String,
  pub id : u32,
  str : i32,
  skl : i32,
  spd : i32,
  hurt : i32,
  stun : i32,
  broke : i32,
  ctrl : Option<u32>,
  master : Option<u32>,
  action : bool,
  // bound
  arm : bool,
  wrist : bool,
  leg : bool,
  lock : bool,
}

impl Unit {
  pub fn new(name : &str, str : i32, skl : i32, spd : i32) -> Self {
    Self {
      name : name.to_string(),
      id : 0,
      str,
      skl,
      spd,
      hurt : 0,
      stun : 0,
      broke : 0,
      ctrl : None,
      master : None,
      action : false,
      arm : false,
      wrist : false,
      leg : false,
      lock : false,
    }
  }

  pub fn change_id(&mut self, id : u32) {
  self.id = id;
  }

  pub fn reset(&mut self) {
  self.hurt = 0;
  self.stun = 0;
  self.action = false;
  self.arm = false;
  self.wrist = false;
  self.leg = false;
  self.lock = false;
  }

  pub fn state(&self) -> String {
  let mut sf = String::new();
  sf += self.name.as_str();
  write!(sf, "{}", self.id).unwrap();

  let mut s = String::new();
  if self.stun > 0 {
    write!(s, "晕{} ", self.stun).unwrap();
  } else if self.broke > 0 {
    write!(s, "破{} ", self.broke).unwrap();
  } else {
    s += "    ";
  }
  if self.ctrled() {
    write!(s, "&{}控 ", self.ctrl.unwrap()).unwrap();
  } else if self.mastered() {
    write!(s, "控&{} ", self.master.unwrap()).unwrap();
  } else {
    s += "     ";
  }
  if self.lock {
    s += "锁  ";
  } else {
    if self.arm {
      s += "臂";
      if self.leg {s += "腿";} else {s += "  ";}
    } else if self.wrist {
      s += "腕";
      if self.leg {s += "腿";} else {s += "  ";} 
    } else if self.leg {
      s += "腿  ";
    } else {
      s += "    ";
    }
  }
  let hurt = if self.hurt > 0 {
    format!("{:2}", self.hurt)
  }else{
    "  ".to_string()
  };
  format!("{} : {:2},{:2},{:2}({},{})", sf, self.str(), self.skl(), self.spd(), hurt, s)
  }

  pub fn hurt_lv(&self) -> i32 {
  self.hurt / 5
  }

  pub fn str(&self) -> i32 {
  0.max(self.str - self.hurt_lv())
  }

  pub fn skl(&self) -> i32 {
  0.max(self.skl - self.hurt_lv())
  }

  pub fn spd(&self) -> i32 {
  0.max(self.spd - self.hurt_lv())
  }

  pub fn str_lv(&self) -> i32 {
  if self.str() == 0 {
    0
  } else {
    self.str() / 5 + 1
  }
  }

  pub fn skl_lv(&self) -> i32 {
  if self.skl() == 0 {
    0
  } else {
    self.skl() / 5 + 1
  }
  }

  pub fn spd_lv(&self) -> i32 {
  if self.spd() == 0 {
    0
  } else {
    self.spd() / 5 + 1
  }
  }

  pub fn take_dmg(&mut self, dmg : i32) {
  self.hurt += dmg
  }

  pub fn take_stun(&mut self, stun : i32) {
  self.stun += stun;
  self.action = false;
  }

  pub fn stun(&self) -> i32 {
  self.stun
  }

  pub fn take_broke(&mut self) {
    self.broke += 1;
  }

  pub fn clear_broke(&mut self) {
    self.broke = 0;
  }

  pub fn broke(&self) -> i32 {
    self.broke
  }

  pub fn mastered_id(&self) -> Option<u32> {
  self.master
  }

  pub fn recover(&mut self) {
    let heal = root(self.hurt);
    self.hurt = 0.max(self.hurt - heal);
    if self.stun > 0 {
      self.stun -= 1;
    }
  }

  pub fn refresh_action(&mut self) {
    if !self.is_stun() {
      self.action = true;
    }
  }

  pub fn action(&self) -> bool {
  self.action
  }

  pub fn can_select(&self) -> bool {
    self.action() && !self.ctrled() && !self.defeated()
  }

  pub fn finish(&mut self) {
    self.action = false;
  }

  pub fn take_bound(&mut self) -> &str {
  if self.wrist == false {
    self.wrist = true;
    "[腕]"
  } else if self.leg == false {
    self.leg = true;
    "[腿]"
  } else if self.arm == false {
    self.arm = true;
    "[臂]"
  } else if self.lock == false {
    self.lock = true;
    "[锁]"
  } else {
    ""
  }
  }

  pub fn take_bounds(&mut self, n : i32) -> String {
  let mut s = String::new();
  for _ in 0..n {
    let a = self.take_bound();
    if a != "" {
    s += a;
    s += " ";
    }
  }
  s
  }

  pub fn take_untie(&mut self) -> &str {
    if self.lock {
      self.lock = false;
      "[锁]"
    } else if self.wrist && !self.arm {
      self.wrist = false;
      "[腕]"
    } else if self.leg {
      self.leg = false;
      "[腿]"
    } else if self.arm {
      self.arm = false;
      "[臂]"
    } else {
      ""
    }
  }

  pub fn take_unties(&mut self, n : i32) -> String {
    let mut s = String::new();
    for _ in 0..n {
      let a = self.take_untie();
      if a != "" {
      s += a;
      s += " ";
      }
    }
    s
  }

  pub fn take_ctrl(&mut self, ctrl : u32) {
    self.ctrl = Some(ctrl);
  }

  pub fn take_master(&mut self, master : u32) {
    self.master = Some(master);
  }

  pub fn cancel_ctrl(&mut self) {
    self.ctrl = None;
    self.master = None;
  }

  // 定性状态
  pub fn is_stun(&self) -> bool {
    self.stun > 0
  }
  
  pub fn defeated(&self) -> bool {
    self.lock
  }

  pub fn ctrled(&self) -> bool {
    self.ctrl.is_some()
  }

  pub fn mastered(&self) -> bool {
    self.master.is_some()
  }

  pub fn restrain(&self) -> bool {
    self.wrist && self.leg
  }

  pub fn block(&self) -> bool {
    if self.stun > 0 {
      false
    } else if self.ctrled() {
      false
    } else if self.restrain() {
      false
    } else {
      true
    }
  }

  pub fn have_bound(&self) -> bool {
    self.wrist || self.leg || self.arm || self.lock 
  }

  pub fn can_target(&self) -> bool {
    !self.ctrled()
  }

  pub fn can_stand(&self) -> bool {
    !self.leg && !self.is_stun()
  }

  pub fn can_ctrl(&self) -> bool {
    !self.wrist && !self.leg && !self.arm && !self.lock && self.str_lv() > 0 && self.skl_lv() > 0
  }

  pub fn can_punch(&self) -> bool {
  !self.wrist && !self.leg && !self.arm && !self.lock && self.str_lv() > 0 && self.skl_lv() > 0
  }

  pub fn can_kick(&self) -> bool {
  !self.leg && !self.lock && self.str_lv() > 0 && self.skl_lv() > 0
  }

  pub fn can_def(&self) -> bool {
    !self.is_stun() && !self.wrist && self.str() > 0 && self.skl() > 0
  }

  pub fn can_untie(&self) -> bool {
    !self.is_stun() && !self.wrist && !self.leg && !self.arm && !self.lock && self.skl() > 0
  }

  pub fn can_ctrl_w(&self, ut : &Unit) -> bool {
    let u = &self;
    if !u.can_ctrl() {
      false
    } else if ut.is_stun(){
      true
    } else if ut.restrain() {
      true
    } else if ut.struggle_lv() == 0 || u.str_lv() - ut.struggle_lv() >= 2 {
      ut.can_stand() == false || u.skl_lv() >= ut.spd_lv()
    } else {
      false
    }
  }

  pub fn can_untie_self(&self) -> bool {
  !self.is_stun() && !self.wrist
  }

  // 定量状态
  pub fn struggle_lv(&self) -> i32 {
  let mut rs = self.str_lv();
  if self.wrist {
    rs -= 1;
  }
  if self.leg {
    rs -= 1;
  }
  0.max(rs)
  }

  pub fn antibound_lv(&self) -> i32 {
    if self.is_stun() {
      0
    } else {
      self.str_lv()
    }
  }

  pub fn evd_lv(&self) -> i32 {
  let evd = if self.leg {
    self.spd / 2
  } else {
    self.spd
  };
  get_lv(evd)
  }
}

fn get_lv(i : i32) -> i32 {
  if i <= 0 {
  0
  } else {
  i / 5 + 1
  }
}

fn root(i : i32) -> i32 {
  for r in 0..20 {
    if r * r > i {
      return r - 1
    }
  }
  20
}