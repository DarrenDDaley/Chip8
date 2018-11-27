extern crate sdl2;

use CHIP8_WIDTH;
use CHIP8_HEIGHT;

const SCREEN_SCALE: u32 = 5;
const SCREEN_WIDTH: u32 = (CHIP8_WIDTH as u32) * SCREEN_SCALE;
const SCREEN_HEIGHT: u32 = (CHIP8_HEIGHT as u32) * SCREEN_SCALE;

const PROGRAM_TITLE: &str = "Chip 8 Emulator";

pub struct Display {
}

impl Display {
    pub fn new() -> Display {
        Display {
        }
    }

    pub fn initialize(&self) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window(&self.screen_name,
                                            SCREEN_WIDTH * self.scale,
                                            SCREEN_HEIGHT * self.scale)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        let mut event_pump = sdl_context.event_pump().unwrap();
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }
            canvas.present();
        }
    }
}