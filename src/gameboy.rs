use crate::meta;

const PROG_START: u16 = 0;
const MEMORY_SIZE: u16 = 0xFFFF;

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

    pub fn read_i8(&self, from: u16) -> i8 {
        let value = self.read(from) as u8;
        let body = (value | 0b01111111_u8) as i8;

        let result =
            if (value | 0b1000000) > 0 {
                body * -1
            } else {
                body
            };

        return result;
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

pub struct Emulator {
    pub mmu: MMU,
    pub pc: u16,
    pub sp: u16,
    pub registers: Registers,

    z_flag: bool,
    h_flag: bool,
    n_flag: bool,
    c_flag: bool,
}

struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0,
            b: 0, c: 0,
            d: 0, e: 0,
            h: 0, l: 0,
        }
    }

    pub fn bc(&self) -> u16 { Self::join_to_u16(self.b, self.c) }
    pub fn de(&self) -> u16 { Self::join_to_u16(self.d, self.e) }
    pub fn hl(&self) -> u16 { Self::join_to_u16(self.h, self.l) }

    pub fn set_a(&mut self, value : u8) { self.a = value; }
    pub fn set_b(&mut self, value : u8) { self.b = value; }
    pub fn set_c(&mut self, value : u8) { self.c = value; }
    pub fn set_d(&mut self, value : u8) { self.d = value; }
    pub fn set_e(&mut self, value : u8) { self.e = value; }
    pub fn set_h(&mut self, value : u8) { self.h = value; }
    pub fn set_l(&mut self, value : u8) { self.l = value; }

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

impl Emulator {
    pub fn new(mut mmu : MMU) -> Self {
        Self {
            mmu,
            pc: PROG_START,
            sp: 0,
            registers: Registers::new(),

            z_flag: false,
            h_flag: false,
            n_flag: false,
            c_flag: false,
        }
    }

    pub fn running(&self) -> bool {
        self.pc < MEMORY_SIZE
    }

    // pub fn run(&mut self) {
    //     while self.running() {
    //         let opcode = self.read_opcode();
    //         self.step(opcode);
    //
    //         // match opcode.x {
    //         //     0 => match opcode.z {
    //         //         0 => match opcode.y {
    //         //             0 => self.nop(),
    //         //             1 => {
    //         //                 // LD
    //         //                 self.mmu.write_u16(self.read_nn(), self.sp);
    //         //             },
    //         //             2 => self.stop(),
    //         //             3 => self.jr(self.read_d()),
    //         //             4..7 => self.jr2(self.read_d(), opcode.y - 4),
    //         //             _ => 123,
    //         //         },
    //         //         1 => match opcode.q {
    //         //             0 => {
    //         //                 // LD
    //         //
    //         //                 self.ld(self.read_rp(opcode.p), self.read_nn())
    //         //             },
    //         //             1 => 222,
    //         //             _ => 123
    //         //         },
    //         //         _ => 123
    //         //     },
    //         //     _ =>  println!("PUK"),
    //         // }
    //     }
    // }

    fn read_opcode(&mut self) -> Opcode {
        let data = self.mmu.read(self.pc);
        self.inc_pc_by(1);
        return Opcode.new(data);
    }

    fn read_d(&mut self) -> i8 {
        let value = self.mmu.read_i8(self.pc);
        self.inc_pc_by(1);
        return value;
    }

    pub fn read_n(&mut self) -> u8 {
        let value = self.mmu.read_u8(self.pc);
        self.inc_pc_by(1);
        return value;
    }

    pub fn read_nn(&mut self) -> u16 {
        let value = self.mmu.read_u16(self.pc);
        self.inc_pc_by(2);
        return value;
    }

    pub fn inc_pc_by(&mut self, bytes: u8) {
        self.pc = self.pc + bytes as u16;
    }

    fn add16(&mut self, a : u16, b : u16)  -> u16 {
        let tmp = a as u32 + b as u32;

        self.n_flag = false;
        self.c_flag = tmp > 0xFFFF;
        self.h_flag = ((a & 0x0FFF) + (b & 0x0FFF)) > 0x0FFF;

        return tmp as u16;
    }

    fn step(&mut self, opcode : Opcode) {
        match opcode.raw {
            // NOP
            0x00 => {},
            // LD BC,d16
            0x01 => {
                println!("")
                self.registers.set_bc(self.read_nn())
            },
            // LD (BC),A
            0x02 => self.mmu.write_u8(self.registers.bc(), self.registers.a),
            // INC BC
            0x03 => self.registers.set_bc(self.registers.bc() + 1),
            // INC B
            0x04 => self.registers.set_b(self.registers.b + 1),
            // DEC B
            0x05 => self.registers.set_b(self.registers.b - 1),
            // LD B,d8
            0x06 => self.registers.set_b(self.read_n()),
            // RLCA
            0x07 => {}, // TODO
            // LD (a16),SP
            0x08 => self.mmu.write_u16(self.read_nn(), self.sp),
            // ADD HL,BC
            0x09 => {
                let tmp = self.add16(self.registers.hl(), self.registers.bc());
                self.registers.set_hl(tmp);
            },
            // LD A,(BC)
            0x0A => {
                let tmp = self.mmu.read_u8(self.registers.bc());
                self.registers.set_a(tmp);
            },
            // DEC BC
            0x0B => self.registers.set_bc(self.registers.bc() - 1),

            _ => 123
        }
    }
}
