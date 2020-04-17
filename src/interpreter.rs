use crate::instruction::Instruction;

macro_rules! parse_registers {
    ($reg:expr) => {
        Interpreter::parse_register_idx($reg)
    };

    ( $( $reg:expr ),+ ) => {
        ( $( Interpreter::parse_register_idx($reg) ),+ )
    };
}

static REGISTER_NAMES: &[&'static str] = &[
    "$zero", "$at", "$v0", "$v1", "$a0", "$a1", "$a2", "$a3", "$t0", "$t1", "$t2", "$t3", "$t4",
    "$t5", "$t6", "t7", "$s0", "$s1", "$s2", "$s3", "$s4", "$s5", "$s6", "$s7", "$t8", "$t9",
    "$k0", "$k1", "$gp", "$sp", "$fp", "$ra",
];

pub struct Interpreter<'a> {
    registers: [i32; 32],
    program: Vec<Instruction<'a>>,
    pc: usize,
}

impl Interpreter<'_> {
    pub fn step(&mut self) {
        let next_ins = &self.program[self.pc];
        match next_ins {
            Instruction::Add { dest, x, y } => {
                if let (Some(dest_reg), Some(x_reg), Some(y_reg)) = parse_registers!(dest, x, y) {
                    self.registers[dest_reg] = self.registers[x_reg] + self.registers[y_reg];
                    self.pc += 1;
                } else {
                    panic!("Error parsing Add");
                }
            },

            Instruction::LoadImm { dest, imm } => {
                if let Some(dest_reg) = parse_registers!(dest) {
                    self.registers[dest_reg] = *imm;
                    self.pc += 1;
                }
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

    pub fn get_register(&self, reg: &str) -> Option<i32> {
        Interpreter::parse_register_idx(reg)
            .and_then(|idx| self.registers.get(idx).cloned())
    }

    fn parse_register_idx(reg: &str) -> Option<usize> {
        REGISTER_NAMES.iter().position(|&x| x == reg)
    }
}