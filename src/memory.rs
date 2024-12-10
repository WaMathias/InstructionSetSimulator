// memory.rs

pub struct Memory {
    pub memory: Vec<u16>,
    pub data: Vec<u32>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Memory {
            memory: vec![0; size],
            data: vec![0; size],
        }
    }

    pub fn read(&self, address: usize) -> u16 {
        self.memory[address]
    }

    pub fn write(&mut self, address: usize, value: u16) {
        self.memory[address] = value;
    }

    pub fn init_memory(&mut self, size: usize, default_value: u16) -> Vec<u16> {
        vec![default_value; size]
    }
    pub fn clear(&mut self, default_value: u16) {
        for byte in self.memory.iter_mut() {
            *byte = default_value;
        }
    }

    pub fn dump(&mut self) {
        for (i, byte) in self.memory.iter().enumerate() {
            println!("Address: {}: {}", i, byte);
        }
    }

    pub fn load_program(&mut self, slice: &[u16]) {
        if slice.len() > self.memory.len() {
            panic!("Slcie is too large for the memory size");
        }
        self.memory[..slice.len()].copy_from_slice(slice);
    }

    pub fn copy(&mut self, src: usize, dest: usize, length: usize) {
        for i in 0..length {
            self.memory[dest + i] = self.memory[src + i];
        }
    }

    fn compare(&mut self, addr1: usize, addr2: usize, length: usize) -> bool {
        self.memory[addr1..length] == self.memory[addr2..length] // should be 'addr1..addr1+length'
    }

    fn resize(&mut self, new_size: usize, default_value: u16) {
        self.memory.resize(new_size, default_value)
    }

    fn protect(&mut self, start_addr: usize, end_addr: usize) {
        self.protect(start_addr, end_addr)
    }

    fn lock(&mut self, start_addr: usize, end_addr: usize) {
        self.lock(start_addr, end_addr) // TODO: satisfy compiler with loop in short future
    }
}
