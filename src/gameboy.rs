use crate::meta;

struct Opcode {
    pub x: u8,
    pub y: u8,
    pub z: u8,
    pub p: u8,
    pub q: u8,
    pub raw: u8
}

impl Opcode {
    pub fn new(opcode : u8) -> Self {
        let x = opcode & 0b11000000;
        let y = opcode & 0b00111000;
        let z = opcode & 0b00000111;
        let p = opcode & 0b00110000;
        let q = opcode & 0b00001000;

        return Self { x, y, z, p, q, raw: opcode };
    }
}
