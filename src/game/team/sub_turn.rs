// use super::super::unit::Unit;
// use crate::wyrand::Dice;
// use std::fmt::Write;
use super::choose_skill::Skill;

use super::*;

impl Team {
  pub fn sub_turn(&mut self, o : bool, ai_1 : bool) -> bool {
    if let Some((team, ids, can_wait)) = self.get_choose_unit() {
      if o {print!("{}", self.state_ids(&ids))}
      let idq = if ai_1 && team == 1 {
        Some(self.ai_unit(&ids))
      } else {
        self.io_unit(&ids, can_wait)
      };
      if let Some(id) = idq {
        let skls = self.skill_choose(id);
        let skl = if ai_1 && team == 1 {
          self.ai_skill(id, skls)
        } else {
          self.io_skill(id, skls)
        };
        let s = self.skill_exe(id, skl);
        if o {print!("{}", s)};
      } else {
        self.wait(&ids);
      }
      true
    } else {
      false
    }
  }

  pub fn state_ids(&self, ids : &[u32]) -> String {
    let mut s = String::new();
    writeln!(s, "第{:^3}回合, 力 技 速(伤,  状态   束缚)", self.turn).unwrap();
    for p in &self.board {
      let sh = if p.unit.action() {
        if p.unit.can_select() {
          if ids.contains(&p.id){
            ">"
          }else if self.next_ids.contains(&p.id) {
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
  
  fn skill_choose(&self, id : u32) -> Vec<Skill> {
    let p = self.id_pos(id);
    self.get_choose_skill(p)
  }

  fn wait(&mut self, wait_ids : &[u32]) {
    for id in wait_ids {
      if !self.wait_ids.contains(&id) {
        self.wait_ids.push(*id);
      }
    }
  }

  pub fn skill_exe(&mut self, id : u32, skill : Skill) -> String {
    self.wait_ids.clear();
    
    let p = self.id_pos(id);
    let mut s = String::new();
  self.pos_pawn_mut(p).unwrap().unit.finish();
    let u = &self.pos_pawn(p).unwrap().unit;
    match skill {
      // 略过
      Skill::Pass => {
        writeln!(s, "\n>>>>  {} 放弃行动\n", u.name).unwrap();
        self.cancel_ctrl(p);
      },
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
        s += &self.skill_attack(p, pt, Attack::Punch);
      },
      // 踢腿
      Skill::Kick(pt) => {
        s += &self.skill_attack(p, pt, Attack::Kick);
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

  fn skill_attack(&mut self, 
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
    let back = ut.mastered();
    let ana = attack_analyse(u, ut,tp, back);
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
    ut.take_broke();
    if stun > 0 {
      ut.take_stun(stun);
      self.cancel_ctrl(pt);
    }
    self.dash_to(p, pt);
    s
  }
}

