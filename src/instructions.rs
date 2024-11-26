// instructions.rs

use std::ops::Add;

pub const ADD: u16 = 0x0100;
pub const LOAD: u16 = 0x0101;
pub const STORE: u16 = 0x0102;
pub const SUB: u16 = 0x0103;
pub const MUL: u16 = 0x0104;
pub const DIV: u16 = 0x0105;
pub const CMP: u16 = 0x0106;
pub const INC: u16 = 0x0107;
pub const DEC: u16 = 0x0108;
pub const JMP: u16 = 0x0109;
pub const JZ: u16 = 0x0110; // Jump if Zero Operation
pub const AND: u16 = 0x0111;
pub const OR: u16 = 0x0112;
pub const XOR: u16 = 0x0113;
pub const SHL: u16 = 0x0114; // moves bits of accumulators to the left side
pub const SHR: u16 = 0x0115;
pub const LDI: u16 = 0x0116;
pub const NOP: u16 = 0x0117;