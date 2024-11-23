mod cpu;
mod memory;
mod instructions;

fn main() {
    let mut cpu = cpu::CPU::new();

    // Beispielprogramm: [Opcode, Argumente, ...]
    cpu.load_program(&[
        0x01, 0x00, 0x10, // LOAD R0, 0x10
        0x02, 0x01, 0x00, // ADD R1, R0
        0xFF,             // HALT
    ]);

    cpu.run();
}