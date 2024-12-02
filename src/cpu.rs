use rand::Rng;
use std::io;
use std::io::Write;
use std::thread;
use std::time::{Duration, Instant};

#[warn(dead_code)]
pub struct CPU {
    pc: usize, // Program Counter: Points to the address of the next instruction to be executed in memory
    acc: u32,  // Accumulator: A register used for arithmetic and logical operations
    status: u8, // Status Register: Contains flags that indicate the state of the CPU or the result of operations
    memory: Vec<u8>, // Memory: A vector representing the main memory of the CPU, initialized with the specified size
    stack: Vec<u8>, // Stack: A vector used as a stack for function calls and returns, initialized with a value of 0
    current_event: u32, // Current Event: A value representing the current event or state of the CPU
}

impl CPU {
    // Constructor for the CPU
    pub fn new(memory_size: usize) -> CPU {
        CPU {
            pc: 0,
            acc: 0,
            status: 0,
            memory: vec![0; memory_size],
            stack: vec![0],
            current_event: 0, // Initialize current_event to 0
        }
    }

    // Method to update the current event
    pub fn update_event(&mut self, new_event: u32) {
        self.current_event = new_event;
        println!("Current event updated to: {}", self.current_event);
    }

    // Method to get the current event
    pub fn get_current_event(&self) -> i32 {
        self.current_event as i32
    }

    // Method to execute instructions
    pub fn execute(&mut self, instruction: u16) {
        match instruction {
            0x0100 => self.add(5),    // Example: ADD instruction
            0x0101 => self.load(10),  // Example: LOAD instruction
            0x0102 => self.store(20), // Example: STORE instruction
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
            0x0118 => self.push(34),
            0x0119 => self.pop(),
            0x0120 => self.call(69),
            // 0x0121 => self.ret(),
            0x0122 => self.modulu(32),
            // TODO: Fix everything, a lot implementations
            _ => println!("Unknown instruction"),
        }
    }

    // Example add operation
    fn add(&mut self, value: u32) {
        self.acc = self.acc.wrapping_add(value);
        println!("Accumulator: {}", self.acc);
    }

    // Example load operation
    fn load(&mut self, value: u32) {
        self.acc = value;
        println!("Accumulator loaded: {}", self.acc);
    }

    // Example store operation
    fn store(&mut self, address: usize) {
        if address < self.memory.len() {
            self.memory[address] = self.acc as u8;
            println!("Stored value {} in memory at address {}", self.acc, address);
        } else {
            println!("Invalid memory address: {}", address);
        }
    }

    fn sub(&mut self, value: u32) {
        self.acc = self.acc.wrapping_sub(value);
        println!("Accumulator: {}", self.acc);
    }

    fn mul(&mut self, value: u32) {
        self.acc = self.acc.wrapping_mul(value);
        println!("Accumulator: {}", self.acc);
    }

    fn div(&mut self, value: u32) {
        if value != 0 {
            self.acc /= value;
            println!("Accumulator: {}", self.acc);
        } else {
            println!("Error: Division by 0!");
        }
    }

    fn cmp(&mut self, value: u32) {
        self.status = if self.acc == value {
            0x01 // Equal
        } else if self.acc < value {
            0x02 // Less than
        } else {
            0x04 // Greater than
        };
        println!("Status Register: {}", self.status);
    }

    fn inc(&mut self) {
        self.acc = self.acc.wrapping_add(1);
        println!("Accumulator: {}", self.acc);
    }

    fn dec(&mut self) {
        self.acc = self.acc.wrapping_sub(1);
        println!("Accumulator: {}", self.acc);
    }

    fn jmp(&mut self, address: u32) {
        self.pc = address as usize;
        println!("Program Counter set to: {}", self.pc);
    }

    fn jz(&mut self, address: u32) {
        if self.acc == 0 {
            self.pc = address as usize;
            println!("Jumping to address: {}", self.pc);
        }
    }

    fn and(&mut self, value: u32) {
        self.acc &= value;
        println!("Accumulator: {}", self.acc);
    }

    fn or(&mut self, value: u32) {
        self.acc |= value;
        println!("Accumulator: {}", self.acc);
    }

    fn xor(&mut self, value: u32) {
        self.acc ^= value;
        println!("Accumulator: {}", self.acc);
    }

