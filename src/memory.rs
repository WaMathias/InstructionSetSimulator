
pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Self { data: vec![0; 256] } // Maximal 256 Byte
    }

    pub fn load(&mut self, start: u16, program: &[u8]) {
        // Sicherheitsüberprüfung: Wir stellen sicher, dass das Programm nicht über das Speicherlimit hinaus geht
        if start as usize + program.len() > self.data.len() {
            panic!("Das Programm überschreitet die Speichergrenze");
        }

        self.data.splice(start as usize.., program.iter().cloned());
    }

    pub fn read(&mut self, address: u16) -> u8 {
        if address as usize >= self.data.len() {
            panic!("Speicherzugriff außerhalb des zulässigen Bereichs: Adresse 0x{:X}", address);
        }
        self.data[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        if address as usize >= self.data.len() {
            panic!("Speicherzugriff außerhalb des zulässigen Bereichs: Adresse 0x{:X}", address);
        }
        self.data[address as usize] = value;
    }

    pub fn size(&mut self) -> usize {
        self.data.len()
    }
}
