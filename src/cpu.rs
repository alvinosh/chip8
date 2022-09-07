use crate::{
    display::Display,
    keyboard::{self, KeyBoardEvent, Keyboard},
    memory::Memory,
};
use rand::Rng;

type NNN = u16;
type NN = u8;
type N = u8;
type X = u8;
type Y = u8;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum OpCode {
    NONE,
    HALT,
    ROUTINE(u16),
    CLEAR,
    RETURN,

    GOTO(NNN),
    CALL(u16),

    EQ(X, NN),
    NEQ(X, NN),
    EQ_REG(X, Y),

    SET_CONST(X, NN),
    ADD_CONST(X, NN),

    OR(X, Y),
    AND(X, Y),
    XOR(X, Y),

    SET_REG(X, Y),
    ADD_REG(X, Y),
    SUB_REG(X, Y),

    BSHIFT_RGHT(X, Y),
    BSHIFT_LEFT(X, Y),

    SUBTRACT(X, Y),
    NEQ_REG(X, Y),

    SETI(NNN),
    JUMP(NNN),
    RAND(X, NN),

    DRAW(X, Y, N),

    KEY_P(X),
    KEY_NP(X),

    GET_DELAY(X),
    GET_KEY(X),
    SET_DELAY(X),
    SET_SOUND(X),

    ADDI(X),
    SPRI(X),

    BCP(X),

    DUMP(X),
    LOAD(X),
}

pub struct CPU {
    registers: [u8; 16],
    i: u16,
    #[allow(dead_code)]
    sound: u8,
    #[allow(dead_code)]
    delay: u8,

    pc: u16,
    sp: u8,

    stack: [u16; 12],

    latest_fetch: u16,
    latest_addr: u16,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: [0; 16],
            i: 0,
            delay: 0,
            sound: 0,

            pc: Memory::get_offset() as u16,
            sp: 0,
            stack: [0; 12],

