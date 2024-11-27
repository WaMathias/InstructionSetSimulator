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

    // Simuliere das Ausführen der Instruktionen
    cpu.execute(memory.read(0)); // Add
    cpu.execute(memory.read(1)); // Load
    cpu.execute(memory.read(2)); // Store
}
