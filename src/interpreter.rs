use crate::instruction::Instruction;
use crate::registers::Register;

pub struct Interpreter {
    registers: [i32; 32],
    program: Vec<Instruction>,
    pc: usize,
}

impl Interpreter {
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
        }
    }

    pub fn new(program: Vec<Instruction>) -> Interpreter {
        Interpreter {
            registers: [0; 32],
            program: program,
            pc: 0,
        }
    }

    pub fn get_register(&self, reg: &Register) -> i32 {
        self.registers[reg.ord as usize]
    }
}
