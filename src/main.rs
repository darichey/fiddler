mod instruction;
mod interpreter;
mod registers;

use instruction::Instruction;
use interpreter::Interpreter;
use registers::*;

fn main() {
    // let prog = program! {
    //     LoadImm "$t1", 5,
    //     LoadImm "$t2", 5,
    //     Add "$t0", "$t1", "$t2"
    // };

    let program = vec![
        Instruction::LoadImm {
            dest: &T1,
            imm: 5
        },
        Instruction::LoadImm {
            dest: &T2,
            imm: 7
        },
        Instruction::Add {
            dest: &T0,
            x: &T1,
            y: &T2,
        }
    ];
    let mut interpreter = Interpreter::new(program);
    interpreter.step();
    assert_eq!(5, interpreter.get_register(&T1));
    interpreter.step();
    assert_eq!(7, interpreter.get_register(&T2));
    interpreter.step();
    assert_eq!(12, interpreter.get_register(&T0));
}
