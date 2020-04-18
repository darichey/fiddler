use std::ops::{Index, IndexMut};

#[derive(Clone, Copy)]
pub enum Register {
    ZERO,
    AT,
    V0,
    V1,
    A0,
    A1,
    A2,
    A3,
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9,
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    K0,
    K1,
    GP,
    SP,
    FP,
    RA,
}

pub struct Registers {
    underlying: [i32; 32],
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            underlying: [0; 32],
        }
    }
}

impl Index<Register> for Registers {
    type Output = i32;

    fn index(&self, index: Register) -> &Self::Output {
        &self.underlying[index as usize]
    }
}

impl IndexMut<Register> for Registers {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        &mut self.underlying[index as usize]
    }
}
