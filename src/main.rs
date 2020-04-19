mod instruction;
mod interpreter;
#[allow(dead_code)]
mod registers;
mod service;

use instruction::Instruction::*;
use interpreter::Interpreter;
use registers::Register::*;

fn main() {
    let program = vec![
        LoadImm { dest: T1, imm: 5 },
        StoreWord {
            from: T1,
            address: 0,
        },
        LoadImm { dest: T2, imm: 7 },
        LoadWord {
            dest: T3,
            address: 0,
        },
        Add {
            dest: A0,
            x: T2,
            y: T3,
        },
        LoadImm { dest: V0, imm: 1 },
        SysCall
    ];

    let memory = &mut [0i32; 10];

    let mut interpreter = Interpreter::new(program, memory);
    interpreter.step();
    assert_eq!(5, interpreter.get_register(&T1));
    interpreter.step();
    assert_eq!(5, interpreter.get_memory(0));
    interpreter.step();
    assert_eq!(7, interpreter.get_register(&T2));
    interpreter.step();
    assert_eq!(5, interpreter.get_register(&T3));
    interpreter.step();
    assert_eq!(12, interpreter.get_register(&A0));
    interpreter.step();
    assert_eq!(1, interpreter.get_register(&V0));
    interpreter.step();
}
