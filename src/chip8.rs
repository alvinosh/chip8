use crate::{
    cpu::{OpCode, CPU},
    memory::Memory,
};

pub struct CHIP8 {
    cpu: CPU,
    memory: Memory,
}

impl CHIP8 {
    pub fn create(cpu: CPU, memory: Memory) -> Self {
        Self { cpu, memory }
    }

    pub fn get_cpu(&self) -> &CPU {
        &self.cpu
    }

    pub fn get_memory(&self) -> &Memory {
        &self.memory
    }

    pub fn execute(&mut self) -> Result<OpCode, String> {
        let instruction = self.cpu.fetch(&mut self.memory);
        let op_code = self.cpu.decode(instruction);
        Ok(op_code)
        // self.cpu.execute(&mut self.memory)
    }
}
