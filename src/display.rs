extern crate sdl2;

pub struct Display {
    screen_name: String,
    screen_width: u32,
    screen_height: u32,
    scale: u32,
}

impl Display {
    pub fn new(screen_name: String, screen_width: u32, screen_height: u32, scale: u32) -> Display {
        Display {
            screen_name,
            screen_width,
            screen_height,
            scale,
        }
    }

    pub fn initialize(&self) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window(&self.screen_name,
                                            self.screen_width * self.scale,
                                            self.screen_height * self.scale)
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