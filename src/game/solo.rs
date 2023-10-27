use super::unit::Unit;
use crate::wyrand::Dice;
use std::fmt::Write;

pub struct Solo {
    units : Vec<Unit>,
    turn : i32,
    dice : Dice,
}

impl Solo {
    pub fn new(unit0 : Unit, unit1 : Unit, dice : Dice) -> Self {
        Self {
            units : vec!(unit0, unit1),
            turn : 0,
            dice,
        }
    }

    pub fn state(&self) -> String {
        self.units[0].state() + self.units[1].state().as_str()
    }

    pub fn main_loop_times(&mut self, times : usize) -> Vec<i32> {
        let mut list = vec!();
        for _ in 0..times {
            self.reset();
            list.push(self.main_loop().1);
        }
        list
    }

    pub fn reset(&mut self) {
        self.turn = 0;
        self.units[0].reset();
        self.units[1].reset();
    }

    pub fn main_loop(&mut self) -> (String, i32) {
        let mut s = String::new();
        s += self.state().as_str();
        loop {
            self.turn += 1;
            s += self.turn().as_str();
            if self.turn > 10 {
                return (s, 2)
            } else if self.units[0].str() == 0 {
                return (s, 1);
            } else if self.units[1].str() == 0 {
                return (s, 0);
            } 
        }
    }

    pub fn turn(&mut self) -> String {
        let mut s = String::new();
        writeln!(s, "Turn = {}", self.turn).unwrap();
        let first : usize = if self.units[0].spd() < self.units[1].spd() { 
            1 
        } else if self.units[0].spd() > self.units[1].spd() { 
            0 
        } else {
            (self.dice.d(2) - 1).try_into().unwrap()
        };

        s += self.punch(first).as_str();
        s += self.punch(1 - first).as_str();

        s += self.state().as_str();
        s
    }
    
    // action
    pub fn punch(&mut self, act : usize) -> String {
        let mut s = String::new();
        let tar = 1 - act;
        writeln!(s, "{act} => {tar}").unwrap();
        let acc = 100 + self.units[act].skl() * 10;
        let evd = self.units[tar].spd() * 10;
        let hit = acc - evd;
        let atk = 5 + self.units[act].str();
        let def = self.units[tar].str();
        let cri = 10 * self.units[act].skl();
        let cvd = 10 * self.units[tar].skl();
        let cht = cri - cvd;
        let ubk = 50 + cri - cvd;
        let d = self.dice.d(100);
        if d <= hit {
            let dmg = if d <= cht {
                write!(s, "Crit!").unwrap();
                0.max(atk)
            } else if d <= ubk {
                write!(s, "Hit!").unwrap();
                0.max(atk - def / 2)
            } else {
                write!(s, "Block!").unwrap();
                0.max(atk - def)
            };
            write!(s, " : {}", dmg).unwrap();
            self.units[tar].take_dmg(dmg);

        } else {
            write!(s, "Miss").unwrap()
        }
        write!(s, " ({d}, {hit}, {ubk}, {cht})\n").unwrap();
        s
    }
}

/*
10月17日，规则1
攻击=力+5
防御=力
命中=100%+（技-速）×10%
破守=50%+（技-技）×10%（防御减半）
暴击=（技-技）×10%（防御归零）
先力量归0者判负
以13 13 13为基础，分别比拼三维其中一项+2，都是50%胜率。

*/