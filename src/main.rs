mod display;

pub fn main() {
    let display = display::Display::new("Chip 8 Emulator".to_string(),
                                                800,
                                                600,
                                                1);
    display.initialize();
}
