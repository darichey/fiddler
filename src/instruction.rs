use crate::registers::Register;

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
        dest: Register,
        address: i32,
    },
    StoreWord {
        from: Register,
        address: i32,
    },
}
