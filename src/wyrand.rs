#[allow(warnings)]

//#[repr(transparent)]
struct WyRand {
    state: u64,
}

impl WyRand {
    /// Creates a new [`WyRand`] instance with the provided seed. Be sure
    /// to obtain the seed value from a good entropy source, either from
    /// hardware, OS source, or from a suitable crate, like `getrandom`.
    //#[inline]
    pub fn new(state: u64) -> Self {
        Self { state }
    }

    /// Generates a random [`u64`] value and advances the PRNG state.
    //#[inline]
    pub fn rand(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0xa076_1d64_78bd_642f);
        let t = u128::from(self.state).wrapping_mul(u128::from(self.state ^ 0xe703_7ed1_a0b4_28db));
        (t.wrapping_shr(64) ^ t) as u64
    }
}

pub struct Dice {
    wy : WyRand,
}

impl Dice {
    pub fn new(state : u64) -> Self {
        Self {
            wy : WyRand::new(state)
        }
    }

    pub fn d(&mut self, n : i32) -> i32 {
        let rand = self.wy.rand();
        let nu = n as u64;
        let result : i32 = (rand % nu).try_into().unwrap();
        result + 1
    }
}