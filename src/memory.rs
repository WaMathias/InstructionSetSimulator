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
}

// TODO: Speicherinitialiserung, Daten lesen, Daten schreiben, Speicher löschen, Speicher Dump, Speicherbereich kopieren, Speicher vergleichen, Blcokweises Laden, Speicherbereich schützen, Speichergröße anpassen, Speichersperren,