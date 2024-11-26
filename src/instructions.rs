// instructions.rs

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
pub const LDI: u16 = 0x0116; // load immediate
pub const NOP: u16 = 0x0117;

// implemented in cpu.rs until here

pub const PUSH: u16 = 0x0118; // Wert auf Stack festlegen
pub const POP: u16 = 0x0119; // Wert vom Stack nehmen
pub const CALL: u16 = 0x0120; // call function
pub const RET: u16 = 0x0121; // return of function
pub const MOD: u16 = 0x0122; // berechnet den Rest
pub const NEG: u16 = 0x0123; // negiert den Wert
pub const SET_FLAG: u16 = 0x0124; // Zero, Carry, Overflow
pub const CLR_FLAG: u16 = 0x0125; // removes flag
pub const TEST: u16 = 0x0126; // test if a specific bit is set in acc and set flag
pub const NOT: u16 = 0x0127; // negiert auch alle bits im acc
pub const ROL: u16 = 0x0128; //rotates all bits in the accumulators to the left side
pub const ROR: u16 = 0x0129; // rotates to the left side
pub const IN: u16 = 0x0130; // reads value from input device
pub const OUT: u16 = 0x0131; // gives value to output device
pub const INT: u16 = 0x0132; // interrupt
pub const IRET: u16 = 0x0133; // before interrupt
pub const HALT: u16 = 0x0134; // holds cpu
pub const WAIT: u16 = 0x0135; // idles and wait for something
pub const STEP: u16 = 0x0136; // executes one order and then waits
pub const CLR: u16 = 0x0137; // deletes memory
pub const RNG: u16 = 0x0138; // creates random value in acc