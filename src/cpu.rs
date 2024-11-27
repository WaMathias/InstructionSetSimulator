// cpu.rs

#[warn(dead_code)]
pub struct CPU {
    pub pc: u32,  // Program Counter
    pub acc: u32, // Accumulator (für arithmetische Operationen)
    pub status: u8, // Status Register (einfach gehalten)
    pub memory: Vec<u8>, // Simulierter Speicher
    pub stack: Vec<u32>,
    // TODO: Satisfy Linter with implementing pc and status
}

impl CPU {
    // Konstruktor für die CPU
    pub fn new(memory_size: usize) -> CPU {
        CPU {
            pc: 0,
            acc: 0,
            status: 0,
            memory: vec![0; memory_size],
            stack: vec![0],
        }
    }

    // Methode zum Ausführen von Instruktionen
    pub fn execute(&mut self, instruction: u16) {
        match instruction {
            0x0100 => self.add(5), // Beispiel: ADD Instruktion
            0x0101 => self.load(10), // Beispiel: LOAD Instruktion
            0x0102 => self.store(20), // Beispiel: STORE Instruktion
            0x0103 => self.sub(10),
            0x0104 => self.mul(20),
            0x0105 => self.div(10),
            0x0106 => self.cmp(50),
            0x0107 => self.inc(),
            0x0108 => self.dec(),
            0x0109 => self.jmp(3),
            0x0110 => self.jz(3),
            0x0111 => self.and(3),
            0x0112 => self.or(32),
            0x0113 => self.xor(68),
            0x0114 => self.shl(654),
            0x0115 => self.shr(2345),
            0x0116 => self.ldi(234),
            0x0117 => self.nop(),
            0x0118 => self.push(),
            0x0119 => self.pop(),
            0x0120 => self.call(69),
            0x0121 => self.ret(),
            0x0122 => self.modulu(32),

            // TODO: Fix everything, a lot implementations

            _ => println!("Unbekannte Instruktion"),
        }
    }

    // Beispiel-Add-Operation
    fn add(&mut self, value: u32) {
        self.acc = self.acc.wrapping_add(value);
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
            println!("Speicheradresse {} ungültig!", address);
        }
    }

    fn sub(&mut self, value: u32) {
        self.acc = self.acc.wrapping_sub(value);
        println!("Accumulator: {}", self.acc);
    }

    fn mul(&mut self, value: u32) {
        self.acc = self.acc.wrapping_mul(value);
        println!("Accumulator: {}", self.acc);
    }

    fn div(&mut self, value: u32) {
        if value != 0 {
            self.acc /= value;
            println!("Accumulator: {}", self.acc);
        } else {
            println!("Fehler: Division durch 0!");
        }
    }

    fn cmp(&mut self, value: u32) {
        self.status = if self.acc == value {
            0x01
        } else if self.acc < value {
            0x02
        } else {
            0x04
        };
        println!("Status-Register: {}", self.status);
    }

    fn inc(&mut self) {
        self.acc = self.acc.wrapping_add(1);
        println!("Accumulator: {}", self.acc);
    }

    fn dec(&mut self) {
        self.acc = self.acc.wrapping_sub(1);
        println!("Accumulator: {}", self.acc);
    }

    fn jmp(&mut self, address: u32) {
        self.pc = address;
        println!("Program Counter gesetzt auf: {}", self.pc);
    }

    fn jz(&mut self, address: u32) {
        if self.acc == 0 {
            self.pc = address;
            println!("Jumping to address: {}", self.pc);
        }
    }

    fn and(&mut self, value: u32) {
        self.acc &= value;
        println!("Accumulator geladen: {}", self.acc);
    }

    fn or(&mut self, value: u32) {
        self.acc |= value;
        println!("Accumulator geladen: {}", self.acc);
    }

    fn xor(&mut self, value: u32) {
        self.acc ^= value;
    }

    fn shl(&mut self, value: u32) {
        self.acc <<= value;
        println!("Accumulator geladen: {}", self.acc);
    }

    fn shr(&mut self, value: u32) {
        self.acc >>= value;
        println!("Accumulator geladen: {}", self.acc);
    }

    fn ldi(&mut self, value: u32) {
        self.acc = value;
        println!("Accumulator geladen: {}", self.acc);
    }

    fn nop(&mut self) {
        // do nothing and skip
        println!("Nichts wird gemacht")
    }

    fn push(&mut self) {
        self.stack.push(self.acc)
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn call(&mut self, _return_address: i32) {
        self.push();
    }

    fn ret(&mut self) {
        self.pop();
    }

    fn modulu(&mut self, value: u32) { // equivalent to 'pub const MOD', just changed name, because inconvenience with the mod crate
        self.acc = self.acc % value
    }

    fn neg(&mut self) {
        // self.acc = (-self.acc as i32) as u32; /* FIX */
    }
}