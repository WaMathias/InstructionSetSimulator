// main.rs
mod cpu;
mod instructions;
mod memory;

use cpu::CPU;
use instructions::*;
use memory::Memory;

fn main() {
    let mut cpu = CPU::new(256); // Erstelle eine CPU mit 256 Bytes Speicher
    let mut memory = Memory::new(256); // Erstelle den Speicher

    // Lade eine Instruktion in den Speicher
    memory.write(0, ADD);
    memory.write(1, LOAD);
    // cpu.rs

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
                println!(
                    "Speicheradresse {} außerhalb des zulässigen Bereichs",
                    address
                );
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

        fn cmp(&mut self, value: u32) {
            if self.acc == value {
                self.status = 0x01;
                println!("Accumulator ist gleich dem Wert: {}", self.acc);
            } else {
                self.status = 0x00;
                println!("Accumulator ist ungleich dem wert: {}", self.acc);
            }
        }

        fn inc(&mut self) {
            self.acc = self.acc + 1;
            println!("Accumulator geladen: {}", self.acc);
        }

        fn dec(&mut self) {
            self.acc = self.acc - 1;
            println!("Accumulator geladen: {}", self.acc);
        }

        fn jmp(&mut self, address: usize) {
            // undefined value of address
            self.pc = address as u32; // redefinition of address
        }

        fn jz(&mut self, address: usize) {
            if self.acc == 0 {
                self.pc = address as u32;
                println!("Wert {} im Speicher an Adresse {}", self.acc, address);
            } else {
                println!("Not Zero, continuing...")
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
            println!("Nicht wird gemacht")
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

        fn modulu(&mut self, value: u32) {
            // equivalent to 'pub const MOD', just changed name, because inconvenience with the mod crate
            self.acc = self.acc % value
        }

        fn neg(&mut self) {
            // self.acc = (-self.acc as i32) as u32; /* FIX */
        }
    }

    // Simuliere das Ausführen der Instruktionen
    cpu.execute(memory.read(0)); // Add
    cpu.execute(memory.read(1)); // Load
    cpu.execute(memory.read(2)); // Store
}
