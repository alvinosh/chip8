mod chip8;
mod cpu;
mod display;
mod memory;

use chip8::CHIP8;
use cpu::{OpCode, CPU};
use display::Display;
use memory::Memory;

fn main() {
    let cpu = CPU::new();
    let memory = Memory::from_rom("Airplane");
    let mut display = Display::new("CHIP-8");

    let mut chip8 = CHIP8::create(cpu, memory, display);

    // chip8.get_memory().dump();
    let debug = false;
    let log_all = false;

    while let Ok(op_code) = chip8.execute(debug) {
        match op_code {
            OpCode::HALT => {}
            _ => {
                if log_all {
                    let format = format!("{:?}", op_code);
                    let len = format.len();
                    print!("{} {:<1$} |", format, 20 - len);
                    chip8.log_last();
                }
            }
        }
    }
}
