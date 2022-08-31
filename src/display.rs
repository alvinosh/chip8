use sdl2::{
    event::EventPollIterator, pixels::Color, rect::Rect, render::Canvas, video::Window, EventPump,
};

const SIZE: (u32, u32) = (64, 32);
const RESOLUTION: u32 = 16;

const BG: Color = Color::RGB(100, 100, 150);
const FG: Color = Color::RGB(80, 80, 200);

pub struct Display {
    canvas: Canvas<Window>,
    event_pump: EventPump,
}

impl Display {
    pub fn new(title: &str) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(title, SIZE.0 * RESOLUTION, SIZE.1 * RESOLUTION)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();

        Self {
            canvas: canvas,
            event_pump: event_pump,
        }
    }

    pub fn clear_dispaly(&mut self) {
        self.canvas.set_draw_color(BG);
        self.canvas.clear();
    }

    pub fn draw_suqare(&mut self, x: u8, y: u8) {
        self.canvas.set_draw_color(FG);
        self.canvas
            .fill_rect(Rect::new(
                x as i32 * RESOLUTION as i32,
                y as i32 * RESOLUTION as i32,
                RESOLUTION as u32,
                RESOLUTION as u32,
            ))
            .expect("Could Not Draw Rectangle");
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn events(&mut self) -> EventPollIterator {
        self.event_pump.poll_iter()
    }
}