            latest_fetch: 0,
            latest_addr: 0,
        }
    }

    pub fn fetch(&mut self, mem: &mut Memory) -> u16 {
        let most_sig = mem[self.pc + 0];
        let lest_sig = mem[self.pc + 1];
        // self.log_value(" -- ", (most_sig as u16) << 8 | lest_sig as u16);

        self.latest_fetch = (most_sig as u16) << 8 | lest_sig as u16;
        self.latest_addr = self.pc;
        self.latest_fetch
    }

    pub fn decode(&self, instruction: u16) -> OpCode {
        // the buffer actually only needs to store 16 bits (4bits * 4)
        // however for simplicity the array holds 32 bits (8bits * 4)
        // to make indexing and splitting easier
        let mut buf: [u8; 4] = [0; 4];
        // we want an instruction that looks like
        // 1010011110001111 to be split into
        // 1010 0111 1000 1111
        // we first mask the bits we actually care about, in chunks of 4
        // we generate the mask by shifting a 1111 nibble 12 bytes for the first chunk, then 8 and so on
        // then we shift the masked value back so the number can be accurately cast as a u8
        buf[0] = ((instruction & 0b1111 << 12) >> 12) as u8;
        buf[1] = ((instruction & 0b1111 << 08) >> 08) as u8;
        buf[2] = ((instruction & 0b1111 << 04) >> 04) as u8;
        buf[3] = ((instruction & 0b1111 << 00) >> 00) as u8;

        fn tripple(buf: [u8; 3]) -> u16 {
            let mut output: u16 = 0;
            output += (buf[0] as u16) << 8;
            output += (buf[1] as u16) << 4;
            output += (buf[2] as u16) << 0;
            output
        }

        fn double(buf: [u8; 2]) -> u8 {
            let mut output: u8 = 0;
            output += (buf[0] << 4) as u8;
            output += (buf[1] << 0) as u8;
            output
        }

        fn single(buf: u8) -> u8 {
            let mut output: u8 = 0;
            output += (buf << 0) as u8;
            output
        }

        match buf {
            [0x0, 0x0, 0x0, 0x0] => OpCode::HALT,
            [0x0, 0x0, 0xE, 0x0] => OpCode::CLEAR,
            [0x0, 0x0, 0xE, 0xE] => OpCode::RETURN,
            [0x0, nnn @ ..] => OpCode::ROUTINE(tripple(nnn)),
            [0x1, nnn @ ..] => OpCode::GOTO(tripple(nnn)),
            [0x2, nnn @ ..] => OpCode::CALL(tripple(nnn)),
            [0x3, x, nn @ ..] => OpCode::EQ(single(x), double(nn)),
            [0x4, x, nn @ ..] => OpCode::NEQ(single(x), double(nn)),
            [0x5, x, y, 0x0] => OpCode::EQ_REG(single(x), single(y)),
            [0x6, x, nn @ ..] => OpCode::SET_CONST(single(x), double(nn)),
            [0x7, x, nn @ ..] => OpCode::ADD_CONST(single(x), double(nn)),
            [0x8, x, y, 0x0] => OpCode::SET_REG(single(x), single(y)),
            [0x8, x, y, 0x1] => OpCode::OR(single(x), single(y)),
            [0x8, x, y, 0x2] => OpCode::AND(single(x), single(y)),
            [0x8, x, y, 0x3] => OpCode::XOR(single(x), single(y)),
            [0x8, x, y, 0x4] => OpCode::ADD_REG(single(x), single(y)),
            [0x8, x, y, 0x5] => OpCode::SUB_REG(single(x), single(y)),
            [0x8, x, y, 0x6] => OpCode::BSHIFT_RGHT(single(x), single(y)),
            [0x8, x, y, 0x7] => OpCode::SUBTRACT(single(x), single(y)),
            [0x8, x, y, 0xE] => OpCode::BSHIFT_LEFT(single(x), single(y)),
            [0x9, x, y, 0x0] => OpCode::NEQ_REG(single(x), single(y)),
            [0xA, nnn @ ..] => OpCode::SETI(tripple(nnn)),
            [0xB, nnn @ ..] => OpCode::JUMP(tripple(nnn)),
            [0xC, x, nn @ ..] => OpCode::RAND(single(x), double(nn)),
            [0xD, x, y, n] => OpCode::DRAW(single(x), single(y), single(n)),
            [0xE, x, 0x9, 0xE] => OpCode::KEY_P(single(x)),
            [0xE, x, 0xA, 0x1] => OpCode::KEY_NP(single(x)),
            [0xF, x, 0x0, 0x7] => OpCode::GET_DELAY(single(x)),
            [0xF, x, 0x0, 0xA] => OpCode::GET_KEY(single(x)),
            [0xF, x, 0x1, 0x5] => OpCode::SET_DELAY(single(x)),
            [0xF, x, 0x1, 0x8] => OpCode::SET_SOUND(single(x)),
            [0xF, x, 0x1, 0xE] => OpCode::ADDI(single(x)),
            [0xF, x, 0x2, 0x9] => OpCode::SPRI(single(x)),
            [0xF, x, 0x3, 0x3] => OpCode::BCP(single(x)),
            [0xF, x, 0x5, 0x5] => OpCode::DUMP(single(x)),
            [0xF, x, 0x6, 0x5] => OpCode::LOAD(single(x)),
            _ => OpCode::NONE,
        }
    }

    pub fn execute(
        &mut self,
        mem: &mut Memory,
        display: &mut Display,
        keyboard: &mut Keyboard,
        opcode: &OpCode,
    ) -> Result<(), String> {
        let mut inc = true;

        match opcode {
            OpCode::NONE => {}
            OpCode::HALT => {}
            OpCode::ROUTINE(_) => {
                self.log_last();
                todo!();
            }
            OpCode::CLEAR => display.clear_dispaly(),
            OpCode::CALL(addr) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = *addr;
                inc = false;
            }
            OpCode::RETURN => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
                inc = true;
            }
            OpCode::GOTO(addr) => {
                self.pc = *addr;
                inc = false;
            }

            OpCode::EQ(reg, val) => {
                if self.registers[*reg as usize] == *val {
                    println!("LOL");
                    self.pc += 2;
                }
            }
            OpCode::NEQ(reg, val) => {
                if self.registers[*reg as usize] != *val {
                    self.pc += 2;
                }
            }
            OpCode::EQ_REG(rega, regb) => {
                if self.registers[*rega as usize] == self.registers[*regb as usize] {
                    self.pc += 2;
                }
            }
            OpCode::SET_CONST(reg, val) => {
                self.registers[*reg as usize] = *val;
            }
            OpCode::ADD_CONST(reg, val) => {
                self.registers[*reg as usize] += *val;
            }
            OpCode::OR(rega, regb) => {
                self.registers[*rega as usize] =
                    self.registers[*rega as usize] | self.registers[*regb as usize]
            }
            OpCode::AND(rega, regb) => {
                self.registers[*rega as usize] =
                    self.registers[*rega as usize] & self.registers[*regb as usize]
            }
            OpCode::XOR(rega, regb) => {
                self.registers[*rega as usize] =
                    self.registers[*rega as usize] ^ self.registers[*regb as usize]
            }
            OpCode::SET_REG(rega, regb) => {
                self.registers[*rega as usize] = self.registers[*regb as usize]
            }
            OpCode::ADD_REG(rega, regb) => {
                self.registers[*rega as usize] =
                    self.registers[*rega as usize] + self.registers[*regb as usize]
            }
            OpCode::SUB_REG(rega, regb) => {
                self.registers[*rega as usize] =
                    self.registers[*rega as usize] - self.registers[*regb as usize]
            }
            OpCode::BSHIFT_RGHT(rega, _regb) => {
                self.registers[15] = self.registers[*rega as usize] >> 7;
                self.registers[*rega as usize] = self.registers[*rega as usize] >> 1
            }
            OpCode::BSHIFT_LEFT(rega, _regb) => {
                self.registers[15] = self.registers[*rega as usize] & 0b1;
                self.registers[*rega as usize] = self.registers[*rega as usize] << 1
            }
            OpCode::SUBTRACT(rega, regb) => {
                self.registers[*rega as usize] =
                    self.registers[*regb as usize] - self.registers[*rega as usize];
            }
            OpCode::NEQ_REG(rega, regb) => {
                if self.registers[*rega as usize] != self.registers[*regb as usize] {
                    self.pc += 2;
                }
            }
            OpCode::SETI(addr) => {
                self.i = *addr;
            }
            OpCode::JUMP(addr) => {
                self.pc = self.registers[0] as u16 + *addr;
                inc = false;
            }
            OpCode::RAND(reg, val) => {
                let num = rand::thread_rng().gen_range(0..255);
                self.registers[*reg as usize] = num & *val;
            }
            OpCode::DRAW(rega, regb, n) => {
                let x = self.registers[*rega as usize];
                let y = self.registers[*regb as usize];
                for i in 0..*n {
                    let bits: u8 = mem[self.i + i as u16];
                    // println!("BITS: {:#010b}", bits);
                    for j in 0..8 {
                        let bit = (bits >> (8 - j)) & 0b1;
                        if display.draw_suqare(x + j, y + i, bit == 1) {
                            self.registers[15] = 1;
                        }
                    }
                }
            }
            OpCode::KEY_P(reg) => {
                if keyboard.is_key_pressed(self.registers[*reg as usize]) {
                    self.pc += 2;
                }
            }
            OpCode::KEY_NP(reg) => {
                if !keyboard.is_key_pressed(self.registers[*reg as usize]) {
                    self.pc += 2;
                }
            }
            OpCode::GET_KEY(reg) => loop {
                if let KeyBoardEvent::KeyPressed(n) = keyboard.wait_key(display) {
                    self.registers[*reg as usize] = n;
                    break;
                }
            },
            OpCode::GET_DELAY(reg) => {
                self.registers[*reg as usize] = self.delay;
            }
            OpCode::SET_DELAY(reg) => self.delay = self.registers[*reg as usize],
            OpCode::SET_SOUND(reg) => self.sound = self.registers[*reg as usize],
            OpCode::ADDI(reg) => {
                self.i += self.registers[*reg as usize] as u16;
            }
            OpCode::SPRI(reg) => {
                let char = self.registers[*reg as usize];
                self.i = char as u16 * 5 as u16;
            }
            OpCode::BCP(reg) => {
                let decimal = self.registers[*reg as usize];
                mem[self.i] = decimal / 100;
                mem[self.i + 1] = (decimal % 100) / 10;
                mem[self.i + 2] = decimal % 10;
            }
            OpCode::DUMP(idx) => {
                for i in 0..*idx {
                    mem[self.i + i as u16] = self.registers[i as usize];
                }
            }
            OpCode::LOAD(idx) => {
                for i in 0..*idx {
                    self.registers[i as usize] = mem[self.i + i as u16];
                }
            }
        }

        if inc {
            self.pc += 2
        };
        Ok(())
    }

    pub fn delay_timer(&mut self) {
        if self.delay > 0 {
            self.delay -= 1;
        }
    }

    pub fn sound_timer(&mut self) {
        if self.sound > 0 {
            self.sound -= 1;
        }
    }

    pub fn log_value<T>(&self, label: &str, value: T)
    where
        T: Into<u16>,
    {
        let val: u16 = value.try_into().unwrap();
        println!(
            "{0} ADDR {1:#06x} : VAL {2:#06x} {2:#018b} {2:#05}",
            label, self.latest_addr, val
        );
    }

    pub fn log_addr(&mut self, mem: &mut Memory, addr: u16) {
        print!("ADDR : {:#06x}", addr);
        self.log_value("|", mem[addr]);
    }

    pub fn log_last(&mut self) {
        let opcode = self.decode(self.latest_fetch);
        let format = format!("{:?}", opcode);
        let len = format.len();
        print!("{} {:<1$} |", format, 20 - len);
        self.log_value("|", self.latest_fetch);
    }

    pub fn pc_valid(&self) -> bool {
        (self.pc as usize) < Memory::get_cap()
    }
}
