mod chip8;
mod cpu;
mod memory;

use chip8::CHIP8;
use cpu::CPU;
use memory::Memory;

fn main() {
    let cpu = CPU::new();
    let memory = Memory::from_rom("maze");
    let mut chip8 = CHIP8::create(cpu, memory);

    // chip8.get_memory().dump();

    for _ in 0..=32 {
        chip8.execute().unwrap();
    }
}
