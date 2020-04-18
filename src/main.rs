mod instruction;
mod interpreter;
#[allow(dead_code)]
mod registers;

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
            dest: T0,
            x: T2,
            y: T3,
        },
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
    assert_eq!(12, interpreter.get_register(&T0));
}
