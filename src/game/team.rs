use super::unit::Unit;
use crate::wyrand::Dice;
use std::fmt::Write;

struct Pawn {
    pub unit : Unit,
    pub team : i32,
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
    dice: Dice,
    turn : i32,
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
        Self {
            board,
            dice,
            turn : 0,
        }
    }

    pub fn main_loop(&mut self, n : i32) -> String {
        let mut s = String::new();
        for _ in 0..n {
            s += self.turn().as_str();
        }
        s
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
                if !ut.is_stun() && !ut.defeated() && ut.struggle_lv() > 0 {
                    if u.str_lv() <= ut.struggle_lv() {
                        writeln!(s, "{} 挣脱了 {} 的压制", ut.name, u.name);
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
        for pt in self.find_target(p) {
            if let Some(skill) = self.make_choice(p, pt) {
                return self.take_choice(p, skill);
            }
        }
            
        s += "无可执行行动，跳过回合\n";
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
                writeln!(s, "{} 压制并开始捆绑 {} !", u.name, ut.name).unwrap();
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
                let ut = &self.pos_pawn(pt).unwrap().unit;
                let hit_rate = 0.max(100.min(100 + u.skl_lv() * 25 - ut.spd_lv() * 25));
                let mut cri_rate = 0.max(100.min(25 + u.skl_lv() * 25 - ut.spd_lv() * 25));
                let atk = u.str();
                let def = ut.str() / 2;
                let can_def = ut.can_def() && ut.skl_lv() > u.skl_lv();
                let def_txt = if can_def {
                    cri_rate = 0;
                    "会被格挡"
                } else {
                    "无法格挡"
                };
                writeln!(s, "{} 挥拳 {} !\n命中率{}%, {}, 暴击率{}%", u.name, ut.name, hit_rate, def_txt, cri_rate).unwrap();
                let dice = self.dice.d(100);
                let u = &mut self.pos_pawn_mut(p).unwrap().unit;
                let ut = &self.pos_pawn(pt).unwrap().unit;
                let is_hit = dice <= hit_rate;
                let is_cri = dice <= cri_rate;
                let mut txt = (if is_hit {"命中"} else {"落空"}).to_string();
                let dmg = if is_hit {
                    if can_def {
                        txt += ", 被格挡\n";
                        1.max((atk - def) / 2)
                    }else{
                        txt += ", 直击";
                        if is_cri {
                            txt += ", 暴击!";
                            atk
                        }else{
                            txt += ", 未暴击";
                            1.max(atk - def)
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
                
                writeln!(s, "掷骰 {dice}, {txt}").unwrap();
                if is_hit {
                    write!(s, "造成 {} 点伤害", dmg).unwrap();
                    if stun > 0 {
                        write!(s, ", 并击晕 {} 回合", stun).unwrap();
                    }
                    s += "\n";
                }
                // 结算
                self.cancel_ctrl(p);
                let ut = &mut self.pos_pawn_mut(pt).unwrap().unit;
                ut.take_dmg(dmg);
                if stun > 0 {
                    ut.take_stun(stun);
                    self.cancel_ctrl(pt);
                }
                self.dash_to(p, pt);
            },
            // 踢腿
            Skill::Kick(pt) => {
                let ut = &self.pos_pawn(pt).unwrap().unit;
                let hit_rate = 0.max(100.min(75 + u.skl_lv() * 25 - ut.spd_lv() * 25));
                let mut cri_rate = 0.max(100.min(25 + u.skl_lv() * 25 - ut.spd_lv() * 25));
                let atk = u.str() + 5;
                let def = ut.str() / 2;
                let can_def = ut.can_def() && ut.skl_lv() >= u.skl_lv();
                let def_txt = if can_def {
                    cri_rate = 0;
                    "会被格挡"
                } else {
                    "无法格挡"
                };
                writeln!(s, "{} 踢腿 {} \n命中率{}%, {}, 暴击率{}%", u.name, ut.name, hit_rate, def_txt, cri_rate).unwrap();
                let dice = self.dice.d(100);
                let u = &self.pos_pawn(p).unwrap().unit;
                let ut = &self.pos_pawn(pt).unwrap().unit;
                let is_hit = dice <= hit_rate;
                let is_cri = dice <= cri_rate;
                let mut txt = (if is_hit {"命中"} else {"落空"}).to_string();
                let dmg = if is_hit {
                    if can_def {
                        txt += ", 被格挡\n";
                        1.max((atk - def) / 2)
                    }else{
                        txt += ", 直击";
                        if is_cri {
                            txt += ", 暴击!";
                            atk
                        }else{
                            txt += ", 未暴击";
                            1.max(atk - def)
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
                
                writeln!(s, "掷骰 {dice}, {txt}").unwrap();
                if is_hit {
                    write!(s, "造成 {} 点伤害", dmg).unwrap();
                    if stun > 0 {
                        write!(s, ", 并击晕 {} 回合", stun).unwrap();
                    }
                    s += "\n";
                }
                // 结算
                self.cancel_ctrl(p);
                let ut = &mut self.pos_pawn_mut(pt).unwrap().unit;
                ut.take_dmg(dmg);
                if stun > 0 {
                    ut.take_stun(stun);
                    self.cancel_ctrl(pt);
                }
                self.dash_to(p, pt);
            },
            // 解绑
            Skill::Untie(pt) => {
                let point = u.skl_lv();
                self.cancel_ctrl(p);
                let txt = self.pos_pawn_mut(pt).unwrap().unit.take_unties(point);
                let u = &self.pos_pawn(p).unwrap().unit;
                let ut = &self.pos_pawn(pt).unwrap().unit;
                writeln!(s, "{} 解绑 {}, 依次解绑了 {}部位", u.name, ut.name, txt).unwrap();
                self.dash_to(p, pt);
            },
            // 解绑自身
            Skill::UntieSelf => {
                let txt = self.pos_pawn_mut(p).unwrap().unit.take_unties(1);
                let u = &self.pos_pawn_mut(p).unwrap().unit;
                writeln!(s, "{} 解绑 自身, 依次解绑了 {}部位", u.name, txt).unwrap();
            },
            // 挣脱
            Skill::Escape => {
                let rate = u.str_lv() * 10;
                let dice = self.dice.d(100);
                let u = &self.pos_pawn_mut(p).unwrap().unit;
                let is_escape = dice <= rate;
                writeln!(s, "{} 尝试挣脱, 成功率 {}%", u.name, rate).unwrap();
                write!(s, "掷骰 {dice}, ").unwrap();
                if is_escape {
                    let bd = self.pos_pawn_mut(p).unwrap().unit.take_untie();
                    writeln!(s, "挣脱成功, 解除 {} 束缚", bd).unwrap();
                }else{
                    s += "挣脱失败\n";
                }
            },
            // 维持压制
            Skill::CtnCtrl => {
                writeln!(s, "{} 维持压制，继续捆绑", u.name).unwrap();
            },
        }
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
        // 维持控制
        let u = &self.pos_pawn(p).unwrap().unit;
        if let Some(it) = u.mastered_id() {
            if self.id_pawn(it).unit.defeated() == false {
                return Some(Skill::CtnCtrl)
            }
        }

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
                if u.can_punch() &&  ut.can_def() && u.skl_lv() == ut.skl_lv() {
                    return Some(Skill::Punch(pt))
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
        let mut p_f = None;
        for (i, p) in self.board.iter().enumerate() {
            let i = i as i32;
            let u = &p.unit;
            if u.action() && u.spd() > spd_f.unwrap_or(-1) {
                spd_f = Some(u.spd());
                p_f = Some(i);
            }
        }
        p_f
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