pub struct Memory {
    pub ram: [u8; 4096],
}

impl Memory {
    pub fn new() -> Self {
        Self { ram: [0; 4096] }
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}
