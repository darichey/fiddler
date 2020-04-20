use crate::address::Address;
use crate::registers::Register;

#[derive(Copy, Clone)]
pub enum Instruction {
    LoadImm {
        dest: Register,
        imm: i32,
    },
    Add {
        dest: Register,
        x: Register,
        y: Register,
    },
    LoadWord {
        to: Register,
        from: Address,
    },
    StoreWord {
        from: Register,
        to: Address,
    },
    SysCall,
}
