use crate::registers::Flag::{C, N, H, Z};
use crate::registers::Registers;
use crate::mmu::MMU;
use crate::consts::{PROG_START, MEMORY_SIZE};
use crate::executor;

pub struct Cpu {
    pub mmu: MMU,
    pub pc: u16,
    pub sp: u16,
    pub reg: Registers,
    pub ei: bool,
}

impl Cpu {
    pub fn new(mut mmu : MMU) -> Self {
        Self {
            mmu,
            pc: PROG_START,
            sp: 0,
            reg: Registers::new(),
            ei: false
        }
    }

    pub fn running(&self) -> bool {
        self.pc < MEMORY_SIZE
    }

    pub fn read_opcode(&mut self) -> Opcode {
        let data = self.mmu.read_u8(self.pc);
        self.reg.pc += 1;

        return data;
    }

    pub fn read_d(&mut self) -> i8 {
        let value = self.mmu.read_u8(self.pc);
        self.reg.pc += 1;

        return value as i8;
    }

    pub fn read_n(&mut self) -> u8 {
        let value = self.mmu.read_u8(self.pc);
        self.reg.pc += 1;

        return value;
    }

    pub fn read_nn(&mut self) -> u16 {
        let value = self.mmu.read_u16(self.pc);
        self.reg.pc += 2;;

        return value;
    }

    pub fn stack_push(&mut self, v: u16) {
        self.reg.sp -= 2;
        self.mmu.write_u16(self.reg.sp, v);
    }

    pub fn stack_pop(&mut self) -> u16 {
        let r = self.mmu.read_u16(self.reg.sp);
        self.reg.sp += 2;

        return r;
    }

    pub fn jr(&mut self, delta: i8) {
        self.pc = ((u32::from(self.reg.pc) as i32) + i32::from(delta)) as u16;
    }

    pub fn alu_dec(&mut self, a: u8) -> u8 {
        let tmp = a.wrapping_sub(1);

        self.reg.flag_set(H, a.trailing_zeros() >= 4);
        self.reg.flag_set(N, true);
        self.reg.flag_set(Z, tmp == 0);

        return tmp;
    }

    pub fn alu_dec16(&mut self, a: u16) -> u16 {
        let tmp = a.wrapping_sub(1);

        self.reg.flag_set(H, a.trailing_zeros() >= 4);
        self.reg.flag_set(N, true);
        self.reg.flag_set(Z, tmp == 0);

        return tmp;
    }

    pub fn alu_inc(&mut self, a: u8) -> u8 {
        let tmp = a.wrapping_add(1);

        self.reg.flag_set(H, (a & 0x0f) + 0x01 > 0x0f);
        self.reg.flag_set(N, false);
        self.reg.flag_set(Z, tmp == 0x00);

        return tmp;
    }

    pub fn alu_inc16(&mut self, a: u16) -> u16 {
        let tmp = a.wrapping_add(1);

        self.reg.flag_set(H, (a & 0x0f) + 0x01 > 0x0f);
        self.reg.flag_set(N, false);
        self.reg.flag_set(Z, tmp == 0x00);

        return tmp;
    }

    pub fn alu_add_u8(&mut self, a : u8, b : u8)  -> u8 {
        let tmp = a.wrapping_add(b);

        self.reg.flag_set(C, u16::from(a) + u16::from(b) > 0xff);
        self.reg.flag_set(H, ((a & 0x0F) + (b & 0x0F)) > 0x0F);
        self.reg.flag_set(Z, tmp == 0x00);
        self.reg.flag_set(N, false);

        return tmp as u8;
    }

    pub fn alu_add_u16(&mut self, a : u16, b : u16)  -> u16 {
        let tmp = a.wrapping_add(b);

        self.reg.flag_set(C, a > 0xFFFF - n);
        self.reg.flag_set(H, ((a & 0x0FFF) + (b & 0x0FFF)) > 0x0FFF);
        self.reg.flag_set(N, false);

        return tmp;
    }

    pub fn alu_sub(&mut self, a : u8, b : u8)  -> u8 {
        let tmp = a.wrapping_sub(b);

        self.reg.flag_set(C, u16::from(a) < u16::from(b));
        self.reg.flag_set(H, (a & 0x0f) < (b & 0x0f));
        self.reg.flag_set(Z, tmp == 0x00);
        self.reg.flag_set(N, false);

        return tmp as u8;
    }

    pub fn alu_adc(&mut self, a: u8, b: u8) -> u8 {
        let c = u8::from(self.c_flag);
        let tmp = a.wrapping_add(n).wrapping_add(c);

        self.reg.flag_set(C, u16::from(a) + u16::from(b) + u16::from(c) > 0xff);
        self.reg.flag_set(H, (a & 0x0f) + (n & 0x0f) + (c & 0x0f) > 0x0f);
        self.reg.flag_set(Z, tmp == 0x00);
        self.reg.flag_set(N, false);

        return tmp as u8;
    }

    pub fn alu_sbc(&mut self, a : u8, b: u8) -> u8 {
        let c = u8::from(self.c_flag);
        let r = a.wrapping_sub(b).wrapping_sub(c);

        self.reg.flag_set(C, u16::from(a) < u16::from(b) + u16::from(c));
        self.reg.flag_set(H, (a & 0x0f) < (b & 0x0f) + c);
        self.reg.flag_set(Z, tmp == 0x00);
        self.reg.flag_set(N, true);

        return tmp as u8;
    }

