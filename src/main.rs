// main.rs
mod cpu;
mod instructions;
mod memory;

use cpu::CPU;
use instructions::*;
use memory::Memory;

use std::io;

fn main() {
    let mut cpu = CPU::new(256); // Erstelle eine CPU mit 256 Bytes Speicher
    let mut memory = Memory::new(256); // Erstelle den Speicher

    // Lade eine Instruktion in den Speicher
    memory.write(0, ADD);
    memory.write(1, LOAD);
    memory.write(3, STORE);
    memory.write(4, SUB);
    memory.write(5, MUL);
    memory.write(6, DIV);
    memory.write(7, CMP);
    memory.write(8, INC);
    memory.write(9, DEC);
    memory.write(10, JMP);
    memory.write(11, JZ);
    memory.write(12, AND);
    memory.write(13, OR);
    memory.write(14, XOR);
    memory.write(15, SHL);
    memory.write(16, SHR);
    memory.write(17, LDI);
    memory.write(18, NOP);
    memory.write(19, PUSH);
    memory.write(20, POP);
    memory.write(21, CALL);
    memory.write(22, RET);
    memory.write(23, MOD);
    memory.write(24, NEG);
    memory.write(25, SET_FLAG);
    memory.write(26, CLR_FLAG);
    memory.write(27, TEST);
    memory.write(28, NOT);
    memory.write(29, ROL);
    memory.write(30, ROR);
    memory.write(31, IN);
    memory.write(32, OUT);
    memory.write(33, INT);
    memory.write(34, IRET);
    memory.write(35, HALT);
    memory.write(36, WAIT);
    memory.write(37, STEP);
    memory.write(38, CLR);
    memory.write(39, RNG);

    // Simuliere das Ausführen der Instruktionen
    cpu.execute(memory.read(0)); // Add
    cpu.execute(memory.read(1)); // Load
    cpu.execute(memory.read(2)); // Store

    run_menu(&mut cpu, &mut memory);
}

fn run_menu(cpu: &mut CPU, memory: &mut Memory) {
    loop {
        println!("\n*** Instrcution Set Simulator ***");
        println!("1. show memory");
        println!("2. load program");
        println!("3. Start execution");
        println!("4. End");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Fehler beim Einlesen");

        match input.trim() {
            "1" => {
                println!("Memory content");
                memory.dump();
            }

            "2" => {
                println!("Load Program");
                let program = vec![0x1234, 0x5678];
                memory.load_program(&program); // TODO: implement that function
                println!("Loaded program");
            }

            "3" => {
                println!("Start execution...");
                cpu.run(memory); // TODO: implement that
            }

            "4" => {
                println!("Quit program");
                break;
            }
            _ => println!("Invalid option!, please try again"),
        }
    }
}
