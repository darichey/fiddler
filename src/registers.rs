pub struct Register {
  pub ord: u8,
  pub name: &'static str
}

pub static ZERO: Register = Register {
  ord: 0,
  name: "$zero"
};
pub static AT: Register = Register {
  ord: 1,
  name: "$at"
};
pub static V0: Register = Register {
  ord: 2,
  name: "$v0"
};
pub static V1: Register = Register {
  ord: 3,
  name: "$v1"
};
pub static A0: Register = Register {
  ord: 4,
  name: "$a0"
};
pub static A1: Register = Register {
  ord: 5,
  name: "$a1"
};
pub static A2: Register = Register {
  ord: 6,
  name: "$a2"
};
pub static A3: Register = Register {
  ord: 7,
  name: "$a3"
};
pub static T0: Register = Register {
  ord: 8,
  name: "$t0"
};
pub static T1: Register = Register {
  ord: 9,
  name: "$t1"
};
pub static T2: Register = Register {
  ord: 10,
  name: "$t2"
};
pub static T3: Register = Register {
  ord: 11,
  name: "$t3"
};
pub static T4: Register = Register {
  ord: 12,
  name: "$t4"
};
pub static T5: Register = Register {
  ord: 13,
  name: "$t5"
};
pub static T6: Register = Register {
  ord: 14,
  name: "$t6"
};
pub static T7: Register = Register {
  ord: 15,
  name: "$t7"
};
pub static T8: Register = Register {
  ord: 16,
  name: "$t8"
};
pub static T9: Register = Register {
  ord: 17,
  name: "$t9"
};
pub static S0: Register = Register {
  ord: 18,
  name: "$s0"
};
pub static S1: Register = Register {
  ord: 19,
  name: "$s1"
};
pub static S2: Register = Register {
  ord: 20,
  name: "$s2"
};
pub static S3: Register = Register {
  ord: 21,
  name: "$s3"
};
pub static S4: Register = Register {
  ord: 22,
  name: "$s4"
};
pub static S5: Register = Register {
  ord: 23,
  name: "$s5"
};
pub static S6: Register = Register {
  ord: 24,
  name: "$s6"
};
pub static S7: Register = Register {
  ord: 25,
  name: "$s7"
};
pub static K0: Register = Register {
  ord: 26,
  name: "$k0"
};
pub static K1: Register = Register {
  ord: 27,
  name: "$k1"
};
pub static GP: Register = Register {
  ord: 28,
  name: "$gp"
};
pub static SP: Register = Register {
  ord: 29,
  name: "$sp"
};
pub static FP: Register = Register {
  ord: 30,
  name: "$fp"
};
pub static RA: Register = Register {
  ord: 31,
  name: "$ra"
};