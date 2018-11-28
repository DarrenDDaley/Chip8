use sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use std::ops;

use CHIP8_WIDTH;
use CHIP8_HEIGHT;

const SCREEN_SCALE: u32 = 20;
const SCREEN_WIDTH: u32 = (CHIP8_WIDTH as u32) * SCREEN_SCALE;
const SCREEN_HEIGHT: u32 = (CHIP8_HEIGHT as u32) * SCREEN_SCALE;

const PROGRAM_TITLE: &str = "Chip 8 Emulator";

pub struct Display {
    canvas: Canvas<Window>,
}

impl Display {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window(PROGRAM_TITLE,
                                                         SCREEN_WIDTH,
                                                        SCREEN_HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Display { canvas }
    }

    pub fn draw(&mut self, pixels: &[[u8; CHIP8_WIDTH]; CHIP8_HEIGHT]) {
        for y in 0..CHIP8_HEIGHT {
            for x in 0..CHIP8_WIDTH {

                self.canvas.set_draw_color(colour(pixels[y][x]));

                let x = (x as u32) * SCREEN_SCALE;
                let y = (y as u32) * SCREEN_SCALE;
                self.canvas.fill_rect(Rect::new(x as i32, y as i32, SCREEN_SCALE, SCREEN_SCALE));
            }
        }
        self.canvas.present();
    }
}

fn colour(pixel: u8) -> pixels::Color {
    match pixel {
        0 => pixels::Color::RGB(0, 0, 0),
        _ => pixels::Color::RGB(255, 255, 255),
    }
}