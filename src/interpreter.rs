use crate::address::Address;
use crate::instruction::Instruction;
use crate::memory::Memory;
use crate::registers::{Register, Registers};
use crate::service::Service;
use std::convert::TryFrom;
use std::io::Write;

pub struct Interpreter<'a, W: Write> {
    pub registers: Registers,
    pub program: Vec<Instruction>,
    pub pc: usize,
    pub memory: Memory<'a>,
    output: W,
}

impl<W: Write> Interpreter<'_, W> {
    pub fn step(&mut self) {
        let next_ins = self.program[self.pc];
        match next_ins {
            Instruction::Add { dest, x, y } => {
                let res = self.registers[x] + self.registers[y];
                self.registers[dest] = res;

                self.pc += 1;
            }

            Instruction::LoadImm { dest, imm } => {
                self.registers[dest] = imm;

                self.pc += 1;
            }

            Instruction::LoadWord { to, from } => {
                self.registers[to] = self.memory.get_word(self.calculate_address(from));

                self.pc += 1;
            }

            Instruction::StoreWord { from, to } => {
                self.memory.set_word(self.calculate_address(to), self.registers[from]);

                self.pc += 1;
            }

            Instruction::SysCall => {
                let service = Service::try_from(self.registers[Register::V0]).expect("bad service");
                self.exec_service(service);
                self.pc += 1;
            }
        }
    }

    fn exec_service(&mut self, service: Service) {
        match service {
            Service::PrintInt => {
                let arg = self.registers[Register::A0];
                write!(self.output, "{}", arg).unwrap();
            }

            Service::PrintString => {
                let address = self.registers[Register::A0];
                let string = self.memory.get_string(address as usize);
                write!(self.output, "{}", string).unwrap();
            }
        }
    }

    fn calculate_address(&self, address: Address) -> usize {
        self.registers[address.base] as usize + address.offset
    }

    pub fn new<'a>(program: Vec<Instruction>, memory: Memory<'a>, output: W) -> Interpreter<'a, W> {
        Interpreter {
            registers: Registers::new(),
            program,
            pc: 0,
            memory,
            output,
        }
    }
}
