use std::fmt::Write;

pub struct Unit {
    name : String,
    id : u32,
    str : i32,
    skl : i32,
    spd : i32,
    hurt : i32,
    stun : i32,
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
        if self.action {
            sf += "|";
        }else{
            sf += " ";
        }
        sf += self.name.as_str();
        write!(sf, "{}", self.id).unwrap();

        let mut s = String::new();
        if self.stun > 0 {
            write!(s, "晕{} ", self.stun).unwrap();
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
        if self.arm {
            s += "臂";
        }else{
            s+= "  ";
        }
        if self.wrist {
            s += "腕";
        }else{
            s+= "  ";
        }
        if self.leg {
            s += "腿";
        }else{
            s+= "  ";
        }
        if self.lock {
            s += "锁";
        }else{
            s+= "  ";
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
        self.str() / 5
    }

    pub fn skl_lv(&self) -> i32 {
        self.skl() / 5
    }

    pub fn spd_lv(&self) -> i32 {
        self.spd() / 5
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

    pub fn recover(&mut self) {
        self.hurt = 0.max(self.hurt - 2);
        if self.stun > 0 {
            self.stun -= 1;
        }
        if self.stun == 0 {
            self.action = true;
        }
    }

    pub fn action(&self) -> bool {
        self.action
    }

    pub fn finish(&mut self) {
        self.action = false;
    }

    pub fn take_bound(&mut self) -> bool {
        if self.wrist == false {
            self.wrist = true;
            true
        } else if self.leg == false {
            self.leg = true;
            true
        } else if self.arm == false {
            self.arm = true;
            true
        } else if self.lock == false {
            self.lock = true;
            true
        } else {
            false
        }
    }

    pub fn take_ctrl(&mut self, ctrl : u32) {
        self.ctrl = Some(ctrl);
    }

    // 定性状态
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

    pub fn can_target(&self) -> bool {
        !self.ctrled()
    }
}