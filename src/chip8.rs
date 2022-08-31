use sdl2::{event::Event, keyboard::Keycode};

use crate::{
    cpu::{OpCode, CPU},
    display::Display,
    memory::Memory,
};

pub struct CHIP8 {
    cpu: CPU,
    memory: Memory,
    display: Display,
}

impl CHIP8 {
    pub fn create(cpu: CPU, memory: Memory, display: Display) -> Self {
        Self {
            cpu,
            memory,
            display,
        }
    }

    pub fn get_cpu(&self) -> &CPU {
        &self.cpu
    }

    pub fn get_memory(&self) -> &Memory {
        &self.memory
    }

    pub fn execute(&mut self, debug: bool) -> Result<OpCode, &str> {
        if !self.cpu.pc_valid() {
            return Err("Counter Reached End Of Memory");
        }
        let instruction = self.cpu.fetch(&mut self.memory);
        let op_code = self.cpu.decode(instruction);

        'poll: loop {
            for event in self.display.events() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => return Err("Program Quit"),
                    Event::KeyDown {
                        keycode: Some(Keycode::W),
                        ..
                    } => break 'poll,
                    _ => {}
                }
            }
            if !debug {
                break;
            };
        }

        self.cpu
            .execute(&mut self.memory, &mut self.display, &op_code)
            .unwrap();
        self.display.present();

        Ok(op_code)
        // self.cpu.execute(&mut self.memory)
    }

    pub fn log_last(&mut self) {
        self.cpu.log_last(&mut self.memory);
    }
}
