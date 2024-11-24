// instructions.rs

pub const ADD: u8 = 0x01;
pub const LOAD: u8 = 0x02;
pub const STORE: u8 = 0x03;
pub const SUB: u8 = 0x04;
pub const MUL: u8 = 0x05;
pub const DIV: u8 = 0x06;
pub const CMP: u8 = 0x07;
pub const INC: u8 = 0x08;
pub const DEC: u8 = 0x09;
pub const JMP: u8 = 0x0A;
pub const JZ: u8 = 0x0B; // Jump if Zero Operation
pub const AND: u8 = 0x0C;
pub const OR: u8 = 0x0D;
pub const XOR: u8 = 0x0E;
pub const SHL: u8 = 0x0F; // moves bits of accumulators to the left side
