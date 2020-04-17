use crate::registers::Register;

pub enum Instruction {
    LoadImm { dest: &'static Register, imm: i32 },
    Add { dest: &'static Register, x: &'static Register, y:&'static Register }
}