use std::{
    fs::{self},
    ops::{Index, IndexMut},
    path::Path,
};

const CAPACITY: usize = 4096;
const OFFSET: usize = 512;

pub struct Memory {
    buffer: [u8; CAPACITY],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            buffer: [0; CAPACITY],
        }
    }

    pub fn from_rom<P: AsRef<Path>>(path: P) -> Self {
        let file_buffer = fs::read(path).expect("FAILED : Could Not Read ROM");
        let mut buffer: [u8; CAPACITY] = [0; CAPACITY];

        // 0
        buffer[0] = 0xF0;
        buffer[1] = 0x90;
        buffer[2] = 0x90;
        buffer[3] = 0x90;
        buffer[4] = 0xF0;

        // 1
        buffer[5] = 0x20;
        buffer[6] = 0x60;
        buffer[7] = 0x20;
        buffer[8] = 0x20;
        buffer[9] = 0x70;

        // 2
        buffer[10] = 0x20;
        buffer[11] = 0x60;
        buffer[12] = 0x20;
        buffer[13] = 0x20;
        buffer[14] = 0x70;

        // 3
        buffer[15] = 0xF0;
        buffer[16] = 0x10;
        buffer[17] = 0xF0;
        buffer[18] = 0x10;
        buffer[19] = 0xF0;

        //4
        buffer[20] = 0x90;
        buffer[21] = 0x90;
        buffer[22] = 0xF0;
        buffer[23] = 0x10;
        buffer[24] = 0x10;

        //5
        buffer[25] = 0xF0;
        buffer[26] = 0x80;
        buffer[27] = 0xF0;
        buffer[28] = 0x10;
        buffer[29] = 0xF0;

        //6
        buffer[30] = 0xF0;
        buffer[31] = 0x80;
        buffer[32] = 0xF0;
        buffer[33] = 0x90;
        buffer[34] = 0xF0;

        //7
        buffer[35] = 0xF0;
        buffer[36] = 0x10;
        buffer[37] = 0x20;
        buffer[38] = 0x40;
        buffer[39] = 0x40;

        // 8
        buffer[40] = 0xF0;
        buffer[41] = 0x90;
        buffer[42] = 0xF0;
        buffer[43] = 0x90;
        buffer[44] = 0xF0;

        // 9
        buffer[45] = 0xF0;
        buffer[46] = 0x90;
        buffer[47] = 0xF0;
        buffer[48] = 0x10;
        buffer[49] = 0xF0;

        // A
        buffer[50] = 0xF0;
        buffer[51] = 0x90;
        buffer[52] = 0xF0;
        buffer[53] = 0x90;
        buffer[54] = 0x90;

        // B
        buffer[55] = 0xE0;
        buffer[56] = 0x90;
        buffer[57] = 0xE0;
        buffer[58] = 0x90;
        buffer[59] = 0xE0;

        // C
        buffer[60] = 0xF0;
        buffer[61] = 0x80;
        buffer[62] = 0x80;
        buffer[63] = 0x80;
        buffer[64] = 0xF0;

        // D
        buffer[65] = 0xE0;
        buffer[66] = 0x90;
        buffer[67] = 0x90;
        buffer[68] = 0x90;
        buffer[69] = 0xE0;

        // E
        buffer[70] = 0xF0;
        buffer[71] = 0x80;
        buffer[72] = 0xF0;
        buffer[73] = 0x80;
        buffer[74] = 0xF0;

        // F
        buffer[75] = 0xF0;
        buffer[76] = 0x80;
        buffer[77] = 0xF0;
        buffer[78] = 0x80;
        buffer[79] = 0x80;

        for (idx, byte) in file_buffer.into_iter().enumerate() {
            if idx < CAPACITY - OFFSET {
                buffer[OFFSET + idx] = byte;
            }
        }

        Self { buffer }
    }

    #[allow(dead_code)]
    pub fn dump(&self) {
        let line_size = 16;
        let chunks = CAPACITY / line_size;
        for i in 0..chunks {
            for j in 0..line_size {
                print!(" {:#06x} ", self.buffer[i * line_size + j])
            }
            println!("");
        }
    }

    pub fn get_cap() -> usize {
        CAPACITY
    }

    pub fn get_offset() -> usize {
        OFFSET
    }
}
impl Index<u16> for Memory {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        if index >= (CAPACITY) as u16 {
            panic!(
                "Memory Index : {} Is Out Of Bounds {}..{}",
                index, 0, CAPACITY
            );
        }
        &self.buffer[index as usize]
    }
}

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        if index >= (CAPACITY) as u16 {
            panic!("Memory Index : {} Is Out Of Bounds", index);
        }
        &mut self.buffer[index as usize]
    }
}
