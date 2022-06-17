
pub struct Registers {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16
}

pub enum Flag {
    C = 0b0001_0000,
    H = 0b0010_0000,
    N = 0b0100_0000,
    Z = 0b1000_0000,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0, f: 0,
            b: 0, c: 0,
            d: 0, e: 0,
            h: 0, l: 0,
            sp: 0,
            pc: 0x0100,
        }
    }

    pub fn flag_set(&mut self, flag : Flag, value : bool) {
        let mask = flag as u8;

        match value {
            true  => self.f |=  mask,
            false => self.f &= !mask,
        }

        self.f &= 0xF0;
    }

    pub fn flag_get(&self, flag : Flag) -> bool {
        let mask = flag as u8;

        return self.f & mask > 0;
    }

    pub fn af(&self) -> u16 { Self::join_to_u16(self.a, self.f) }
    pub fn bc(&self) -> u16 { Self::join_to_u16(self.b, self.c) }
    pub fn de(&self) -> u16 { Self::join_to_u16(self.d, self.e) }
    pub fn hl(&self) -> u16 { Self::join_to_u16(self.h, self.l) }

    pub fn set_af(&mut self, value : u16) {
        let (a, f) = Self::split_to_u8(value);
        self.a = a;
        self.f = f;
    }

    pub fn set_bc(&mut self, value : u16) {
        let (b, c) = Self::split_to_u8(value);
        self.b = b;
        self.c = c;
    }

    pub fn set_de(&mut self, value : u16) {
        let (b, e) = Self::split_to_u8(value);
        self.b = b;
        self.e = e;
    }

    pub fn set_hl(&mut self, value : u16) {
        let (h, l) = Self::split_to_u8(value);
        self.h = h;
        self.l = l;
    }

    fn join_to_u16(v1: u8, v2: u8) -> u16 {
        ((v1 as u16) << 8) | (v2 as u16)
    }

    fn split_to_u8(value: u16) -> (u8, u8) {
        (((value & 0xF0) >> 8 ) as u8, (value & 0x0F) as u8 )
    }
}
