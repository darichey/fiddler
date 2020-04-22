use fiddler::address::Address;
use fiddler::default_registers;
use fiddler::instruction::Instruction::*;
use fiddler::interpreter::Interpreter;
use fiddler::memory::Memory;
use fiddler::registers::Register::*;
use fiddler::registers::Registers;
use std::io::{self, Cursor};

#[test]
fn test_load_imm() {
    let program = vec![LoadImm { dest: T0, imm: 5 }];
    let mut interpreter =
        Interpreter::new(program, Registers::new(), Memory::new_empty(), io::sink());

    interpreter.run();

    assert_eq!(5, interpreter.registers[T0]);
}

#[test]
fn test_add() {
    let program = vec![Add {
        dest: T2,
        x: T0,
        y: T1,
    }];
    let registers = default_registers!(T0 => 1, T1 => 5);
    let mut interpreter = Interpreter::new(program, registers, Memory::new_empty(), io::sink());

    interpreter.run();

    assert_eq!(6, interpreter.registers[T2]);
}

#[test]
fn test_load_word() {
    let program = vec![LoadWord {
        to: T0,
        from: Address {
            base: T1,
            offset: 0,
        },
    }];

    let registers = default_registers!(T1 => 0);
    let buf = &mut [0u8; 4];
    let mut memory = Memory::new(buf);
    memory.set_word(0, 0x1234);

    let mut interpreter = Interpreter::new(program, registers, memory, io::sink());

    interpreter.run();

    assert_eq!(0x1234, interpreter.registers[T0]);
}

#[test]
fn test_store_word() {
    let program = vec![StoreWord {
        from: T0,
        to: Address {
            base: T1,
            offset: 0,
        },
    }];

    let registers = default_registers!(T0 => 0x1234, T1 => 0);
    let buf = &mut [0u8; 4];
    let memory = Memory::new(buf);

    let mut interpreter = Interpreter::new(program, registers, memory, io::sink());

    interpreter.run();

    assert_eq!(0x1234, interpreter.memory.get_word(0));
}

#[test]
fn test_print_int() {
    let program = vec![SysCall];
    let registers = default_registers!(V0 => 1, A0 => 1234);

    let mut buf = [0u8; 4];
    let output = Cursor::new(&mut buf[..]);
    let mut interpreter = Interpreter::new(program, registers, Memory::new_empty(), output);

    interpreter.run();

    assert_eq!(b"1234", &buf);
}

#[test]
fn test_print_string() {
    let program = vec![SysCall];
    let registers = default_registers!(V0 => 4, A0 => 0);

    let mem_buf = &mut [0u8; 11];
    let mut memory = Memory::new(mem_buf);
    memory.set_string(0, "hello world");

    let mut out_buf = [0u8; 11];
    let output = Cursor::new(&mut out_buf[..]);

    let mut interpreter = Interpreter::new(program, registers, memory, output);

    interpreter.run();

    assert_eq!(b"hello world", &out_buf);
}
