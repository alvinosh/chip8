use crate::memory::Memory;

type NNN = u16;
type NN = u8;
type N = u8;
type X = u8;
type Y = u8;

#[derive(Debug)]
pub enum OpCode {
    None,
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
    pc: u16,
    stack: [u16; 12],
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: [0; 16],
            i: 0,
            pc: 512,
            stack: [0; 12],
        }
    }

    pub fn fetch(&mut self, mem: &mut Memory) -> u16 {
        let most_sig = mem[self.pc + 0];
        let lest_sig = mem[self.pc + 1];
        self.pc += 2;
        (most_sig as u16) << 8 | lest_sig as u16
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
            output += (buf[0] as u16) << 0;
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
            _ => OpCode::None,
        }

        // print!("VALUE : ");

        // for i in 0..4 {
        //     print!(" {:#03x} ", buf[i])
        // }
        // println!()
    }

    pub fn execute(&mut self, mem: &mut Memory) -> Result<(), String> {
        Ok(())
    }

    pub fn log_value<T>(&self, label: &str, value: T)
    where
        T: Into<u16>,
    {
        let val: u16 = value.try_into().unwrap();
        println!(
            "{0} {1:#06x} : {2:#06x} {2:#018b} {2:#05}",
            label, self.pc, val
        );
    }
}
