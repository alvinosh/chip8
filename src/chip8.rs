use std::{
    rc::Rc,
    time::{Duration, Instant},
};

use rand::distributions::Bernoulli;
use sdl2::{event::Event, keyboard::Keycode};

use crate::{
    cpu::{OpCode, CPU},
    debug_levels::DebugLevels,
    display::{self, Display},
    keyboard::{self, KeyBoardEvent, Keyboard},
    memory::Memory,
};

pub struct CHIP8 {
    cpu: CPU,
    memory: Memory,
    display: Display,
    keyboard: Keyboard,
    deltatime: Instant,
}

impl CHIP8 {
    pub fn create(cpu: CPU, memory: Memory, display: Display, keyboard: Keyboard) -> Self {
        Self {
            cpu,
            memory,
            display,
            keyboard,
            deltatime: Instant::now(),
        }
    }

    #[allow(dead_code)]
    pub fn get_cpu(&self) -> &CPU {
        &self.cpu
    }

    #[allow(dead_code)]
    pub fn get_memory(&self) -> &Memory {
        &self.memory
    }

    pub fn execute(&mut self, debug_levels: DebugLevels) -> Result<(), &str> {
        if debug_levels.step {
            loop {
                match self.keyboard.wait_key(&mut self.display) {
                    KeyBoardEvent::Quit => return Err("Quit Application"),
                    KeyBoardEvent::Next => break,
                    KeyBoardEvent::KeyPressed(_) => {}
                }
            }
        }

        self.keyboard.clear_key();

        if !self.cpu.pc_valid() {
            return Err("Counter Reached End Of Memory");
        }
        let instruction = self.cpu.fetch(&mut self.memory);
        let op_code = self.cpu.decode(instruction);

        let event = self.keyboard.get_events(self.display.events());

        if event.contains(&KeyBoardEvent::Quit) {
            return Err("Program Quit");
        }

        if self.deltatime.elapsed().as_millis() > 107 {
            self.cpu.delay_timer();
            self.cpu.sound_timer();
            self.deltatime = Instant::now()
        }
        self.cpu
            .execute(
                &mut self.memory,
                &mut self.display,
                &mut self.keyboard,
                &op_code,
            )
            .unwrap();

        if debug_levels.log_all {
            self.cpu.log_last();
        }

        self.display.present();

        Ok(())
        // self.cpu.execute(&mut self.memory)
    }

    pub fn log_last(&mut self) {
        self.cpu.log_last();
    }
}
