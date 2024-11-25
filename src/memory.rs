// memory.rs

pub struct Memory {
    pub memory: Vec<u8>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Memory {
            memory: vec![0; size],
        }
    }

    pub fn read(&self, address: usize) -> u8 {
        self.memory[address]
    }

    pub fn write(&mut self, address: usize, value: u8) {
        self.memory[address] = value;
    }

    pub fn init_memory(&mut self, size: usize, default_value: u8) -> Vec<u8> {
        vec![default_value; size]
    }
    pub fn clear(&mut self, default_value: u8) {
        for byte in self.memory.iter_mut() {
            *byte = default_value;
        }
    }

    pub fn dump(&mut self) {
        for (i, byte) in self.memory.iter().enumerate() {
            println!("Address: {}: {}", i, byte);
        }
    }

    pub fn copy(&mut self, src: usize, dest: usize, length: usize) {
        for i in 0..length {
            self.memory[dest + i] = self.memory[src + i];
        }
    }

    fn compare(&mut self, addr1: usize, addr2: usize, length: usize) -> bool {
        self.memory[addr1..length] == self.memory[addr2..length] // should be 'addr1..addr1+length'
    }

    fn resize(&mut self, new_size: usize, default_value: u8) {
        self.memory.resize(new_size, default_value)
    }
}

// TODO: Speicherinitialiserung, Daten lesen, Daten schreiben, Speicher löschen, Speicher Dump, Speicherbereich kopieren, Speicher vergleichen, Blcokweises Laden, Speicherbereich schützen, Speichergröße anpassen, Speichersperren,