    pub fn alu_and(&mut self, a : u8, b: u8) -> u8 {
        let tmp = a & b;

        self.reg.flag_set(C, false);
        self.reg.flag_set(H, true);
        self.reg.flag_set(Z, tmp == 0x00);
        self.reg.flag_set(N, true);

        return tmp as u8;
    }

    pub fn alu_xor(&mut self, a: u8, b: u8) -> u8{
        let tmp = a ^ b;

        self.reg.flag_set(C, false);
        self.reg.flag_set(H, false);
        self.reg.flag_set(Z, tmp == 0x00);
        self.reg.flag_set(N, false);

        return tmp as u8;
    }

    pub fn alu_or(&mut self, a: u8, b: u8) -> u8 {
        let tmp = a | b;

        self.reg.flag_set(C, false);
        self.reg.flag_set(H, false);
        self.reg.flag_set(Z, tmp == 0x00);
        self.reg.flag_set(N, false);

        return tmp as u8;
    }

    pub fn alu_cp(&mut self, a: u8, b: u8) -> u8 {
        self.alu_sub(a, b);

        return a;
    }

    pub fn alu_rlc(&mut self, a: u8) -> u8 {
        let c = (a & 0b10000000) >> 7 == 0x01;
        let tmp = (a << 1) | u8::from(c);

        self.reg.flag_set(C, false);
        self.reg.flag_set(H, false);
        self.reg.flag_set(Z, tmp == 0x00);
        self.reg.flag_set(N, false);

        return tmp;
    }

    pub fn alu_rl(&mut self, a: u8) -> u8 {
        let c = (a & 0b10000000) >> 7 == 0x01;
        let tmp = (a << 1) | u8::from(self.c_flag);

        self.reg.flag_set(C, c);
        self.reg.flag_set(H, false);
        self.reg.flag_set(Z, tmp == 0x00);
        self.reg.flag_set(N, false);

        return tmp;
    }

    pub fn alu_rrc(&mut self, a: u8) -> u8 {
        let c =  a & 0x01 == 0x01;
        let tmp = 0b10000000 | (a >> 1);

        self.reg.flag_set(C, c);
        self.reg.flag_set(H, false);
        self.reg.flag_set(Z, tmp == 0x00);
        self.reg.flag_set(N, false);

        return tmp;
    }

    pub fn alu_rr(&mut self, a: u8) -> u8 {
        let c = (a & 0x01) >> 7 == 0x01;
        let tmp = (a >> 1) | (uu::from(self.c_flag) << 7);

        self.reg.flag_set(C, c);
        self.reg.flag_set(H, false);
        self.reg.flag_set(Z, tmp == 0x00);
        self.reg.flag_set(N, false);

        return tmp;
    }

    pub fn alu_sla(&mut self, a: u8) -> u8 {
        let c = (a & 0b10000000) >> 7 == 0x01;
        let tmp = a << 1;

        self.reg.flag_set(C, c);
        self.reg.flag_set(H, false);
        self.reg.flag_set(Z, tmp == 0x00);
        self.reg.flag_set(N, false);

        return tmp;
    }

    pub fn alu_sra(&mut self, a: u8) -> u8 {
        let c = a & 0x01 == 0x01;
        let tmp = (a >> 1) | (a & 0b10000000);

        self.reg.flag_set(C, c);
        self.reg.flag_set(H, false);
        self.reg.flag_set(Z, tmp == 0x00);
        self.reg.flag_set(N, false);

        return tmp;
    }

    pub fn alu_srl(&mut self, a: u8) -> u8 {
        let c = a & 0x01 == 0x01;
        let tmp = a >> 1;

        self.reg.flag_set(C, c);
        self.reg.flag_set(H, false);
        self.reg.flag_set(Z, tmp == 0x00);
        self.reg.flag_set(N, false);

        return tmp;
    }

    pub fn alu_swap(&mut self, a: u8) -> u8 {
        self.reg.flag_set(C, false);
        self.reg.flag_set(H, false);
        self.reg.flag_set(Z, a == 0x00);
        self.reg.flag_set(N, false);

        return (a >> 4) | (a << 4);
    }

    pub fn alu_bit(&mut self, a: u8, bit_num: u8) {
        let tmp = a & (1 << bit_num) == 0x00;

        self.reg.flag_set(H, false);
        self.reg.flag_set(Z, tmp);
        self.reg.flag_set(N, true);
    }

    pub fn alu_cpl(&mut self) {
        self.reg.a = !self.reg.a;

        self.reg.flag_set(H, true);
        self.reg.flag_set(N, true);
    }

    pub fn alu_ccf(&mut self) {
        let v = !self.reg.flag_get(C);

        self.reg.flag_set(C, v);
        self.reg.flag_set(H, false);
        self.reg.flag_set(N, false);
    }

    pub fn alu_scf(&mut self) {
        self.reg.flag_set(C, true);
        self.reg.flag_set(H, false);
        self.reg.flag_set(N, false);
    }

    pub fn alu_res(&mut self, a: u8, bit_num: u8) -> u8 {
        return a & !(1 << bit_num);
    }
    pub fn alu_set(&mut self, a: u8, bit_num: u8) -> u8 {
        return a | (1 << bit_num);
    }

    pub fn tic(&mut self) -> u8 {
        executor::exec_opcode(self)
    }
}
