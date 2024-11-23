use crate::memory;

pub struct CPU {
    pub registers: [u8; 4],
    pub pc: u16,
    pub memory: memory::Memory,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: [0; 4],
            pc: 0,
            memory: memory::Memory::new(),
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        self.memory.load(0, program);
    }

    pub fn fetch(&mut self) -> u8 {
        let instr = self.memory.read(self.pc);
        self.pc += 1;
        instr
    }

    pub fn run(&mut self) {
        while self.pc < self.memory.size() as u16 {
            let instr = self.fetch();
            if !self.execute(instr) {
                break;
            }
        }
    }
}