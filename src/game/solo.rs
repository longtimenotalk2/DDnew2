use super::unit::Unit;
use crate::wyrand::Dice;

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

    pub fn show(&self) {
        self.units[0].show();
        self.units[1].show();
    }
    
    // action
    pub fn punch(&mut self, act : usize) {
        let tar = 1 - act;
        println!("{act} => {tar}");
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
                print!("Crit!");
                0.max(atk)
            } else if d <= ubk {
                print!("Hit!");
                0.max(atk - def / 2)
            } else {
                print!("Block!");
                0.max(atk - def)
            };
            print!(" : {}", dmg);
            self.units[tar].take_dmg(dmg);

        } else {
            print!("Miss")
        }
        print!(" ({d}, {hit}, {ubk}, {cht})\n");
    }
}