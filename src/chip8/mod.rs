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
        // Fetch
        let pc = self.cpu.pc as usize;
        let opcode = (self.memory.ram[pc] as u16) << 8 | (self.memory.ram[pc + 1] as u16);

        self.cpu.pc += 2;

        // Decode + Execute
        self.execute(opcode);
    }

    fn execute(&mut self, opcode: u16) {
        match opcode & 0xF000 {
            0x0000 => {
                // CLS or RET
            }
            _ => {
                // unknown OPCODE
            }
        }
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
        assert_eq!(cpu.pc, 0x200);
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

    #[test]
    fn cycle_fetches_code_and_increments_pc() {
        let mut chip8 = Chip8::default();

        chip8.memory.ram[0x200] = 0x12;
        chip8.memory.ram[0x201] = 0x34;

        chip8.cycle();

        assert_eq!(chip8.cpu.pc, 0x202)
    }
}
