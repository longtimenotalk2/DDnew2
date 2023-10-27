pub struct Unit {
    str : i32,
    skl : i32,
    spd : i32,
    hurt : i32,
    bound : bool,
    action : bool,
}

impl Unit {
    pub fn new(str : i32, skl : i32, spd : i32) -> Self {
        Self {
            str,
            skl,
            spd,
            hurt : 0,
            bound : false,
            action : true,
        }
    }

    pub fn reset(&mut self) {
        self.hurt = 0;
    }

    pub fn state(&self) -> String {
        format!("{}, {}, {} ({})", self.str(), self.skl(), self.spd(), self.hurt)
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
        self.str / 5
    }

    pub fn skl_lv(&self) -> i32 {
        self.skl / 5
    }

    pub fn spd_lv(&self) -> i32 {
        self.spd / 5
    }

    pub fn take_dmg(&mut self, dmg : i32) {
        self.hurt += dmg
    }
}