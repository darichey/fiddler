mod instruction;
mod interpreter;
mod registers;

use instruction::Instruction::*;
use interpreter::Interpreter;
use registers::*;

fn main() {
    let program = vec![
        LoadImm { dest: &T1, imm: 5 },
        LoadImm { dest: &T2, imm: 7 },
        Add {
            dest: &T0,
            x: &T1,
            y: &T2,
        },
    ];
    let mut interpreter = Interpreter::new(program);
    interpreter.step();
    assert_eq!(5, interpreter.get_register(&T1));
    interpreter.step();
    assert_eq!(7, interpreter.get_register(&T2));
    interpreter.step();
    assert_eq!(12, interpreter.get_register(&T0));
}
