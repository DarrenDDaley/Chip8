use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Input {
    events: sdl2::EventPump,
}

impl Input {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        Input { events: sdl_context.event_pump().unwrap() }
    }

    pub fn poll(&mut self) -> Result<[bool; 16], ()> {

        for event in self.events.poll_iter() {
            if let Event::Quit { .. } = event {
                return Err(());
            };
        }

        let keys: Vec<Keycode> = self.events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        let mut chip8_keys = [false; 16];

        for key in keys {
            match key {
                Keycode::Num1 => Some(chip8_keys[0x1] = true),
                Keycode::Num2 => Some(chip8_keys[0x2] = true),
                Keycode::Num3 => Some(chip8_keys[0x3] = true),
                Keycode::Num4 => Some(chip8_keys[0xc] = true),
                Keycode::Q => Some(chip8_keys[0x4] = true),
                Keycode::W => Some(chip8_keys[0x5] = true),
                Keycode::E => Some(chip8_keys[0x6] = true),
                Keycode::R => Some(chip8_keys[0xd] = true),
                Keycode::A => Some(chip8_keys[0x7] = true),
                Keycode::S => Some(chip8_keys[0x8] = true),
                Keycode::D => Some(chip8_keys[0x9] = true),
                Keycode::F => Some(chip8_keys[0xe] = true),
                Keycode::Z => Some(chip8_keys[0xa] = true),
                Keycode::X => Some(chip8_keys[0x0] = true),
                Keycode::C => Some(chip8_keys[0xb] = true),
                Keycode::V => Some(chip8_keys[0xf] = true),
                _ => None,
            };
        }

        Ok(chip8_keys)
    }
}