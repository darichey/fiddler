mod instruction;
mod interpreter;

use instruction::Instruction;
use interpreter::Interpreter;

fn main() {
    // let prog = program! {
    //     LoadImm "$t1", 5,
    //     LoadImm "$t2", 5,
    //     Add "$t0", "$t1", "$t2"
    // };

    let program = vec![
        Instruction::LoadImm {
            dest: "$t1",
            imm: 5
        },
        Instruction::LoadImm {
            dest: "$t2",
            imm: 7
        },
        Instruction::Add {
            dest: "$t0",
            x: "$t1",
            y: "$t2",
        }
    ];
    let mut interpreter = Interpreter::new(program);
    interpreter.step();
    assert_eq!(5, interpreter.get_register("$t1").unwrap());
    interpreter.step();
    assert_eq!(7, interpreter.get_register("$t2").unwrap());
    interpreter.step();
    assert_eq!(12, interpreter.get_register("$t0").unwrap());
}
