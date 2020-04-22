#[derive(Clone, Copy, PartialEq)]
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

    pub fn get(&self, register: Register) -> i32 {
        self.underlying[register as usize]
    }

    pub fn set(&mut self, register: Register, x: i32) {
        if register != Register::ZERO {
            self.underlying[register as usize] = x;
        }
    }
}

#[macro_export]
macro_rules! default_registers {
    ( $($key:expr => $value:expr ),+ ) => {
        {
            let mut registers = Registers::new();
            $(
                registers.set($key, $value);
            )+
            registers
        }
    };
}
