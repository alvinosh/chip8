use sdl2::{
    event::EventPollIterator, pixels::Color, rect::Rect, render::Canvas, video::Window, EventPump,
};

const SIZE: (u32, u32) = (64, 32);
const RESOLUTION: u32 = 16;

const BG: Color = Color::RGB(200, 100, 150);
const FG: Color = Color::RGB(80, 80, 200);

pub struct Display {
    canvas: Canvas<Window>,
    event_pump: EventPump,

    buffer: [bool; SIZE.0 as usize * SIZE.1 as usize],

    key: Option<u8>,
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
        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        Self {
            canvas: canvas,
            event_pump: event_pump,
            key: None,
            buffer: [false; SIZE.0 as usize * SIZE.1 as usize],
        }
    }

    pub fn clear_dispaly(&mut self) {
        self.canvas.set_draw_color(BG);
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn dump_buffer(&mut self) {
        for y in 0..SIZE.1 {
            for x in 0..SIZE.0 {
                print!(
                    "{}",
                    if self.buffer[y as usize * SIZE.0 as usize + x as usize] {
                        "1"
                    } else {
                        "0"
                    }
                )
            }
            println!();
        }
    }

    pub fn draw_suqare(&mut self, x: u8, y: u8, color: bool) -> bool {
        let x = ((x - 1) as u32 % SIZE.0) as u8;
        let y = (y as u32 % SIZE.1) as u8;

        let over = self.buffer[x as usize + y as usize * SIZE.0 as usize] == color;

        // println!(
        //     "X: {} Y: {} | {} ^ {} = {}",
        //     x,
        //     y,
        //     self.buffer[(x + y * SIZE.0 as u8) as usize],
        //     color,
        //     self.buffer[(x + y * SIZE.0 as u8) as usize] ^ color
        // );

        self.buffer[x as usize + y as usize * SIZE.0 as usize] ^= color;

        self.canvas
            .set_draw_color(if self.buffer[x as usize + y as usize * SIZE.0 as usize] {
                FG
            } else {
                BG
            });
        self.canvas
            .fill_rect(Rect::new(
                x as i32 * RESOLUTION as i32,
                y as i32 * RESOLUTION as i32,
                RESOLUTION as u32,
                RESOLUTION as u32,
            ))
            .expect("Could Not Draw Rectangle");

        over
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn events(&mut self) -> EventPollIterator {
        self.event_pump.poll_iter()
    }
}
