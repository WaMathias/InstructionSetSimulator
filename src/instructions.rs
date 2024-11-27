// ADD: Adds two values and stores the result in the accumulator.
pub const ADD: u16 = 0x0100;

// LOAD: Loads a value from memory into the accumulator.
pub const LOAD: u16 = 0x0101;

// STORE: Stores the value in the accumulator into memory.
pub const STORE: u16 = 0x0102;

// SUB: Subtracts a value from the accumulator.
pub const SUB: u16 = 0x0103;

// MUL: Multiplies the value in the accumulator by another value.
pub const MUL: u16 = 0x0104;

// DIV: Divides the value in the accumulator by another value.
pub const DIV: u16 = 0x0105;

// CMP: Compares the value in the accumulator with another value and sets the appropriate flags.
pub const CMP: u16 = 0x0106;

// INC: Increments the value in the accumulator by one.
pub const INC: u16 = 0x0107;

// DEC: Decrements the value in the accumulator by one.
pub const DEC: u16 = 0x0108;

// JMP: Jumps to a specified address in the program.
pub const JMP: u16 = 0x0109;

// JZ: Jumps to a specified address if the zero flag is set.
pub const JZ: u16 = 0x0110; // Jump if Zero Operation

// AND: Performs a bitwise AND operation with the accumulator.
pub const AND: u16 = 0x0111;

// OR: Performs a bitwise OR operation with the accumulator.
pub const OR: u16 = 0x0112;

// XOR: Performs a bitwise XOR operation with the accumulator.
pub const XOR: u16 = 0x0113;

// SHL: Shifts the bits in the accumulator to the left.
pub const SHL: u16 = 0x0114; // moves bits of accumulators to the left side

// SHR: Shifts the bits in the accumulator to the right.
pub const SHR: u16 = 0x0115;

// LDI: Loads an immediate value into the accumulator.
pub const LDI: u16 = 0x0116; // load immediate

// NOP: Performs no operation (No Operation).
pub const NOP: u16 = 0x0117;

// PUSH: Pushes a value onto the stack.
pub const PUSH: u16 = 0x0118; // Value to stack

// POP: Pops a value from the stack.
pub const POP: u16 = 0x0119; // Value from stack

// CALL: Calls a function.
pub const CALL: u16 = 0x0120; // call function

// RET: Returns control to the calling function.
pub const RET: u16 = 0x0121; // return of function

// MOD: Calculates the remainder of a division.
pub const MOD: u16 = 0x0122; // calculates the remainder

// NEG: Negates the value in the accumulator.
pub const NEG: u16 = 0x0123; // negates the value

// SET_FLAG: Sets a specific flag (e.g., Zero, Carry, Overflow).
pub const SET_FLAG: u16 = 0x0124; // Zero, Carry, Overflow

// CLR_FLAG: Clears a specific flag.
pub const CLR_FLAG: u16 = 0x0125; // removes flag

// TEST: Tests if a specific bit is set in the accumulator and sets the flag accordingly.
pub const TEST: u16 = 0x0126; // test if a specific bit is set in acc and set flag

// NOT: Negates all bits in the accumulator.
pub const NOT: u16 = 0x0127; // negates all bits in acc

// ROL: Rotates all bits in the accumulator to the left.
pub const ROL: u16 = 0x0128; // rotates all bits in the accumulators to the left side

// ROR: Rotates all bits in the accumulator to the right.
pub const ROR: u16 = 0x0129; // rotates to the right side

// IN: Reads a value from an input device.
pub const IN: u16 = 0x0130; // reads value from input device

// OUT: Outputs a value to an output device.
pub const OUT: u16 = 0x0131; // gives value to output device

// INT: Triggers an interrupt.
pub const INT: u16 = 0x0132; // interrupt

// IRET: Prepares to return from an interrupt.
pub const IRET: u16 = 0x0133; // before interrupt

// HALT: Halts the CPU.
pub const HALT: u16 =  0x0134; // holds cpu

// WAIT: Idles and waits for an event or condition.
pub const WAIT: u16 = 0x0135; // idles and wait for something

// STEP: Executes one instruction and then waits.
pub const STEP: u16 = 0x0136; // executes one order and then waits

// CLR: Clears memory or a specific area in memory.
pub const CLR: u16 = 0x0137; // deletes memory

// RNG: Generates a random value and stores it in the accumulator.
pub const RNG: u16 = 0x0138; // creates random value in acc