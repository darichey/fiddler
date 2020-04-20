mod address;
mod instruction;
mod interpreter;
mod memory;
#[allow(dead_code)]
mod registers;
mod service;

use address::Address;
use instruction::Instruction::*;
use interpreter::Interpreter;
use memory::Memory;
use registers::Register::*;

fn main() {
    let program = vec![
        LoadImm { dest: T1, imm: 5 },
        LoadImm { dest: T4, imm: 0 },
        StoreWord {
            from: T1,
            to: Address { base: T4, offset: 0 },
        },
        LoadImm { dest: T2, imm: 7 },
        LoadWord {
            to: T3,
            from: Address { base: T4, offset: 0 },
        },
        Add {
            dest: A0,
            x: T2,
            y: T3,
        },
        LoadImm { dest: V0, imm: 1 },
        SysCall,

        LoadImm { dest: V0, imm: 4 },
        LoadImm { dest: A0, imm: 5 },
        SysCall,
    ];

    let mem = &mut [0u8; 32];
    let mut memory = Memory::new(mem);

    // string happens to be "null terminated" because the default value in the array is 0
    memory.set_string(5, "hello world"); 

    let mut interpreter = Interpreter::new(program, memory);

    interpreter.step();
    assert_eq!(5, interpreter.registers[T1]);
    interpreter.step();
    assert_eq!(0, interpreter.registers[T4]);
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

    interpreter.step();
    interpreter.step();
    interpreter.step();
}
