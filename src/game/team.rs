use super::unit::Unit;
use crate::wyrand::Dice;
use std::fmt::Write;

struct Pawn {
    pub unit : Unit,
    pub team : i32,
    pub id : u32,
}

enum Skill {
    Ctrl,
    Punch,
    Kick,
    Untie,
    Unbound,
    Struggle,
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

    pub fn turn(&mut self) -> String {
        let mut s = String::new();
        writeln!(s, "--------------------------Turn {}", self.turn).unwrap();

        while let Some(pos) = self.get_next_actor() {
            self.action(pos);
        }
        s
    }

    fn action(&mut self, p : i32) -> String {
        let mut s = String::new();

        s += "跳过回合\n";
        s
    }

    fn make_choice(&self, p : i32, pt : i32) -> Option<Skill> {
        None
    }

    pub fn find_target(&self, p : i32) -> Vec<i32> {
        let mut list = vec!();
        let team = self.get_pawn(p).unwrap().team;
        let mut stop = [false, false];
        for i in 1..self.board.len() {
            for s in 0..2 {
                let i = i as i32;
                if stop[s] == false {
                    let pt = p + [-1, 1][s] * i;
                    if let Some(pw) = self.get_pawn(pt) {
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
        list.push(p);
        list
    }

    fn dash_to(&mut self, pos : usize, tar : usize) {
        if pos > tar {
            let t = self.board.remove(pos);
            self.board.insert(tar+1, t);
        } else {
            let t = self.board.remove(pos);
            self.board.insert(tar-1, t);
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

    fn get_pawn(&self, pos : i32) -> Option<&Pawn> {
        let rs : Result<usize, _> = pos.try_into() ;
        if let Ok(i) = rs {
            self.board.get(i)
        }else {
            None
        }
        
    }
}