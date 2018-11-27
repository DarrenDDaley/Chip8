use sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use CHIP8_WIDTH;
use CHIP8_HEIGHT;

const SCREEN_SCALE: u32 = 5;
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

    }

    fn colour(pixel: u8) -> pixels::Color {
        match value {
            0 => pixels::Color::RGB(0, 0, 0),
            _ => pixels::Color::RGB(255, 255, 255),
        }
    }
}