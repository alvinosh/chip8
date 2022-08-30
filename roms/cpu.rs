use crate::memory::Memory;

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

    pub fn execute(&mut self, mem: &mut Memory) -> Result<(), String> {
        let most_sig = mem[self.pc + 0];
        let lest_sig = mem[self.pc + 1];

        let value: u16 = (most_sig as u16) << 8 | lest_sig as u16;

        self.log_value("   VALUE", value);

        self.pc += 2;
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
