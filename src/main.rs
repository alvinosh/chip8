mod chip8;
mod cpu;
mod debug_levels;
mod display;
mod keyboard;
mod memory;

use std::{io, path::Path};

use chip8::CHIP8;
use cpu::CPU;
use debug_levels::DebugLevels;
use display::Display;
use keyboard::Keyboard;
use memory::Memory;

fn main() -> Result<(), String> {
    print!("Enter A Path To A Valid Rom");

    let mut path_file = String::new();
    let memory = match io::stdin().read_line(&mut path_file) {
        Ok(n) => {
            let trimmed_path_file = path_file.trim();
            let path = Path::new(&trimmed_path_file);
            if path.exists() {
                Memory::from_rom(path)
            } else {
                return Err("Path Does Not Exist".to_string());
            }
        }
        Err(error) => return Err("Failed".to_string()),
    };

    let cpu = CPU::new();
    let mut display = Display::new("CHIP-8");
    display.clear_dispaly();

    let keybaord = Keyboard::new();

    let mut chip8 = CHIP8::create(cpu, memory, display, keybaord);

    while let Ok(()) = chip8.execute(DebugLevels {
        log_all: true,
        step: false,
    }) {}

    Ok(())
}
