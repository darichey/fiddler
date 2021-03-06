use crate::address::Address;
use crate::instruction::Instruction;
use crate::memory::Memory;
use crate::registers::{Register, Registers};
use crate::service::Service;
use std::convert::TryFrom;
use std::io::Write;

pub struct Interpreter<'a, W: Write> {
    pub program: Vec<Instruction>,
    pub registers: Registers,
    pub memory: Memory<'a>,
    output: W,
    pc: usize,
}

impl<W: Write> Interpreter<'_, W> {
    pub fn run(&mut self) {
        while self.pc < self.program.len() {
            self.step();
        }
    }

    pub fn step(&mut self) {
        let next_ins = self.program[self.pc];
        match next_ins {
            Instruction::Add { dest, x, y } => {
                let res = self.registers.get(x) + self.registers.get(y);
                self.registers.set(dest, res);

                self.pc += 1;
            }

            Instruction::LoadImm { dest, imm } => {
                self.registers.set(dest, imm);

                self.pc += 1;
            }

            Instruction::LoadWord { to, from } => {
                self.registers.set(to, self.memory.get_word(self.calculate_address(from)));

                self.pc += 1;
            }

            Instruction::StoreWord { from, to } => {
                self.memory
                    .set_word(self.calculate_address(to), self.registers.get(from));

                self.pc += 1;
            }

            Instruction::SysCall => {
                let service = Service::try_from(self.registers.get(Register::V0)).expect("bad service");
                self.exec_service(service);
                self.pc += 1;
            }
        }
    }

    fn exec_service(&mut self, service: Service) {
        match service {
            Service::PrintInt => {
                let arg = self.registers.get(Register::A0);
                write!(self.output, "{}", arg).unwrap();
            }

            Service::PrintString => {
                let address = self.registers.get(Register::A0);
                let string = self.memory.get_string(address as usize);
                write!(self.output, "{}", string).unwrap();
            }
        }
    }

    fn calculate_address(&self, address: Address) -> usize {
        self.registers.get(address.base) as usize + address.offset
    }

    pub fn new<'a>(
        program: Vec<Instruction>,
        registers: Registers,
        memory: Memory<'a>,
        output: W,
    ) -> Interpreter<'a, W> {
        Interpreter {
            program,
            registers,
            memory,
            output,
            pc: 0,
        }
    }
}
