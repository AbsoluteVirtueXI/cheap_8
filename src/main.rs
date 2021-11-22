use cheap_8::CHIP8;
fn main() {
    let mut chip8 = CHIP8::new();
    chip8.registers[0] = 5;
    chip8.registers[1] = 10;
    chip8.registers[2] = 10;
    chip8.registers[3] = 10;

    chip8.memory[0] = 0x80;
    chip8.memory[1] = 0x14;
    chip8.memory[2] = 0x80;
    chip8.memory[3] = 0x24;
    chip8.memory[4] = 0x80;
    chip8.memory[5] = 0x34;
    chip8.memory[6] = 0x21;
    chip8.memory[7] = 0x00;
    chip8.memory[8] = 0x21;
    chip8.memory[9] = 0x00;
    chip8.memory[10] = 0x00;
    chip8.memory[11] = 0x00;
    // function add twice
    chip8.memory[0x100] = 0x80;
    chip8.memory[0x101] = 0x14;
    chip8.memory[0x102] = 0x80;
    chip8.memory[0x103] = 0x14;
    chip8.memory[0x104] = 0x00;
    chip8.memory[0x105] = 0xEE;

    chip8.run();
    assert_eq!(chip8.registers[0], 75);
    println!(
        "5 + 10 + 10 + 10 + (10 * 2) + (10 * 2): {}",
        chip8.registers[0]
    );
}
