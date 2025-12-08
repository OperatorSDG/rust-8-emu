use rust_8_emu::Chip8;

#[test]
fn chip8_new_initializes_cpu_and_memory() {
    let chip8 = Chip8::new();
    assert_eq!(chip8.cpu.pc, 0x200);
    assert_eq!(chip8.memory.ram, [0; 4096]);
}
