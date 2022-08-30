use crate::{cpu::CPU, memory::Memory};

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

    pub fn execute(&mut self) -> Result<(), String> {
        self.cpu.execute(&mut self.memory)
    }
}
