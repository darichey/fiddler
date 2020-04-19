mod instruction;
mod interpreter;
mod memory;
#[allow(dead_code)]
mod registers;
mod service;

use instruction::Instruction::*;
use interpreter::Interpreter;
use memory::Memory;
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
        SysCall,
    ];

    let mem = &mut [0u8; 32];
    let memory = Memory::new(mem);

    let mut interpreter = Interpreter::new(program, memory);
    interpreter.step();
    assert_eq!(5, interpreter.registers[T1]);
    interpreter.step();
    assert_eq!(5, interpreter.memory.get_word(0));
    interpreter.step();
    assert_eq!(7, interpreter.registers[T2]);
    interpreter.step();
    assert_eq!(5, interpreter.registers[T3]);
    interpreter.step();
    assert_eq!(12, interpreter.registers[A0]);
    interpreter.step();
    assert_eq!(1, interpreter.registers[V0]);
    interpreter.step();
}
