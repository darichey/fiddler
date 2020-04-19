use std::ops::{Index, IndexMut};

pub struct Memory<'a> {
    underlying: &'a mut [u8],
}

impl Memory<'_> {
    pub fn new(underlying: &mut [u8]) -> Memory {
        Memory { underlying }
    }

    pub fn get_word(&mut self, address: i32) -> i32 {
        i32::from_ne_bytes([
            self[address + 0],
            self[address + 1],
            self[address + 2],
            self[address + 3],
        ])
    }

    pub fn set_word(&mut self, address: i32, word: i32) {
        let bytes = word.to_ne_bytes();
        self[address + 0] = bytes[0];
        self[address + 1] = bytes[1];
        self[address + 2] = bytes[2];
        self[address + 3] = bytes[3];
    }
}

impl Index<i32> for Memory<'_> {
    type Output = u8;

    fn index(&self, index: i32) -> &u8 {
        &self.underlying[index as usize]
    }
}

impl IndexMut<i32> for Memory<'_> {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        &mut self.underlying[index as usize]
    }
}
