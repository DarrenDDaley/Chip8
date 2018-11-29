use super::*;

use CHIP8_WIDTH;
use CHIP8_HEIGHT;


#[test]
fn opcode_test_clear_screen() {

    let mut cpu = CPU::new([0 as u8; 4096]);
    cpu.video_ram = [[128; CHIP8_WIDTH]; CHIP8_HEIGHT];
    cpu.opcode_clear_screen();

    for y in 0..CHIP8_HEIGHT {
        for x in 0..CHIP8_WIDTH {
            assert_eq!(cpu.video_ram[y][x], 0);
        }
    }
}