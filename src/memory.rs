use std::str;

pub struct Memory<'a> {
    underlying: &'a mut [u8],
}

impl Memory<'_> {
    pub fn new(underlying: &mut [u8]) -> Memory {
        Memory { underlying }
    }

    pub fn new_empty() -> Memory<'static> {
        Memory {
            underlying: &mut [0; 0],
        }
    }

    pub fn get_word(&self, address: usize) -> i32 {
        i32::from_ne_bytes([
            self.underlying[address + 0],
            self.underlying[address + 1],
            self.underlying[address + 2],
            self.underlying[address + 3],
        ])
    }

    pub fn set_word(&mut self, address: usize, word: i32) {
        let bytes = word.to_ne_bytes();
        self.underlying[address + 0] = bytes[0];
        self.underlying[address + 1] = bytes[1];
        self.underlying[address + 2] = bytes[2];
        self.underlying[address + 3] = bytes[3];
    }

    pub fn get_string(&self, address: usize) -> &str {
        let mut end = address;
        while end < self.underlying.len() && self.underlying[end] != 0 {
            end += 1;
        }
        let bytes = &self.underlying[address .. end];
        str::from_utf8(bytes).unwrap()
    }

    pub fn set_string(&mut self, address: usize, string: &str) {
        let bytes = string.as_bytes();
        for i in 0..string.len() {
            self.underlying[address + i] = bytes[i]; 
        }
    }
}
