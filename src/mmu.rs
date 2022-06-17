use crate::consts::{PROG_START, MEMORY_SIZE};

pub struct MMU {
    pub memory: [u8; 0xFFFF],
}

impl MMU {
    pub fn new() -> Self {
        Self {
            memory: [0; 0xFFFF]
        }
    }

    pub fn from_rom(rom: &Vec<u8>) -> Self {
        let mut obj = Self::new();
        obj.load_rom(rom);

        return obj;
    }

    pub fn load_rom(&mut self, rom: &Vec<u8>) {
        for (i, e) in rom.iter().enumerate() {
            if PROG_START + (i as u16) < MEMORY_SIZE {
                self.memory[PROG_START as usize + i] = *e;
            }
        }
    }

    pub fn read_u8(&self, from: u16) -> u8 {
        self.read(from)
    }

    pub fn read_u16(&self, from: u16) -> u16 {
        let value_fn = self.read(from);
        let value_sn = self.read(from + 1);

        ((value_sn as u16 ) << 8) | value_fn as u16
    }

    fn write_u16(&mut self, dest: u16, value: u16) {
        self.write_u8(dest, (value & 0xFF) as u8);
        self.write_u8(dest +1, ((value >> 8) & 0xFF) as u8);
    }

    fn write_u8(&mut self, dest: u16, value: u8) {
        self.memory[dest as usize] = value;
    }


    fn read(&self, dest: u16) -> u8 {
        self.memory[dest as usize]
    }
}
