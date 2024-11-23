use crate::cpu::CPU;

pub fn decodeAndExecute(cpu: &mut CPU, opcode: u8) -> bool {
    match opcode {
        0x01 => { // LOAD
            let register = "sfadsd";
        }
        _ => panic!("Unknown Op Code: 0x{:02x}", opcode)
    }
    true
}