    fn shl(&mut self, value: u32) {
        self.acc <<= value;
        println!("Accumulator: {}", self.acc);
    }

    fn shr(&mut self, value: u32) {
        self.acc >>= value;
        println!("Accumulator: {}", self.acc);
    }

    fn ldi(&mut self, value: u32) {
        self.acc = value;
        println!("Accumulator loaded: {}", self.acc);
    }

    fn nop(&mut self) {
        // Do nothing and skip
        println!("No operation performed");
    }

    fn push(&mut self, value: u32) {
        self.stack.push(value as u8);
        println!("Pushed value: {}", value);
    }

    fn pop(&mut self) {
        if let Some(value) = self.stack.pop() {
            println!("Popped value: {}", value);
            Some(value);
        } else {
            println!("Stack is empty!");
        }
    }

    fn call(&mut self, address: u32) {
        self.stack.push(self.pc as u8); // Save the current PC to the stack
        self.pc = address as usize; // Jump to the new address
        println!("Calling address: {}", address);
    }

    fn modulu(&mut self, value: u32) {
        if value != 0 {
            self.acc %= value;
            println!("Accumulator: {}", self.acc);
        } else {
            println!("Error: Division by 0!");
        }
    }

    fn neg(&mut self, value: i32) {
        self.acc = -value as u32; // Negate the passed value, by transforming it to u32
        println!("Accumulator set to negated value: {}", self.acc);
    }

    fn set_flag(&mut self, value: u32) {
        self.status |= value as u8; // Set the specified flag
        println!("Status Register updated: {}", self.status);
    }

    fn clr_flag(&mut self, value: u32) {
        self.status &= !(value as u8); // Clear the specified flag
        println!("Status Register updated: {}", self.status);
    }

    fn test(&mut self, value: u32) {
        // Example test function (implementation needed)
        println!("Testing value: {}", value);
    }

    fn not(&mut self, value: u32) {
        self.acc = !self.acc; // Bitwise NOT operation
        println!("Accumulator after NOT: {}", self.acc);
    }

    fn rol(&mut self, value: u32) {
        // Rotate left (implementation needed)
        println!("Rotate left operation (not implemented)");
    }

    fn ror(&mut self, value: u32) {
        // Rotate right operation (implementation needed)
        println!("Rotate right operation (not implemented)");
    }

    fn input(&mut self, value: u32) -> u32 {
        let mut input = String::new();
        println!("Enter a value please (positive integer): ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<u32>() {
            Ok(parsed_value) => parsed_value, // Return the parsed value if successful
            Err(_) => {
                println!("Invalid input. Returning the default value: {}", value);
                value // Return the provided default value if parsing fails
            }
        }
    }

    fn output(&mut self, value: u32) {
        println!("Giving an output of: {}", value);
    }

    fn int(&mut self, value: u32) {
        // Interrupt handling (implementation needed)
        println!("Interrupt operation (not implemented)");
    }

    fn iret(&mut self, value: u32) {
        // Interrupt return operation (implementation needed)
        println!("Interrupt return operation (not implemented)");
    }

    fn wait(&mut self, event: u32, max_wait: Option<Duration>) {
        println!("Waiting for event: {}", event);

        let starttime = Instant::now();

        while self.current_event != event {
            if let Some(timeout) = max_wait {
                if starttime.elapsed() >= timeout {
                    println!("Timeout reached while waiting for event: {}", event);
                    return;
                }
            }

            thread::sleep(Duration::from_millis(100));
            println!("Current event: {}, still waiting...", self.current_event);
        }

        self.neg(event as i32);
    }

    fn halt(&mut self, value: u32) {
        // Halt the CPU (implementation needed)
        println!("Halt operation (not implemented)");
    }

    fn step(&mut self, value: u32) {
        // Step through the next instruction (implementation needed)
        println!("Step operation (not implemented)");
    }

    fn clr(&mut self, value: u32) {
        // Clear operation (implementation needed)
        println!("Clear operation (not implemented)");
    }

    fn rng(&mut self) {
        let mut rng = rand::thread_rng();
        let value: u32 = rng.gen_range(0..=u32::MAX);
        self.acc = value; // Store the random value in the accumulator
        println!("Random value generated: {}", self.acc);
    }
}
