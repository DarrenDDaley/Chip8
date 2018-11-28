use super::cpu;

use CHIP8_WIDTH;
use CHIP8_HEIGHT;


#[test]
fn opcode_test_00e0() {

    let mut cpu = cpu::CPU::new([0 as u8; 4096]);
    cpu.video_ram = [[128; CHIP8_WIDTH]; CHIP8_HEIGHT];
    cpu.opcode_execute(0x00E0);

    for y in 0..CHIP8_HEIGHT {
        for x in 0..CHIP8_WIDTH {
            assert_eq!(cpu.video_ram[y][x], 0);
        }
    }
}