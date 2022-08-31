use std::{
    fs::{self},
    ops::{Index, IndexMut},
};

pub struct Memory {
    buffer: [u8; 4096],
}

impl Memory {
    pub fn new() -> Self {
        let mut buffer: [u8; 4096] = [0; 4096];

        Self { buffer }
    }

    pub fn from_rom(name: &str) -> Self {
        let file_buffer =
            fs::read(format!("roms/{}.c8", name)).expect("FAILED : Could Not Read ROM");
        let mut buffer: [u8; 4096] = [0; 4096];

        for (idx, byte) in file_buffer.into_iter().enumerate() {
            if idx < 4096 - 512 {
                buffer[512 + idx] = byte;
            }
        }

        Self { buffer }
    }

    pub fn dump(&self) {
        let line_size = 16;
        let chunks = 4096 / line_size;
        for i in 0..chunks {
            for j in 0..line_size {
                print!(" {:#06x} ", self.buffer[i * line_size + j])
            }
            println!("");
        }
    }

    pub fn get_buffer(&self) -> [u8; 4096] {
        return self.buffer;
    }
}
impl Index<u16> for Memory {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        if index < 0 || index >= 4096 {
            panic!("Memory Index Out Of Bounds");
        }
        &self.buffer[index as usize]
    }
}

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        if index < 0 || index >= 4096 {
            panic!("Memory Index Out Of Bounds");
        }
        &mut self.buffer[index as usize]
    }
}
