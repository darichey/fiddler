pub enum Instruction<'a> {
    LoadImm { dest: &'a str, imm: i32 },
    Add { dest: &'a str, x: &'a str, y: &'a str }
}