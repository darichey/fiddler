use crate::registers::Register;

pub enum Instruction {
    LoadImm {
        dest: &'static Register,
        imm: i32,
    },
    Add {
        dest: &'static Register,
        x: &'static Register,
        y: &'static Register,
    },
    LoadWord {
        dest: &'static Register,
        address: i32,
    },
    StoreWord {
        from: &'static Register,
        address: i32,
    }
}
