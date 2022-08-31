mod chip8;
mod cpu;
mod memory;

use chip8::CHIP8;
use cpu::{OpCode, CPU};

use memory::Memory;

fn main() {
    let cpu = CPU::new();
    let memory = Memory::from_rom("c8_test");
    let mut chip8 = CHIP8::create(cpu, memory);

    // chip8.get_memory().dump();

    for _ in 0..=(4096 - 512) {
        if let Ok(op_code) = chip8.execute() {
            match op_code {
                OpCode::HALT => {
                    println!("{:?}", op_code);
                    break;
                }
                _ => println!("{:?}", op_code),
            }
        }
    }
}
