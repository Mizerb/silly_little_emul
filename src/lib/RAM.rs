pub struct GBRAM {
    ram: [u8; 0x8000]
}

impl GBRAM {
    pub fn new() -> GBRAM {
        GBRAM {
            ram: [0; 0x8000]
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.ram[address as usize] = value;
    }
}