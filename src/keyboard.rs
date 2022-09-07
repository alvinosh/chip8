use sdl2::{
    event::{Event, EventPollIterator},
    keyboard::Keycode,
};

use crate::display::Display;

#[derive(PartialEq, Copy, Clone)]
pub enum KeyBoardEvent {
    Quit,
    Next,
    KeyPressed(u8),
}

pub struct Keyboard {
    key: Option<KeyBoardEvent>,
}

impl Keyboard {
    pub fn new() -> Self {
        Self { key: None }
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        if let Some(KeyBoardEvent::KeyPressed(k)) = self.key {
            return k == key;
        } else {
            return false;
        }
    }

    pub fn clear_key(&mut self) {
        self.key = None;
    }

    pub fn map_code(&mut self, code: Keycode) -> Option<KeyBoardEvent> {
        self.key = match code {
            Keycode::W => Some(KeyBoardEvent::Next),
            Keycode::Num0 => Some(KeyBoardEvent::KeyPressed(0)),
            Keycode::Num1 => Some(KeyBoardEvent::KeyPressed(1)),
            Keycode::Num2 => Some(KeyBoardEvent::KeyPressed(2)),
            Keycode::Num3 => Some(KeyBoardEvent::KeyPressed(3)),
            Keycode::Num4 => Some(KeyBoardEvent::KeyPressed(4)),
            Keycode::Num5 => Some(KeyBoardEvent::KeyPressed(5)),
            Keycode::Num6 => Some(KeyBoardEvent::KeyPressed(6)),
            Keycode::Num7 => Some(KeyBoardEvent::KeyPressed(7)),
            Keycode::Num8 => Some(KeyBoardEvent::KeyPressed(8)),
            Keycode::Num9 => Some(KeyBoardEvent::KeyPressed(9)),
            Keycode::A => Some(KeyBoardEvent::KeyPressed(10)),
            Keycode::B => Some(KeyBoardEvent::KeyPressed(11)),
            Keycode::C => Some(KeyBoardEvent::KeyPressed(12)),
            Keycode::D => Some(KeyBoardEvent::KeyPressed(13)),
            Keycode::E => Some(KeyBoardEvent::KeyPressed(14)),
            Keycode::F => Some(KeyBoardEvent::KeyPressed(15)),
            _ => None,
        };
        self.key
    }

    pub fn map_event(&mut self, event: Event) -> Option<KeyBoardEvent> {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => Some(KeyBoardEvent::Quit),
            Event::KeyDown {
                keycode: Some(Keycode::W),
                ..
            } => Some(KeyBoardEvent::Next),

            Event::KeyDown {
                keycode: Some(code),
                ..
            } => {
                if let Some(key) = self.map_code(code) {
                    Some(key)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn get_events(&mut self, events: EventPollIterator) -> Vec<KeyBoardEvent> {
        events.filter_map(|e| self.map_event(e)).collect()
    }

    pub fn wait_key(&mut self, display: &mut Display) -> KeyBoardEvent {
        loop {
            for event in display.events() {
                match self.map_event(event) {
                    Some(a) => {
                        return a;
                    }
                    _ => {
                        // print!("{:#?}", event);
                    }
                }
            }
        }
    }
}
