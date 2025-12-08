pub mod cpu;
pub mod memory;

pub struct Chip8 {
    pub cpu: cpu::Cpu,
    pub memory: memory::Memory,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            cpu: cpu::Cpu::new(),
            memory: memory::Memory::new(),
        }
    }

    pub fn cycle(&mut self) {
        // Fetch, Decode, Execute cycle implementation goes here
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        Self::new()
    }
}

//===================================================
//                  UNIT TESTS
//===================================================

#[cfg(test)]
mod tests {
    use crate::chip8::memory::Memory;

    use super::*;
    use cpu::Cpu;

    #[test]
    fn cpu_new_initializes_registors() {
        let cpu = Cpu::new();
        assert_eq!(cpu.i, 0);
        assert_eq!(cpu.pc, 0);
        assert_eq!(cpu.v, [0; 16]);
    }

    #[test]
    fn memory_new_initializes_memory() {
        let memory = Memory::new();
        assert_eq!(memory.ram, [0; 4096])
    }

    #[test]
    fn chip8_new_initializes_cpu_and_memory() {
        let chip8 = Chip8::new();
        assert_eq!(chip8.cpu.pc, 0x200);
        assert_eq!(chip8.memory.ram, [0; 4096]);
    }
}
