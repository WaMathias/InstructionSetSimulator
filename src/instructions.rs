use crate::cpu::CPU;

pub fn decode_and_execute(cpu: &mut CPU, opcode: u8) -> bool {
    match opcode {
        0x01 => { // LOAD
            let reg = cpu.fetch() as usize;
            let addr = cpu.fetch();
            cpu.registers[reg] = cpu.memory.read(addr as u16);
        }
        0x02 => { // ADD
            let reg1 = cpu.fetch() as usize;
            let reg2 = cpu.fetch() as usize;
            cpu.registers[reg1] = cpu.registers[reg1].wrapping_add(cpu.registers[reg2]);
        }
        0xFF => return false, // HALT
        _ => panic!("Unbekannter Opcode: 0x{:02X}", opcode),
    }
    true
}
