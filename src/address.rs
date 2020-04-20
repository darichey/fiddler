use crate::registers::Register;

#[derive(Copy, Clone)]
pub struct Address {
    pub base: Register,
    pub offset: usize,
}