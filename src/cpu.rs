// cpu.rs
pub struct CPU {
    pub pc: u32,  // Program Counter
    pub acc: u32, // Accumulator (für arithmetische Operationen)
    pub status: u8, // Status Register (einfach gehalten)
    pub memory: Vec<u8>, // Simulierter Speicher
}

impl CPU {
    // Konstruktor für die CPU
    pub fn new(memory_size: usize) -> CPU {
        CPU {
            pc: 0,
            acc: 0,
            status: 0,
            memory: vec![0; memory_size],
        }
    }

    // Methode zum Ausführen von Instruktionen
    pub fn execute(&mut self, instruction: u8) {
        match instruction {
            0x01 => self.add(5), // Beispiel: ADD Instruktion
            0x02 => self.load(10), // Beispiel: LOAD Instruktion
            0x03 => self.store(20), // Beispiel: STORE Instruktion
            _ => println!("Unbekannte Instruktion"),
        }
    }

    // Beispiel-Add-Operation
    fn add(&mut self, value: u32) {
        self.acc += value;
        println!("Accumulator: {}", self.acc);
    }

    // Beispiel-Laden einer Zahl in den Accumulator
    fn load(&mut self, value: u32) {
        self.acc = value;
        println!("Accumulator geladen: {}", self.acc);
    }

    // Beispiel-Speichern eines Wertes im Speicher
    fn store(&mut self, address: usize) {
        if address < self.memory.len() {
            self.memory[address] = self.acc as u8;
            println!("Wert {} im Speicher an Adresse {}", self.acc, address);
        } else {
            println!("Speicheradresse {} außerhalb des zulässigen Bereichs", address);
        }
    }

    fn sub(&mut self, value: u32) {
        self.acc -= value;
        println!("Accumulator geladen: {}", self.acc);
    }

    fn mul(&mut self, value: u32) {
        self.acc *= value;
        println!("Accumulator geladen: {}", self.acc);
    }

    fn div(&mut self, value: u32) {
        self.acc /= value;
        println!("Accumulator geladen: {}", self.acc);
    }
}
