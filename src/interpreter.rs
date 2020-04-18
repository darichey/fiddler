use crate::instruction::Instruction;
use crate::registers::Register;

pub struct Interpreter<'a> {
    registers: [i32; 32],
    program: Vec<Instruction>,
    pc: usize,
    memory: &'a mut [i32],
}

impl Interpreter<'_> {
    pub fn step(&mut self) {
        let next_ins = &self.program[self.pc];
        match next_ins {
            Instruction::Add { dest, x, y } => {
                let res = self.registers[x.ord as usize] + self.registers[y.ord as usize];
                self.registers[dest.ord as usize] = res;

                self.pc += 1;
            }

            Instruction::LoadImm { dest, imm } => {
                self.registers[dest.ord as usize] = *imm;

                self.pc += 1;
            }

            Instruction::LoadWord { dest, address } => {
                self.registers[dest.ord as usize] = self.memory[*address as usize];

                self.pc += 1;
            }

            Instruction::StoreWord { from, address } => {
                self.memory[*address as usize] = self.registers[from.ord as usize];

                self.pc += 1;
            }
        }
    }

    pub fn new<'a>(program: Vec<Instruction>, memory: &'a mut [i32]) -> Interpreter<'a> {
        Interpreter {
            registers: [0; 32],
            program: program,
            pc: 0,
            memory: memory,
        }
    }

    pub fn get_register(&self, reg: &Register) -> i32 {
        self.registers[reg.ord as usize]
    }

    pub fn get_memory(&self, address: i32) -> i32 {
        self.memory[address as usize]
    }
}
