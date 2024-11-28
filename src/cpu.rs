// cpu.rs

use rand::Rng;
use std::io;
use std::io::Write;

#[warn(dead_code)]
pub struct CPU {
    pub pc: u32,         // Program Counter
    pub acc: u32,        // Accumulator (für arithmetische Operationen)
    pub status: u8,      // Status Register (einfach gehalten)
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
            0x0100 => self.add(5),    // Beispiel: ADD Instruktion
            0x0101 => self.load(10),  // Beispiel: LOAD Instruktion
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
            0x0118 => self.push(34),
            0x0119 => self.pop(2345),
            0x0120 => self.call(69),
            0x0121 => self.ret(435),
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

    fn push(&mut self, value: u32) {
        //
    }

    fn pop(&mut self, value: u32) {
        //
    }

    fn call(&mut self, value: u32) {
        //
    }

    fn ret(&mut self, value: u32) {
        //
    }

    fn modulu(&mut self, value: u32) {
        //
    }

    fn neg(&mut self, value: u32) {
        //
    }

    fn set_flag(&mut self, value: u32) {
        //
    }

    fn clr_flag(&mut self, value: u32) {
        //
    }

    fn test(&mut self, value: u32) {
        //
    }

    fn not(&mut self, value: u32) {
        //
    }

    fn rol(&mut self, value: u32) {
        //
    }

    fn ror(&mut self, value: u32) {
        //
    }

    fn input(&mut self, value: u32) -> u32 {
        let mut input = String::new();
        println!("Enter a value please (positive integer): ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<u32>() {
            Ok(parsed_value) => parsed_value, // Return the parsed value if successful
            Err(_) => {
                println!("Invalid input. Returning the default value: {}", value);
                value // Return the provided default value if parsing fails
            }
        } // TODO: Fixing
    }

    fn output(&mut self, value: u32) {
        println!("Giving an output of: {}", value)
    }

    fn int(&mut self, value: u32) {
        //
    }

    fn iret(&mut self, value: u32) {
        //
    }

    fn halt(&mut self, value: u32) {
        //
    }

    fn step(&mut self, value: u32) {
        //
    }

    fn clr(&mut self, value: u32) {
        //
    }

    fn rng(&mut self) {
        let mut rng = rand::thread_rng();
        let value: u32 = rng.gen_range(0..=u32::MAX);
        self.acc = value;
        println!("Zufälliger Wert generiert: {}", self.acc);
    }
}

