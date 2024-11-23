pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Self { data: vec![0; 256] }
    }

    pub fn load(&mut self, start: u16, program: &[u8]) {
        self.data.splice(start as usize.., program.iter().cloned());
    }

    pub fn read(&mut self, address: u16) -> u8 {
        self.data[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }

    pub fn size(&mut self) -> usize {
        self.data.len()
    }
}
