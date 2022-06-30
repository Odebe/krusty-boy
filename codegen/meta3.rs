use crate::cpu::Cpu;

pub fn exec_opcode(cpu: &mut Cpu) -> u8 {
    let opcode = cpu.read_opcode();

    match opcode {
        0x00 => {
            println!("NOP  ");

            4
        }

        0x01 => {
            println!("LD BC d16");

            let value = cpu.read_nn();
            cpu.reg.set_bc(value);

            12
        }

        0x02 => {
            println!("LD (BC) A");

            let addr = cpu.reg.bc();
            let value = cpu.reg.a;
            cpu.mmu.write_u8(addr, value);

            8
        }

        0x03 => {
            println!("INC BC ");

            let value = cpu.alu_dec(cpu.reg.bc());
            cpu.reg.set_bc(value);

            8
        }

        0x04 => {
            println!("INC B ");

            let value = cpu.alu_inc(cpu.reg.b);

            cpu.reg.b = value;

            4
        }

        0x05 => {
            println!("DEC B ");

            cpu.reg.b = cpu.alu_dec(cpu.reg.b);

            4
        }

        0x06 => {
            println!("LD B d8");

            let value = cpu.read_n();
            cpu.reg.b = value;

            8
        }

        0x07 => {
            println!("RLCA  ");

            cpu.reg.a = cpu.alu_rlc(cpu.reg.a);
            cpu.reg.flag_set(N, false);

            4
        }

        0x08 => {
            println!("LD (a16) SP");

            let addr = cpu.read_nn();
            let value = cpu.sp;
            cpu.mmu.write_u16(addr, value);

            20
        }

        0x09 => {
            println!("ADD HL BC");

            let value = cpu.alu_add_u8(cpu.reg.hl(), cpu.reg.bc());
            cpu.reg.set_hl(value);

            8
        }

        0x0a => {
            println!("LD A (BC)");

            let value = cpu.mmu.read_u16(cpu.reg.bc());
            cpu.reg.a = value;

            8
        }

        0x0b => {
            println!("DEC BC ");

            cpu.reg.set_bc(cpu.alu_dec(cpu.reg.bc()));

            8
        }

        0x0c => {
            println!("INC C ");

            let value = cpu.alu_inc(cpu.reg.c);

            cpu.reg.c = value;

            4
        }

        0x0d => {
            println!("DEC C ");

            cpu.reg.c = cpu.alu_dec(cpu.reg.c);

            4
        }

        0x0e => {
            println!("LD C d8");

            let value = cpu.read_n();
            cpu.reg.c = value;

            8
        }

        0x0f => {
            println!("RRCA  ");

            cpu.reg.a = cpu.alu_rrc(cpu.reg.a);
            cpu.reg.flag_set(N, false);

            4
        }

        0x10 => {
            println!("STOP 0 ");

            cpu.stop();

            4
        }

        0x11 => {
            println!("LD DE d16");

            let value = cpu.read_nn();
            cpu.reg.set_de(value);

            12
        }

        0x12 => {
            println!("LD (DE) A");

            let addr = cpu.reg.de();
            let value = cpu.reg.a;
            cpu.mmu.write_u8(addr, value);

            8
        }

        0x13 => {
            println!("INC DE ");

            let value = cpu.alu_dec(cpu.reg.de());
            cpu.reg.set_de(value);

            8
        }

        0x14 => {
            println!("INC D ");

            let value = cpu.alu_inc(cpu.reg.d);

            cpu.reg.d = value;

            4
        }

        0x15 => {
            println!("DEC D ");

            cpu.reg.d = cpu.alu_dec(cpu.reg.d);

            4
        }

        0x16 => {
            println!("LD D d8");

            let value = cpu.read_n();
            cpu.reg.d = value;

            8
        }

        0x17 => {
            println!("RLA  ");

            cpu.reg.a = cpu.alu_rl(cpu.reg.a);
            cpu.reg.flag_set(N, false);

            4
        }

        0x18 => {
            println!("JR r8 ");

            let delta = cpu.read_n() as i8;

            cpu.jr(delta);

            12
        }

        0x19 => {
            println!("ADD HL DE");

            let value = cpu.alu_add_u8(cpu.reg.hl(), cpu.reg.de());
            cpu.reg.set_hl(value);

            8
        }

        0x1a => {
            println!("LD A (DE)");

            let value = cpu.mmu.read_u16(cpu.reg.de());
            cpu.reg.a = value;

            8
        }

        0x1b => {
            println!("DEC DE ");

            cpu.reg.set_de(cpu.alu_dec(cpu.reg.de()));

            8
        }

        0x1c => {
            println!("INC E ");

            let value = cpu.alu_inc(cpu.reg.e);

            cpu.reg.e = value;

            4
        }

        0x1d => {
            println!("DEC E ");

            cpu.reg.e = cpu.alu_dec(cpu.reg.e);

            4
        }

        0x1e => {
            println!("LD E d8");

            let value = cpu.read_n();
            cpu.reg.e = value;

            8
        }

        0x1f => {
            println!("RRA  ");

            cpu.reg.a = cpu.alu_rr(cpu.reg.a);
            cpu.reg.flag_set(N, false);

            4
        }

        0x20 => {
            println!("JR NZ r8");

            let delta = cpu.read_n() as i8;

            if !cpu.reg.flag_get(Z) {
                cpu.jr(delta);
            }

            12
        }

        0x21 => {
            println!("LD HL d16");

            let value = cpu.read_nn();
            cpu.reg.set_hl(value);

            12
        }

        0x22 => {
            println!("LD (HL+) A");

            let value = cpu.reg.a;
            cpu.reg.hl = value;

            8
        }

        0x23 => {
            println!("INC HL ");

            let value = cpu.alu_dec(cpu.reg.hl());
            cpu.reg.set_hl(value);

            8
        }

        0x24 => {
            println!("INC H ");

            let value = cpu.alu_inc(cpu.reg.h);

            cpu.reg.h = value;

            4
        }

        0x25 => {
            println!("DEC H ");

            cpu.reg.h = cpu.alu_dec(cpu.reg.h);

            4
        }

        0x26 => {
            println!("LD H d8");

            let value = cpu.read_n();
            cpu.reg.h = value;

            8
        }

        0x27 => {
            println!("DAA  ");

            cpu.alu_daa();

            4
        }

        0x28 => {
            println!("JR Z r8");

            let delta = cpu.read_n() as i8;

            if cpu.reg.flag_get(Z) {
                cpu.jr(delta);
            }

            12
        }

        0x29 => {
            println!("ADD HL HL");

            let value = cpu.alu_add_u8(cpu.reg.hl(), cpu.reg.hl());
            cpu.reg.set_hl(value);

            8
        }

        0x2a => {
            println!("LD A (HL+)");

            let value = cpu.reg.hl();
            cpu.reg.a = value;

            8
        }

        0x2b => {
            println!("DEC HL ");

            cpu.reg.set_hl(cpu.alu_dec(cpu.reg.hl()));

            8
        }

        0x2c => {
            println!("INC L ");

            let value = cpu.alu_inc(cpu.reg.l);

            cpu.reg.l = value;

            4
        }

        0x2d => {
            println!("DEC L ");

            cpu.reg.l = cpu.alu_dec(cpu.reg.l);

            4
        }

        0x2e => {
            println!("LD L d8");

            let value = cpu.read_n();
            cpu.reg.l = value;

            8
        }

        0x2f => {
            println!("CPL  ");

            cpu.alu_cpl();

            4
        }

        0x30 => {
            println!("JR NC r8");

            let delta = cpu.read_n() as i8;

            if !cpu.reg.flag_get(C) {
                cpu.jr(delta);
            }

            12
        }

        0x31 => {
            println!("LD SP d16");

            let value = cpu.read_nn();
            cpu.sp = value;

            12
        }

        0x32 => {
            println!("LD (HL-) A");

            let value = cpu.reg.a;
            cpu.reg.hl = value;

            8
        }

        0x33 => {
            println!("INC SP ");

            let value = cpu.alu_dec(cpu.sp);
            cpu.sp = value;

            8
        }

        0x34 => {
            println!("INC (HL) ");

            let addr = cpu.reg.hl();
            let value = cpu.alu_inc(cpu.read_u8(addr));

            cpu.mmu.write_u8(addr, value);

            12
        }

        0x35 => {
            println!("DEC (HL) ");

            cpu.mmu
                .write_u8(addr, cpu.alu_inc(cpu.read_u8(cpu.reg.hl())));

            12
        }

        0x36 => {
            println!("LD (HL) d8");

            let addr = cpu.reg.hl();
            let value = cpu.read_n();
            cpu.mmu.write_u8(addr, value);

            12
        }

        0x37 => {
            println!("SCF  ");

            cpu.alu_scf();

            4
        }

        0x38 => {
            println!("JR C r8");

            let delta = cpu.read_n() as i8;

            if cpu.reg.flag_get(C) {
                cpu.jr(delta);
            }

            12
        }

        0x39 => {
            println!("ADD HL SP");

            let value = cpu.alu_add_u8(cpu.reg.hl(), cpu.sp);
            cpu.reg.set_hl(value);

            8
        }

        0x3a => {
            println!("LD A (HL-)");

            let value = cpu.reg.hl();
            cpu.reg.a = value;

            8
        }

        0x3b => {
            println!("DEC SP ");

            cpu.sp = cpu.alu_dec(cpu.sp);

            8
        }

        0x3c => {
            println!("INC A ");

            let value = cpu.alu_inc(cpu.reg.a);

            cpu.reg.a = value;

            4
        }

        0x3d => {
            println!("DEC A ");

            cpu.reg.a = cpu.alu_dec(cpu.reg.a);

            4
        }

        0x3e => {
            println!("LD A d8");

            let value = cpu.read_n();
            cpu.reg.a = value;

            8
        }

        0x3f => {
            println!("CCF  ");

            cpu.alu_ccf();

            4
        }

        0x40 => {
            println!("LD B B");

            let value = cpu.reg.b;
            cpu.reg.b = value;

            4
        }

        0x41 => {
            println!("LD B C");

            let value = cpu.reg.c;
            cpu.reg.b = value;

            4
        }

        0x42 => {
            println!("LD B D");

            let value = cpu.reg.d;
            cpu.reg.b = value;

            4
        }

        0x43 => {
            println!("LD B E");

            let value = cpu.reg.e;
            cpu.reg.b = value;

            4
        }

        0x44 => {
            println!("LD B H");

            let value = cpu.reg.h;
            cpu.reg.b = value;

            4
        }

        0x45 => {
            println!("LD B L");

            let value = cpu.reg.l;
            cpu.reg.b = value;

            4
        }

        0x46 => {
            println!("LD B (HL)");

            let value = cpu.mmu.read_u16(cpu.reg.hl());
            cpu.reg.b = value;

            8
        }

        0x47 => {
            println!("LD B A");

            let value = cpu.reg.a;
            cpu.reg.b = value;

            4
        }

        0x48 => {
            println!("LD C B");

            let value = cpu.reg.b;
            cpu.reg.c = value;

            4
        }

        0x49 => {
            println!("LD C C");

            let value = cpu.reg.c;
            cpu.reg.c = value;

            4
        }

        0x4a => {
            println!("LD C D");

            let value = cpu.reg.d;
            cpu.reg.c = value;

            4
        }

        0x4b => {
            println!("LD C E");

            let value = cpu.reg.e;
            cpu.reg.c = value;

            4
        }

        0x4c => {
            println!("LD C H");

            let value = cpu.reg.h;
            cpu.reg.c = value;

            4
        }

        0x4d => {
            println!("LD C L");

            let value = cpu.reg.l;
            cpu.reg.c = value;

            4
        }

        0x4e => {
            println!("LD C (HL)");

            let value = cpu.mmu.read_u16(cpu.reg.hl());
            cpu.reg.c = value;

            8
        }

        0x4f => {
            println!("LD C A");

            let value = cpu.reg.a;
            cpu.reg.c = value;

            4
        }

        0x50 => {
            println!("LD D B");

            let value = cpu.reg.b;
            cpu.reg.d = value;

            4
        }

        0x51 => {
            println!("LD D C");

            let value = cpu.reg.c;
            cpu.reg.d = value;

            4
        }

        0x52 => {
            println!("LD D D");

            let value = cpu.reg.d;
            cpu.reg.d = value;

            4
        }

        0x53 => {
            println!("LD D E");

            let value = cpu.reg.e;
            cpu.reg.d = value;

            4
        }

        0x54 => {
            println!("LD D H");

            let value = cpu.reg.h;
            cpu.reg.d = value;

            4
        }

        0x55 => {
            println!("LD D L");

            let value = cpu.reg.l;
            cpu.reg.d = value;

            4
        }

        0x56 => {
            println!("LD D (HL)");

            let value = cpu.mmu.read_u16(cpu.reg.hl());
            cpu.reg.d = value;

            8
        }

        0x57 => {
            println!("LD D A");

            let value = cpu.reg.a;
            cpu.reg.d = value;

            4
        }

        0x58 => {
            println!("LD E B");

            let value = cpu.reg.b;
            cpu.reg.e = value;

            4
        }

        0x59 => {
            println!("LD E C");

            let value = cpu.reg.c;
            cpu.reg.e = value;

            4
        }

        0x5a => {
            println!("LD E D");

            let value = cpu.reg.d;
            cpu.reg.e = value;

            4
        }

        0x5b => {
            println!("LD E E");

            let value = cpu.reg.e;
            cpu.reg.e = value;

            4
        }

        0x5c => {
            println!("LD E H");

            let value = cpu.reg.h;
            cpu.reg.e = value;

            4
        }

        0x5d => {
            println!("LD E L");

            let value = cpu.reg.l;
            cpu.reg.e = value;

            4
        }

        0x5e => {
            println!("LD E (HL)");

            let value = cpu.mmu.read_u16(cpu.reg.hl());
            cpu.reg.e = value;

            8
        }

        0x5f => {
            println!("LD E A");

            let value = cpu.reg.a;
            cpu.reg.e = value;

            4
        }

        0x60 => {
            println!("LD H B");

            let value = cpu.reg.b;
            cpu.reg.h = value;

            4
        }

        0x61 => {
            println!("LD H C");

            let value = cpu.reg.c;
            cpu.reg.h = value;

            4
        }

        0x62 => {
            println!("LD H D");

            let value = cpu.reg.d;
            cpu.reg.h = value;

            4
        }

        0x63 => {
            println!("LD H E");

            let value = cpu.reg.e;
            cpu.reg.h = value;

            4
        }

        0x64 => {
            println!("LD H H");

            let value = cpu.reg.h;
            cpu.reg.h = value;

            4
        }

        0x65 => {
            println!("LD H L");

            let value = cpu.reg.l;
            cpu.reg.h = value;

            4
        }

        0x66 => {
            println!("LD H (HL)");

            let value = cpu.mmu.read_u16(cpu.reg.hl());
            cpu.reg.h = value;

            8
        }

        0x67 => {
            println!("LD H A");

            let value = cpu.reg.a;
            cpu.reg.h = value;

            4
        }

        0x68 => {
            println!("LD L B");

            let value = cpu.reg.b;
            cpu.reg.l = value;

            4
        }

        0x69 => {
            println!("LD L C");

            let value = cpu.reg.c;
            cpu.reg.l = value;

            4
        }

        0x6a => {
            println!("LD L D");

            let value = cpu.reg.d;
            cpu.reg.l = value;

            4
        }

        0x6b => {
            println!("LD L E");

            let value = cpu.reg.e;
            cpu.reg.l = value;

            4
        }

        0x6c => {
            println!("LD L H");

            let value = cpu.reg.h;
            cpu.reg.l = value;

            4
        }

        0x6d => {
            println!("LD L L");

            let value = cpu.reg.l;
            cpu.reg.l = value;

            4
        }

        0x6e => {
            println!("LD L (HL)");

            let value = cpu.mmu.read_u16(cpu.reg.hl());
            cpu.reg.l = value;

            8
        }

        0x6f => {
            println!("LD L A");

            let value = cpu.reg.a;
            cpu.reg.l = value;

            4
        }

        0x70 => {
            println!("LD (HL) B");

            let addr = cpu.reg.hl();
            let value = cpu.reg.b;
            cpu.mmu.write_u8(addr, value);

            8
        }

        0x71 => {
            println!("LD (HL) C");

            let addr = cpu.reg.hl();
            let value = cpu.reg.c;
            cpu.mmu.write_u8(addr, value);

            8
        }

        0x72 => {
            println!("LD (HL) D");

            let addr = cpu.reg.hl();
            let value = cpu.reg.d;
            cpu.mmu.write_u8(addr, value);

            8
        }

        0x73 => {
            println!("LD (HL) E");

            let addr = cpu.reg.hl();
            let value = cpu.reg.e;
            cpu.mmu.write_u8(addr, value);

            8
        }

        0x74 => {
            println!("LD (HL) H");

            let addr = cpu.reg.hl();
            let value = cpu.reg.h;
            cpu.mmu.write_u8(addr, value);

            8
        }

        0x75 => {
            println!("LD (HL) L");

            let addr = cpu.reg.hl();
            let value = cpu.reg.l;
            cpu.mmu.write_u8(addr, value);

            8
        }

        0x76 => {
            println!("HALT  ");

            cpu.halt();

            4
        }

        0x77 => {
            println!("LD (HL) A");

            let addr = cpu.reg.hl();
            let value = cpu.reg.a;
            cpu.mmu.write_u8(addr, value);

            8
        }

        0x78 => {
            println!("LD A B");

            let value = cpu.reg.b;
            cpu.reg.a = value;

            4
        }

        0x79 => {
            println!("LD A C");

            let value = cpu.reg.c;
            cpu.reg.a = value;

            4
        }

        0x7a => {
            println!("LD A D");

            let value = cpu.reg.d;
            cpu.reg.a = value;

            4
        }

        0x7b => {
            println!("LD A E");

            let value = cpu.reg.e;
            cpu.reg.a = value;

            4
        }

        0x7c => {
            println!("LD A H");

            let value = cpu.reg.h;
            cpu.reg.a = value;

            4
        }

        0x7d => {
            println!("LD A L");

            let value = cpu.reg.l;
            cpu.reg.a = value;

            4
        }

        0x7e => {
            println!("LD A (HL)");

            let value = cpu.mmu.read_u16(cpu.reg.hl());
            cpu.reg.a = value;

            8
        }

        0x7f => {
            println!("LD A A");

            let value = cpu.reg.a;
            cpu.reg.a = value;

            4
        }

        0x80 => {
            println!("ADD A B");

            cpu.reg.a = cpu.alu_add_u8(cpu.reg.a, cpu.reg.b);

            4
        }

        0x81 => {
            println!("ADD A C");

            cpu.reg.a = cpu.alu_add_u8(cpu.reg.a, cpu.reg.c);

            4
        }

        0x82 => {
            println!("ADD A D");

            cpu.reg.a = cpu.alu_add_u8(cpu.reg.a, cpu.reg.d);

            4
        }

        0x83 => {
            println!("ADD A E");

            cpu.reg.a = cpu.alu_add_u8(cpu.reg.a, cpu.reg.e);

            4
        }

        0x84 => {
            println!("ADD A H");

            cpu.reg.a = cpu.alu_add_u8(cpu.reg.a, cpu.reg.h);

            4
        }

        0x85 => {
            println!("ADD A L");

            cpu.reg.a = cpu.alu_add_u8(cpu.reg.a, cpu.reg.l);

            4
        }

        0x86 => {
            println!("ADD A (HL)");

            cpu.reg.a = cpu.alu_add_u8(cpu.reg.a, cpu.mmu.read_u16(cpu.reg.hl()));

            8
        }

        0x87 => {
            println!("ADD A A");

            cpu.reg.a = cpu.alu_add_u8(cpu.reg.a, cpu.reg.a);

            4
        }

        0x88 => {
            println!("ADC A B");

            cpu.reg.a = cpu.alu_adc(cpu.reg.a, cpu.reg.b);

            4
        }

        0x89 => {
            println!("ADC A C");

            cpu.reg.a = cpu.alu_adc(cpu.reg.a, cpu.reg.c);

            4
        }

        0x8a => {
            println!("ADC A D");

            cpu.reg.a = cpu.alu_adc(cpu.reg.a, cpu.reg.d);

            4
        }

        0x8b => {
            println!("ADC A E");

            cpu.reg.a = cpu.alu_adc(cpu.reg.a, cpu.reg.e);

            4
        }

        0x8c => {
            println!("ADC A H");

            cpu.reg.a = cpu.alu_adc(cpu.reg.a, cpu.reg.h);

            4
        }

        0x8d => {
            println!("ADC A L");

            cpu.reg.a = cpu.alu_adc(cpu.reg.a, cpu.reg.l);

            4
        }

        0x8e => {
            println!("ADC A (HL)");

            cpu.reg.a = cpu.alu_adc(cpu.reg.a, cpu.mmu.read_u16(cpu.reg.hl()));

            8
        }

        0x8f => {
            println!("ADC A A");

            cpu.reg.a = cpu.alu_adc(cpu.reg.a, cpu.reg.a);

            4
        }

        0x90 => {
            println!("SUB B ");

            cpu.reg.a = cpu.alu_sub(cpu.reg.a, cpu.reg.b);

            4
        }

        0x91 => {
            println!("SUB C ");

            cpu.reg.a = cpu.alu_sub(cpu.reg.a, cpu.reg.c);

            4
        }

        0x92 => {
            println!("SUB D ");

            cpu.reg.a = cpu.alu_sub(cpu.reg.a, cpu.reg.d);

            4
        }

        0x93 => {
            println!("SUB E ");

            cpu.reg.a = cpu.alu_sub(cpu.reg.a, cpu.reg.e);

            4
        }

        0x94 => {
            println!("SUB H ");

            cpu.reg.a = cpu.alu_sub(cpu.reg.a, cpu.reg.h);

            4
        }

        0x95 => {
            println!("SUB L ");

            cpu.reg.a = cpu.alu_sub(cpu.reg.a, cpu.reg.l);

            4
        }

        0x96 => {
            println!("SUB (HL) ");

            cpu.reg.a = cpu.alu_sub(cpu.reg.a, cpu.mmu.read_u16(cpu.reg.hl()));

            8
        }

        0x97 => {
            println!("SUB A ");

            cpu.reg.a = cpu.alu_sub(cpu.reg.a, cpu.reg.a);

            4
        }

        0x98 => {
            println!("SBC A B");

            cpu.reg.a = cpu.alu_sub(cpu.reg.a);

            4
        }

        0x99 => {
            println!("SBC A C");

            cpu.reg.a = cpu.alu_sub(cpu.reg.a);

            4
        }

        0x9a => {
            println!("SBC A D");

            cpu.reg.a = cpu.alu_sub(cpu.reg.a);

            4
        }

        0x9b => {
            println!("SBC A E");

            cpu.reg.a = cpu.alu_sub(cpu.reg.a);

            4
        }

        0x9c => {
            println!("SBC A H");

            cpu.reg.a = cpu.alu_sub(cpu.reg.a);

            4
        }

        0x9d => {
            println!("SBC A L");

            cpu.reg.a = cpu.alu_sub(cpu.reg.a);

            4
        }

        0x9e => {
            println!("SBC A (HL)");

            cpu.reg.a = cpu.alu_sub(cpu.reg.a);

            8
        }

        0x9f => {
            println!("SBC A A");

            cpu.reg.a = cpu.alu_sub(cpu.reg.a);

            4
        }

        0xa0 => {
            println!("AND B ");

            cpu.a = cpu.alu_and(cpu.reg.a, cpu.reg.b);

            4
        }

        0xa1 => {
            println!("AND C ");

            cpu.a = cpu.alu_and(cpu.reg.a, cpu.reg.c);

            4
        }

        0xa2 => {
            println!("AND D ");

            cpu.a = cpu.alu_and(cpu.reg.a, cpu.reg.d);

            4
        }

        0xa3 => {
            println!("AND E ");

            cpu.a = cpu.alu_and(cpu.reg.a, cpu.reg.e);

            4
        }

        0xa4 => {
            println!("AND H ");

            cpu.a = cpu.alu_and(cpu.reg.a, cpu.reg.h);

            4
        }

        0xa5 => {
            println!("AND L ");

            cpu.a = cpu.alu_and(cpu.reg.a, cpu.reg.l);

            4
        }

        0xa6 => {
            println!("AND (HL) ");

            cpu.a = cpu.alu_and(cpu.reg.a, cpu.mmu.read_u16(cpu.reg.hl()));

            8
        }

        0xa7 => {
            println!("AND A ");

            cpu.a = cpu.alu_and(cpu.reg.a, cpu.reg.a);

            4
        }

        0xa8 => {
            println!("XOR B ");

            cpu.reg.a = cpu.alu_xor(cpu.reg.a, cpu.reg.b);

            4
        }

        0xa9 => {
            println!("XOR C ");

            cpu.reg.a = cpu.alu_xor(cpu.reg.a, cpu.reg.c);

            4
        }

        0xaa => {
            println!("XOR D ");

            cpu.reg.a = cpu.alu_xor(cpu.reg.a, cpu.reg.d);

            4
        }

        0xab => {
            println!("XOR E ");

            cpu.reg.a = cpu.alu_xor(cpu.reg.a, cpu.reg.e);

            4
        }

        0xac => {
            println!("XOR H ");

            cpu.reg.a = cpu.alu_xor(cpu.reg.a, cpu.reg.h);

            4
        }

        0xad => {
            println!("XOR L ");

            cpu.reg.a = cpu.alu_xor(cpu.reg.a, cpu.reg.l);

            4
        }

        0xae => {
            println!("XOR (HL) ");

            cpu.reg.a = cpu.alu_xor(cpu.reg.a, cpu.mmu.read_u16(cpu.reg.hl()));

            8
        }

        0xaf => {
            println!("XOR A ");

            cpu.reg.a = cpu.alu_xor(cpu.reg.a, cpu.reg.a);

            4
        }

        0xb0 => {
            println!("OR B ");

            cpu.reg.a = cpu.alu_or(cpu.reg.a, cpu.reg.b);

            4
        }

        0xb1 => {
            println!("OR C ");

            cpu.reg.a = cpu.alu_or(cpu.reg.a, cpu.reg.c);

            4
        }

        0xb2 => {
            println!("OR D ");

            cpu.reg.a = cpu.alu_or(cpu.reg.a, cpu.reg.d);

            4
        }

        0xb3 => {
            println!("OR E ");

            cpu.reg.a = cpu.alu_or(cpu.reg.a, cpu.reg.e);

            4
        }

        0xb4 => {
            println!("OR H ");

            cpu.reg.a = cpu.alu_or(cpu.reg.a, cpu.reg.h);

            4
        }

        0xb5 => {
            println!("OR L ");

            cpu.reg.a = cpu.alu_or(cpu.reg.a, cpu.reg.l);

            4
        }

        0xb6 => {
            println!("OR (HL) ");

            cpu.reg.a = cpu.alu_or(cpu.reg.a, cpu.mmu.read_u16(cpu.reg.hl()));

            8
        }

        0xb7 => {
            println!("OR A ");

            cpu.reg.a = cpu.alu_or(cpu.reg.a, cpu.reg.a);

            4
        }

        0xb8 => {
            println!("CP B ");

            cpu.reg.a = cpu.alu_cp(cpu.reg.a, cpu.reg.b);

            4
        }

        0xb9 => {
            println!("CP C ");

            cpu.reg.a = cpu.alu_cp(cpu.reg.a, cpu.reg.c);

            4
        }

        0xba => {
            println!("CP D ");

            cpu.reg.a = cpu.alu_cp(cpu.reg.a, cpu.reg.d);

            4
        }

        0xbb => {
            println!("CP E ");

            cpu.reg.a = cpu.alu_cp(cpu.reg.a, cpu.reg.e);

            4
        }

        0xbc => {
            println!("CP H ");

            cpu.reg.a = cpu.alu_cp(cpu.reg.a, cpu.reg.h);

            4
        }

        0xbd => {
            println!("CP L ");

            cpu.reg.a = cpu.alu_cp(cpu.reg.a, cpu.reg.l);

            4
        }

        0xbe => {
            println!("CP (HL) ");

            cpu.reg.a = cpu.alu_cp(cpu.reg.a, cpu.mmu.read_u16(cpu.reg.hl()));

            8
        }

        0xbf => {
            println!("CP A ");

            cpu.reg.a = cpu.alu_cp(cpu.reg.a, cpu.reg.a);

            4
        }

        0xc0 => {
            println!("RET NZ ");

            if !cpu.reg.flag_get(Z) {
                cpu.pc = cpu.stack_pop();
            }

            20
        }

        0xc1 => {
            println!("POP BC ");

            cpu.reg.set_bc(cpu.stack_pop());

            12
        }

        0xc2 => {
            println!("JP NZ a16");

            let value = cpu.read_nn();

            if !cpu.reg.flag_get(Z) {
                cpu.pc = value;
            }

            16
        }

        0xc3 => {
            println!("JP a16 ");

            cpu.pc = cpu.read_nn();

            16
        }

        0xc4 => {
            println!("CALL NZ a16");

            let value = cpu.read_nn();

            if !cpu.reg.flag_get(Z) {
                cpu.stack_push(cpu.pc);
                cpu.pc = value;
            }

            24
        }

        0xc5 => {
            println!("PUSH BC ");

            cpu.stack_push(cpu.reg.bc());

            16
        }

        0xc6 => {
            println!("ADD A d8");

            cpu.reg.a = cpu.alu_add_u8(cpu.reg.a, cpu.read_n());

            8
        }

        0xc7 => {
            println!("RST 0 ");

            cpu.stack_push(cpu.pc);
            cpu.pc = 0_u16;

            16
        }

        0xc8 => {
            println!("RET Z ");

            if cpu.reg.flag_get(Z) {
                cpu.pc = cpu.stack_pop();
            }

            20
        }

        0xc9 => {
            println!("RET  ");

            cpu.pc = cpu.stack_pop();

            16
        }

        0xca => {
            println!("JP Z a16");

            let value = cpu.read_nn();

            if cpu.reg.flag_get(Z) {
                cpu.pc = value;
            }

            16
        }

        0xcb => {
            println!("PREFIX CB ");

            4
        }

        0xcc => {
            println!("CALL Z a16");

            let value = cpu.read_nn();

            if cpu.reg.flag_get(Z) {
                cpu.stack_push(cpu.pc);
                cpu.pc = value;
            }

            24
        }

        0xcd => {
            println!("CALL a16 ");

            let value = cpu.read_nn();

            cpu.stack_push(cpu.pc);
            cpu.pc = value;

            24
        }

        0xce => {
            println!("ADC A d8");

            cpu.reg.a = cpu.alu_adc(cpu.reg.a, cpu.read_n());

            8
        }

        0xcf => {
            println!("RST 1 ");

            cpu.stack_push(cpu.pc);
            cpu.pc = 1_u16;

            16
        }

        0xd0 => {
            println!("RET NC ");

            if !cpu.reg.flag_get(C) {
                cpu.pc = cpu.stack_pop();
            }

            20
        }

        0xd1 => {
            println!("POP DE ");

            cpu.reg.set_de(cpu.stack_pop());

            12
        }

        0xd2 => {
            println!("JP NC a16");

            let value = cpu.read_nn();

            if !cpu.reg.flag_get(C) {
                cpu.pc = value;
            }

            16
        }

        0xd4 => {
            println!("CALL NC a16");

            let value = cpu.read_nn();

            if !cpu.reg.flag_get(C) {
                cpu.stack_push(cpu.pc);
                cpu.pc = value;
            }

            24
        }

        0xd5 => {
            println!("PUSH DE ");

            cpu.stack_push(cpu.reg.de());

            16
        }

        0xd6 => {
            println!("SUB d8 ");

            cpu.reg.a = cpu.alu_sub(cpu.reg.a, cpu.read_n());

            8
        }

        0xd7 => {
            println!("RST 2 ");

            cpu.stack_push(cpu.pc);
            cpu.pc = 2_u16;

            16
        }

        0xd8 => {
            println!("RET C ");

            if cpu.reg.flag_get(C) {
                cpu.pc = cpu.stack_pop();
            }

            20
        }

        0xd9 => {
            println!("RETI  ");

            cpu.pc = cpu.stack_pop();
            cpu.ei = true;

            16
        }

        0xda => {
            println!("JP C a16");

            let value = cpu.read_nn();

            if cpu.reg.flag_get(C) {
                cpu.pc = value;
            }

            16
        }

        0xdc => {
            println!("CALL C a16");

            let value = cpu.read_nn();

            if cpu.reg.flag_get(C) {
                cpu.stack_push(cpu.pc);
                cpu.pc = value;
            }

            24
        }

        0xde => {
            println!("SBC A d8");

            cpu.reg.a = cpu.alu_sub(cpu.reg.a);

            8
        }

        0xdf => {
            println!("RST 3 ");

            cpu.stack_push(cpu.pc);
            cpu.pc = 3_u16;

            16
        }

        0xe0 => {
            println!("LDH (a8) A");

            let addr = 0xFF00 | u16::from(cpu.read_n());
            let value = cpu.reg.a;

            cpu.mmu.write_u8(addr, value);

            12
        }

        0xe1 => {
            println!("POP HL ");

            cpu.reg.set_hl(cpu.stack_pop());

            12
        }

        0xe2 => {
            println!("LD (C) A");

            let addr = cpu.reg.c;
            let value = cpu.reg.a;
            cpu.mmu.write_u8(addr, value);

            8
        }

        0xe5 => {
            println!("PUSH HL ");

            cpu.stack_push(cpu.reg.hl());

            16
        }

        0xe6 => {
            println!("AND d8 ");

            cpu.a = cpu.alu_and(cpu.reg.a, cpu.read_n());

            8
        }

        0xe7 => {
            println!("RST 4 ");

            cpu.stack_push(cpu.pc);
            cpu.pc = 4_u16;

            16
        }

        0xe8 => {
            println!("ADD SP r8");

            let value = cpu.alu_add_u8(cpu.sp, cpu.read_d());
            cpu.sp = value;

            16
        }

        0xe9 => {
            println!("JP (HL) ");

            cpu.pc = cpu.mmu.read_u16(cpu.reg.hl());

            4
        }

        0xea => {
            println!("LD (a16) A");

            let addr = cpu.read_nn();
            let value = cpu.reg.a;
            cpu.mmu.write_u8(addr, value);

            16
        }

        0xee => {
            println!("XOR d8 ");

            cpu.reg.a = cpu.alu_xor(cpu.reg.a, cpu.read_n());

            8
        }

        0xef => {
            println!("RST 5 ");

            cpu.stack_push(cpu.pc);
            cpu.pc = 5_u16;

            16
        }

        0xf0 => {
            println!("LDH A (a8)");

            let addr = cpu.mmu.read_u8(0xFF00 | u16::from(cpu.read_n()));
            self.reg.a = cpu.mmu.read_u8(addr);

            12
        }

        0xf1 => {
            println!("POP AF ");

            cpu.mmu.write_u16(addr, cpu.stack_pop());

            12
        }

        0xf2 => {
            println!("LD A (C)");

            let value = cpu.mmu.read_u8(cpu.reg.c);
            cpu.reg.a = value;

            8
        }

        0xf3 => {
            println!("DI  ");

            cpu.ei = false;

            4
        }

        0xf5 => {
            println!("PUSH AF ");

            cpu.stack_push(cpu.read_af());

            16
        }

        0xf6 => {
            println!("OR d8 ");

            cpu.reg.a = cpu.alu_or(cpu.reg.a, cpu.read_n());

            8
        }

        0xf7 => {
            println!("RST 6 ");

            cpu.stack_push(cpu.pc);
            cpu.pc = 6_u16;

            16
        }

        0xf8 => {
            println!("LD HL SP+r8");

            let value = cpu.read_spr8();
            cpu.reg.set_hl(value);

            12
        }

        0xf9 => {
            println!("LD SP HL");

            let value = cpu.reg.hl();
            cpu.sp = value;

            8
        }

        0xfa => {
            println!("LD A (a16)");

            let value = cpu.mmu.read_u16(cpu.read_nn());
            cpu.reg.a = value;

            16
        }

        0xfb => {
            println!("EI  ");

            cpu.ei = true;

            4
        }

        0xfe => {
            println!("CP d8 ");

            cpu.reg.a = cpu.alu_cp(cpu.reg.a, cpu.read_n());

            8
        }

        0xff => {
            println!("RST 7 ");

            cpu.stack_push(cpu.pc);
            cpu.pc = 7_u16;

            16
        }

        0xcb => {
            let postfix = cpu.read_opcode();

            match postfix {
                0x00 => {
                    println!("RLC B ");

                    let op1 = cpu.reg.b;

                    let value = cpu.alu_rlc(op1);

                    cpu.reg.b = value;

                    8
                }

                0x01 => {
                    println!("RLC C ");

                    let op1 = cpu.reg.c;

                    let value = cpu.alu_rlc(op1);

                    cpu.reg.c = value;

                    8
                }

                0x02 => {
                    println!("RLC D ");

                    let op1 = cpu.reg.d;

                    let value = cpu.alu_rlc(op1);

                    cpu.reg.d = value;

                    8
                }

                0x03 => {
                    println!("RLC E ");

                    let op1 = cpu.reg.e;

                    let value = cpu.alu_rlc(op1);

                    cpu.reg.e = value;

                    8
                }

                0x04 => {
                    println!("RLC H ");

                    let op1 = cpu.reg.h;

                    let value = cpu.alu_rlc(op1);

                    cpu.reg.h = value;

                    8
                }

                0x05 => {
                    println!("RLC L ");

                    let op1 = cpu.reg.l;

                    let value = cpu.alu_rlc(op1);

                    cpu.reg.l = value;

                    8
                }

                0x06 => {
                    println!("RLC (HL) ");

                    let addr = cpu.reg.hl();
                    let op1 = cpu.mmu.read_u8(addr);

                    let value = cpu.alu_rlc(op1);

                    cpu.mmu.write_u8(addr, value);

                    16
                }

                0x07 => {
                    println!("RLC A ");

                    let op1 = cpu.reg.a;

                    let value = cpu.alu_rlc(op1);

                    cpu.reg.a = value;

                    8
                }

                0x08 => {
                    println!("RRC B ");

                    let op1 = cpu.reg.b;

                    let value = cpu.alu_rrc(op1);

                    cpu.reg.b = value;

                    8
                }

                0x09 => {
                    println!("RRC C ");

                    let op1 = cpu.reg.c;

                    let value = cpu.alu_rrc(op1);

                    cpu.reg.c = value;

                    8
                }

                0x0a => {
                    println!("RRC D ");

                    let op1 = cpu.reg.d;

                    let value = cpu.alu_rrc(op1);

                    cpu.reg.d = value;

                    8
                }

                0x0b => {
                    println!("RRC E ");

                    let op1 = cpu.reg.e;

                    let value = cpu.alu_rrc(op1);

                    cpu.reg.e = value;

                    8
                }

                0x0c => {
                    println!("RRC H ");

                    let op1 = cpu.reg.h;

                    let value = cpu.alu_rrc(op1);

                    cpu.reg.h = value;

                    8
                }

                0x0d => {
                    println!("RRC L ");

                    let op1 = cpu.reg.l;

                    let value = cpu.alu_rrc(op1);

                    cpu.reg.l = value;

                    8
                }

                0x0e => {
                    println!("RRC (HL) ");

                    let addr = cpu.reg.hl();
                    let op1 = cpu.mmu.read_u8(addr);

                    let value = cpu.alu_rrc(op1);

                    cpu.mmu.write_u8(addr, value);

                    16
                }

                0x0f => {
                    println!("RRC A ");

                    let op1 = cpu.reg.a;

                    let value = cpu.alu_rrc(op1);

                    cpu.reg.a = value;

                    8
                }

                0x10 => {
                    println!("RL B ");

                    let op1 = cpu.reg.b;

                    let value = cpu.alu_rl(op1);

                    cpu.reg.b = value;

                    8
                }

                0x11 => {
                    println!("RL C ");

                    let op1 = cpu.reg.c;

                    let value = cpu.alu_rl(op1);

                    cpu.reg.c = value;

                    8
                }

                0x12 => {
                    println!("RL D ");

                    let op1 = cpu.reg.d;

                    let value = cpu.alu_rl(op1);

                    cpu.reg.d = value;

                    8
                }

                0x13 => {
                    println!("RL E ");

                    let op1 = cpu.reg.e;

                    let value = cpu.alu_rl(op1);

                    cpu.reg.e = value;

                    8
                }

                0x14 => {
                    println!("RL H ");

                    let op1 = cpu.reg.h;

                    let value = cpu.alu_rl(op1);

                    cpu.reg.h = value;

                    8
                }

                0x15 => {
                    println!("RL L ");

                    let op1 = cpu.reg.l;

                    let value = cpu.alu_rl(op1);

                    cpu.reg.l = value;

                    8
                }

                0x16 => {
                    println!("RL (HL) ");

                    let addr = cpu.reg.hl();
                    let op1 = cpu.mmu.read_u8(addr);

                    let value = cpu.alu_rl(op1);

                    cpu.mmu.write_u8(addr, value);

                    16
                }

                0x17 => {
                    println!("RL A ");

                    let op1 = cpu.reg.a;

                    let value = cpu.alu_rl(op1);

                    cpu.reg.a = value;

                    8
                }

                0x18 => {
                    println!("RR B ");

                    let op1 = cpu.reg.b;

                    let value = cpu.alu_rr(op1);

                    cpu.reg.b = value;

                    8
                }

                0x19 => {
                    println!("RR C ");

                    let op1 = cpu.reg.c;

                    let value = cpu.alu_rr(op1);

                    cpu.reg.c = value;

                    8
                }

                0x1a => {
                    println!("RR D ");

                    let op1 = cpu.reg.d;

                    let value = cpu.alu_rr(op1);

                    cpu.reg.d = value;

                    8
                }

                0x1b => {
                    println!("RR E ");

                    let op1 = cpu.reg.e;

                    let value = cpu.alu_rr(op1);

                    cpu.reg.e = value;

                    8
                }

                0x1c => {
                    println!("RR H ");

                    let op1 = cpu.reg.h;

                    let value = cpu.alu_rr(op1);

                    cpu.reg.h = value;

                    8
                }

                0x1d => {
                    println!("RR L ");

                    let op1 = cpu.reg.l;

                    let value = cpu.alu_rr(op1);

                    cpu.reg.l = value;

                    8
                }

                0x1e => {
                    println!("RR (HL) ");

                    let addr = cpu.reg.hl();
                    let op1 = cpu.mmu.read_u8(addr);

                    let value = cpu.alu_rr(op1);

                    cpu.mmu.write_u8(addr, value);

                    16
                }

                0x1f => {
                    println!("RR A ");

                    let op1 = cpu.reg.a;

                    let value = cpu.alu_rr(op1);

                    cpu.reg.a = value;

                    8
                }

                0x20 => {
                    println!("SLA B ");

                    let op1 = cpu.reg.b;

                    let value = cpu.alu_sla(op1);

                    cpu.reg.b = value;

                    8
                }

                0x21 => {
                    println!("SLA C ");

                    let op1 = cpu.reg.c;

                    let value = cpu.alu_sla(op1);

                    cpu.reg.c = value;

                    8
                }

                0x22 => {
                    println!("SLA D ");

                    let op1 = cpu.reg.d;

                    let value = cpu.alu_sla(op1);

                    cpu.reg.d = value;

                    8
                }

                0x23 => {
                    println!("SLA E ");

                    let op1 = cpu.reg.e;

                    let value = cpu.alu_sla(op1);

                    cpu.reg.e = value;

                    8
                }

                0x24 => {
                    println!("SLA H ");

                    let op1 = cpu.reg.h;

                    let value = cpu.alu_sla(op1);

                    cpu.reg.h = value;

                    8
                }

                0x25 => {
                    println!("SLA L ");

                    let op1 = cpu.reg.l;

                    let value = cpu.alu_sla(op1);

                    cpu.reg.l = value;

                    8
                }

                0x26 => {
                    println!("SLA (HL) ");

                    let addr = cpu.reg.hl();
                    let op1 = cpu.mmu.read_u8(addr);

                    let value = cpu.alu_sla(op1);

                    cpu.mmu.write_u8(addr, value);

                    16
                }

                0x27 => {
                    println!("SLA A ");

                    let op1 = cpu.reg.a;

                    let value = cpu.alu_sla(op1);

                    cpu.reg.a = value;

                    8
                }

                0x28 => {
                    println!("SRA B ");

                    let op1 = cpu.reg.b;

                    let value = cpu.alu_sra(op1);

                    cpu.reg.b = value;

                    8
                }

                0x29 => {
                    println!("SRA C ");

                    let op1 = cpu.reg.c;

                    let value = cpu.alu_sra(op1);

                    cpu.reg.c = value;

                    8
                }

                0x2a => {
                    println!("SRA D ");

                    let op1 = cpu.reg.d;

                    let value = cpu.alu_sra(op1);

                    cpu.reg.d = value;

                    8
                }

                0x2b => {
                    println!("SRA E ");

                    let op1 = cpu.reg.e;

                    let value = cpu.alu_sra(op1);

                    cpu.reg.e = value;

                    8
                }

                0x2c => {
                    println!("SRA H ");

                    let op1 = cpu.reg.h;

                    let value = cpu.alu_sra(op1);

                    cpu.reg.h = value;

                    8
                }

                0x2d => {
                    println!("SRA L ");

                    let op1 = cpu.reg.l;

                    let value = cpu.alu_sra(op1);

                    cpu.reg.l = value;

                    8
                }

                0x2e => {
                    println!("SRA (HL) ");

                    let addr = cpu.reg.hl();
                    let op1 = cpu.mmu.read_u8(addr);

                    let value = cpu.alu_sra(op1);

                    cpu.mmu.write_u8(addr, value);

                    16
                }

                0x2f => {
                    println!("SRA A ");

                    let op1 = cpu.reg.a;

                    let value = cpu.alu_sra(op1);

                    cpu.reg.a = value;

                    8
                }

                0x30 => {
                    println!("SWAP B ");

                    let op1 = cpu.reg.b;

                    let value = cpu.alu_swap(op1);

                    cpu.reg.b = value;

                    8
                }

                0x31 => {
                    println!("SWAP C ");

                    let op1 = cpu.reg.c;

                    let value = cpu.alu_swap(op1);

                    cpu.reg.c = value;

                    8
                }

                0x32 => {
                    println!("SWAP D ");

                    let op1 = cpu.reg.d;

                    let value = cpu.alu_swap(op1);

                    cpu.reg.d = value;

                    8
                }

                0x33 => {
                    println!("SWAP E ");

                    let op1 = cpu.reg.e;

                    let value = cpu.alu_swap(op1);

                    cpu.reg.e = value;

                    8
                }

                0x34 => {
                    println!("SWAP H ");

                    let op1 = cpu.reg.h;

                    let value = cpu.alu_swap(op1);

                    cpu.reg.h = value;

                    8
                }

                0x35 => {
                    println!("SWAP L ");

                    let op1 = cpu.reg.l;

                    let value = cpu.alu_swap(op1);

                    cpu.reg.l = value;

                    8
                }

                0x36 => {
                    println!("SWAP (HL) ");

                    let addr = cpu.reg.hl();
                    let op1 = cpu.mmu.read_u8(addr);

                    let value = cpu.alu_swap(op1);

                    cpu.mmu.write_u8(addr, value);

                    16
                }

                0x37 => {
                    println!("SWAP A ");

                    let op1 = cpu.reg.a;

                    let value = cpu.alu_swap(op1);

                    cpu.reg.a = value;

                    8
                }

                0x38 => {
                    println!("SRL B ");

                    let op1 = cpu.reg.b;

                    let value = cpu.alu_srl(op1);

                    cpu.reg.b = value;

                    8
                }

                0x39 => {
                    println!("SRL C ");

                    let op1 = cpu.reg.c;

                    let value = cpu.alu_srl(op1);

                    cpu.reg.c = value;

                    8
                }

                0x3a => {
                    println!("SRL D ");

                    let op1 = cpu.reg.d;

                    let value = cpu.alu_srl(op1);

                    cpu.reg.d = value;

                    8
                }

                0x3b => {
                    println!("SRL E ");

                    let op1 = cpu.reg.e;

                    let value = cpu.alu_srl(op1);

                    cpu.reg.e = value;

                    8
                }

                0x3c => {
                    println!("SRL H ");

                    let op1 = cpu.reg.h;

                    let value = cpu.alu_srl(op1);

                    cpu.reg.h = value;

                    8
                }

                0x3d => {
                    println!("SRL L ");

                    let op1 = cpu.reg.l;

                    let value = cpu.alu_srl(op1);

                    cpu.reg.l = value;

                    8
                }

                0x3e => {
                    println!("SRL (HL) ");

                    let addr = cpu.reg.hl();
                    let op1 = cpu.mmu.read_u8(addr);

                    let value = cpu.alu_srl(op1);

                    cpu.mmu.write_u8(addr, value);

                    16
                }

                0x3f => {
                    println!("SRL A ");

                    let op1 = cpu.reg.a;

                    let value = cpu.alu_srl(op1);

                    cpu.reg.a = value;

                    8
                }

                0x40 => {
                    println!("BIT 0 B");

                    cpu.alu_bit(cpu.reg.b, 0);

                    8
                }

                0x41 => {
                    println!("BIT 0 C");

                    cpu.alu_bit(cpu.reg.c, 0);

                    8
                }

                0x42 => {
                    println!("BIT 0 D");

                    cpu.alu_bit(cpu.reg.d, 0);

                    8
                }

                0x43 => {
                    println!("BIT 0 E");

                    cpu.alu_bit(cpu.reg.e, 0);

                    8
                }

                0x44 => {
                    println!("BIT 0 H");

                    cpu.alu_bit(cpu.reg.h, 0);

                    8
                }

                0x45 => {
                    println!("BIT 0 L");

                    cpu.alu_bit(cpu.reg.l, 0);

                    8
                }

                0x46 => {
                    println!("BIT 0 (HL)");

                    cpu.alu_bit(cpu.mmu.read_u16(cpu.reg.hl()), 0);

                    16
                }

                0x47 => {
                    println!("BIT 0 A");

                    cpu.alu_bit(cpu.reg.a, 0);

                    8
                }

                0x48 => {
                    println!("BIT 1 B");

                    cpu.alu_bit(cpu.reg.b, 1);

                    8
                }

                0x49 => {
                    println!("BIT 1 C");

                    cpu.alu_bit(cpu.reg.c, 1);

                    8
                }

                0x4a => {
                    println!("BIT 1 D");

                    cpu.alu_bit(cpu.reg.d, 1);

                    8
                }

                0x4b => {
                    println!("BIT 1 E");

                    cpu.alu_bit(cpu.reg.e, 1);

                    8
                }

                0x4c => {
                    println!("BIT 1 H");

                    cpu.alu_bit(cpu.reg.h, 1);

                    8
                }

                0x4d => {
                    println!("BIT 1 L");

                    cpu.alu_bit(cpu.reg.l, 1);

                    8
                }

                0x4e => {
                    println!("BIT 1 (HL)");

                    cpu.alu_bit(cpu.mmu.read_u16(cpu.reg.hl()), 1);

                    16
                }

                0x4f => {
                    println!("BIT 1 A");

                    cpu.alu_bit(cpu.reg.a, 1);

                    8
                }

                0x50 => {
                    println!("BIT 2 B");

                    cpu.alu_bit(cpu.reg.b, 2);

                    8
                }

                0x51 => {
                    println!("BIT 2 C");

                    cpu.alu_bit(cpu.reg.c, 2);

                    8
                }

                0x52 => {
                    println!("BIT 2 D");

                    cpu.alu_bit(cpu.reg.d, 2);

                    8
                }

                0x53 => {
                    println!("BIT 2 E");

                    cpu.alu_bit(cpu.reg.e, 2);

                    8
                }

                0x54 => {
                    println!("BIT 2 H");

                    cpu.alu_bit(cpu.reg.h, 2);

                    8
                }

                0x55 => {
                    println!("BIT 2 L");

                    cpu.alu_bit(cpu.reg.l, 2);

                    8
                }

                0x56 => {
                    println!("BIT 2 (HL)");

                    cpu.alu_bit(cpu.mmu.read_u16(cpu.reg.hl()), 2);

                    16
                }

                0x57 => {
                    println!("BIT 2 A");

                    cpu.alu_bit(cpu.reg.a, 2);

                    8
                }

                0x58 => {
                    println!("BIT 3 B");

                    cpu.alu_bit(cpu.reg.b, 3);

                    8
                }

                0x59 => {
                    println!("BIT 3 C");

                    cpu.alu_bit(cpu.reg.c, 3);

                    8
                }

                0x5a => {
                    println!("BIT 3 D");

                    cpu.alu_bit(cpu.reg.d, 3);

                    8
                }

                0x5b => {
                    println!("BIT 3 E");

                    cpu.alu_bit(cpu.reg.e, 3);

                    8
                }

                0x5c => {
                    println!("BIT 3 H");

                    cpu.alu_bit(cpu.reg.h, 3);

                    8
                }

                0x5d => {
                    println!("BIT 3 L");

                    cpu.alu_bit(cpu.reg.l, 3);

                    8
                }

                0x5e => {
                    println!("BIT 3 (HL)");

                    cpu.alu_bit(cpu.mmu.read_u16(cpu.reg.hl()), 3);

                    16
                }

                0x5f => {
                    println!("BIT 3 A");

                    cpu.alu_bit(cpu.reg.a, 3);

                    8
                }

                0x60 => {
                    println!("BIT 4 B");

                    cpu.alu_bit(cpu.reg.b, 4);

                    8
                }

                0x61 => {
                    println!("BIT 4 C");

                    cpu.alu_bit(cpu.reg.c, 4);

                    8
                }

                0x62 => {
                    println!("BIT 4 D");

                    cpu.alu_bit(cpu.reg.d, 4);

                    8
                }

                0x63 => {
                    println!("BIT 4 E");

                    cpu.alu_bit(cpu.reg.e, 4);

                    8
                }

                0x64 => {
                    println!("BIT 4 H");

                    cpu.alu_bit(cpu.reg.h, 4);

                    8
                }

                0x65 => {
                    println!("BIT 4 L");

                    cpu.alu_bit(cpu.reg.l, 4);

                    8
                }

                0x66 => {
                    println!("BIT 4 (HL)");

                    cpu.alu_bit(cpu.mmu.read_u16(cpu.reg.hl()), 4);

                    16
                }

                0x67 => {
                    println!("BIT 4 A");

                    cpu.alu_bit(cpu.reg.a, 4);

                    8
                }

                0x68 => {
                    println!("BIT 5 B");

                    cpu.alu_bit(cpu.reg.b, 5);

                    8
                }

                0x69 => {
                    println!("BIT 5 C");

                    cpu.alu_bit(cpu.reg.c, 5);

                    8
                }

                0x6a => {
                    println!("BIT 5 D");

                    cpu.alu_bit(cpu.reg.d, 5);

                    8
                }

                0x6b => {
                    println!("BIT 5 E");

                    cpu.alu_bit(cpu.reg.e, 5);

                    8
                }

                0x6c => {
                    println!("BIT 5 H");

                    cpu.alu_bit(cpu.reg.h, 5);

                    8
                }

                0x6d => {
                    println!("BIT 5 L");

                    cpu.alu_bit(cpu.reg.l, 5);

                    8
                }

                0x6e => {
                    println!("BIT 5 (HL)");

                    cpu.alu_bit(cpu.mmu.read_u16(cpu.reg.hl()), 5);

                    16
                }

                0x6f => {
                    println!("BIT 5 A");

                    cpu.alu_bit(cpu.reg.a, 5);

                    8
                }

                0x70 => {
                    println!("BIT 6 B");

                    cpu.alu_bit(cpu.reg.b, 6);

                    8
                }

                0x71 => {
                    println!("BIT 6 C");

                    cpu.alu_bit(cpu.reg.c, 6);

                    8
                }

                0x72 => {
                    println!("BIT 6 D");

                    cpu.alu_bit(cpu.reg.d, 6);

                    8
                }

                0x73 => {
                    println!("BIT 6 E");

                    cpu.alu_bit(cpu.reg.e, 6);

                    8
                }

                0x74 => {
                    println!("BIT 6 H");

                    cpu.alu_bit(cpu.reg.h, 6);

                    8
                }

                0x75 => {
                    println!("BIT 6 L");

                    cpu.alu_bit(cpu.reg.l, 6);

                    8
                }

                0x76 => {
                    println!("BIT 6 (HL)");

                    cpu.alu_bit(cpu.mmu.read_u16(cpu.reg.hl()), 6);

                    16
                }

                0x77 => {
                    println!("BIT 6 A");

                    cpu.alu_bit(cpu.reg.a, 6);

                    8
                }

                0x78 => {
                    println!("BIT 7 B");

                    cpu.alu_bit(cpu.reg.b, 7);

                    8
                }

                0x79 => {
                    println!("BIT 7 C");

                    cpu.alu_bit(cpu.reg.c, 7);

                    8
                }

                0x7a => {
                    println!("BIT 7 D");

                    cpu.alu_bit(cpu.reg.d, 7);

                    8
                }

                0x7b => {
                    println!("BIT 7 E");

                    cpu.alu_bit(cpu.reg.e, 7);

                    8
                }

                0x7c => {
                    println!("BIT 7 H");

                    cpu.alu_bit(cpu.reg.h, 7);

                    8
                }

                0x7d => {
                    println!("BIT 7 L");

                    cpu.alu_bit(cpu.reg.l, 7);

                    8
                }

                0x7e => {
                    println!("BIT 7 (HL)");

                    cpu.alu_bit(cpu.mmu.read_u16(cpu.reg.hl()), 7);

                    16
                }

                0x7f => {
                    println!("BIT 7 A");

                    cpu.alu_bit(cpu.reg.a, 7);

                    8
                }

                0x80 => {
                    println!("RES 0 B");

                    let value = cpu.alu_res(cpu.reg.b, 0);

                    cpu.reg.b = value;

                    8
                }

                0x81 => {
                    println!("RES 0 C");

                    let value = cpu.alu_res(cpu.reg.c, 0);

                    cpu.reg.c = value;

                    8
                }

                0x82 => {
                    println!("RES 0 D");

                    let value = cpu.alu_res(cpu.reg.d, 0);

                    cpu.reg.d = value;

                    8
                }

                0x83 => {
                    println!("RES 0 E");

                    let value = cpu.alu_res(cpu.reg.e, 0);

                    cpu.reg.e = value;

                    8
                }

                0x84 => {
                    println!("RES 0 H");

                    let value = cpu.alu_res(cpu.reg.h, 0);

                    cpu.reg.h = value;

                    8
                }

                0x85 => {
                    println!("RES 0 L");

                    let value = cpu.alu_res(cpu.reg.l, 0);

                    cpu.reg.l = value;

                    8
                }

                0x86 => {
                    println!("RES 0 (HL)");

                    let addr = cpu.reg.hl();
                    let op1 = cpu.mmu.read_u8(addr);

                    let value = cpu.alu_res(op1, 0);

                    cpu.mmu.write_u8(addr, value);

                    16
                }

                0x87 => {
                    println!("RES 0 A");

                    let value = cpu.alu_res(cpu.reg.a, 0);

                    cpu.reg.a = value;

                    8
                }

                0x88 => {
                    println!("RES 1 B");

                    let value = cpu.alu_res(cpu.reg.b, 1);

                    cpu.reg.b = value;

                    8
                }

                0x89 => {
                    println!("RES 1 C");

                    let value = cpu.alu_res(cpu.reg.c, 1);

                    cpu.reg.c = value;

                    8
                }

                0x8a => {
                    println!("RES 1 D");

                    let value = cpu.alu_res(cpu.reg.d, 1);

                    cpu.reg.d = value;

                    8
                }

                0x8b => {
                    println!("RES 1 E");

                    let value = cpu.alu_res(cpu.reg.e, 1);

                    cpu.reg.e = value;

                    8
                }

                0x8c => {
                    println!("RES 1 H");

                    let value = cpu.alu_res(cpu.reg.h, 1);

                    cpu.reg.h = value;

                    8
                }

                0x8d => {
                    println!("RES 1 L");

                    let value = cpu.alu_res(cpu.reg.l, 1);

                    cpu.reg.l = value;

                    8
                }

                0x8e => {
                    println!("RES 1 (HL)");

                    let addr = cpu.reg.hl();
                    let op1 = cpu.mmu.read_u8(addr);

                    let value = cpu.alu_res(op1, 1);

                    cpu.mmu.write_u8(addr, value);

                    16
                }

                0x8f => {
                    println!("RES 1 A");

                    let value = cpu.alu_res(cpu.reg.a, 1);

                    cpu.reg.a = value;

                    8
                }

                0x90 => {
                    println!("RES 2 B");

                    let value = cpu.alu_res(cpu.reg.b, 2);

                    cpu.reg.b = value;

                    8
                }

                0x91 => {
                    println!("RES 2 C");

                    let value = cpu.alu_res(cpu.reg.c, 2);

                    cpu.reg.c = value;

                    8
                }

                0x92 => {
                    println!("RES 2 D");

                    let value = cpu.alu_res(cpu.reg.d, 2);

                    cpu.reg.d = value;

                    8
                }

                0x93 => {
                    println!("RES 2 E");

                    let value = cpu.alu_res(cpu.reg.e, 2);

                    cpu.reg.e = value;

                    8
                }

                0x94 => {
                    println!("RES 2 H");

                    let value = cpu.alu_res(cpu.reg.h, 2);

                    cpu.reg.h = value;

                    8
                }

                0x95 => {
                    println!("RES 2 L");

                    let value = cpu.alu_res(cpu.reg.l, 2);

                    cpu.reg.l = value;

                    8
                }

                0x96 => {
                    println!("RES 2 (HL)");

                    let addr = cpu.reg.hl();
                    let op1 = cpu.mmu.read_u8(addr);

                    let value = cpu.alu_res(op1, 2);

                    cpu.mmu.write_u8(addr, value);

                    16
                }

                0x97 => {
                    println!("RES 2 A");

                    let value = cpu.alu_res(cpu.reg.a, 2);

                    cpu.reg.a = value;

                    8
                }

                0x98 => {
                    println!("RES 3 B");

                    let value = cpu.alu_res(cpu.reg.b, 3);

                    cpu.reg.b = value;

                    8
                }

                0x99 => {
                    println!("RES 3 C");

                    let value = cpu.alu_res(cpu.reg.c, 3);

                    cpu.reg.c = value;

                    8
                }

                0x9a => {
                    println!("RES 3 D");

                    let value = cpu.alu_res(cpu.reg.d, 3);

                    cpu.reg.d = value;

                    8
                }

                0x9b => {
                    println!("RES 3 E");

                    let value = cpu.alu_res(cpu.reg.e, 3);

                    cpu.reg.e = value;

                    8
                }

                0x9c => {
                    println!("RES 3 H");

                    let value = cpu.alu_res(cpu.reg.h, 3);

                    cpu.reg.h = value;

                    8
                }

                0x9d => {
                    println!("RES 3 L");

                    let value = cpu.alu_res(cpu.reg.l, 3);

                    cpu.reg.l = value;

                    8
                }

                0x9e => {
                    println!("RES 3 (HL)");

                    let addr = cpu.reg.hl();
                    let op1 = cpu.mmu.read_u8(addr);

                    let value = cpu.alu_res(op1, 3);

                    cpu.mmu.write_u8(addr, value);

                    16
                }

                0x9f => {
                    println!("RES 3 A");

                    let value = cpu.alu_res(cpu.reg.a, 3);

                    cpu.reg.a = value;

                    8
                }

                0xa0 => {
                    println!("RES 4 B");

                    let value = cpu.alu_res(cpu.reg.b, 4);

                    cpu.reg.b = value;

                    8
                }

                0xa1 => {
                    println!("RES 4 C");

                    let value = cpu.alu_res(cpu.reg.c, 4);

                    cpu.reg.c = value;

                    8
                }

                0xa2 => {
                    println!("RES 4 D");

                    let value = cpu.alu_res(cpu.reg.d, 4);

                    cpu.reg.d = value;

                    8
                }

                0xa3 => {
                    println!("RES 4 E");

                    let value = cpu.alu_res(cpu.reg.e, 4);

                    cpu.reg.e = value;

                    8
                }

                0xa4 => {
                    println!("RES 4 H");

                    let value = cpu.alu_res(cpu.reg.h, 4);

                    cpu.reg.h = value;

                    8
                }

                0xa5 => {
                    println!("RES 4 L");

                    let value = cpu.alu_res(cpu.reg.l, 4);

                    cpu.reg.l = value;

                    8
                }

                0xa6 => {
                    println!("RES 4 (HL)");

                    let addr = cpu.reg.hl();
                    let op1 = cpu.mmu.read_u8(addr);

                    let value = cpu.alu_res(op1, 4);

                    cpu.mmu.write_u8(addr, value);

                    16
                }

                0xa7 => {
                    println!("RES 4 A");

                    let value = cpu.alu_res(cpu.reg.a, 4);

                    cpu.reg.a = value;

                    8
                }

                0xa8 => {
                    println!("RES 5 B");

                    let value = cpu.alu_res(cpu.reg.b, 5);

                    cpu.reg.b = value;

                    8
                }

                0xa9 => {
                    println!("RES 5 C");

                    let value = cpu.alu_res(cpu.reg.c, 5);

                    cpu.reg.c = value;

                    8
                }

                0xaa => {
                    println!("RES 5 D");

                    let value = cpu.alu_res(cpu.reg.d, 5);

                    cpu.reg.d = value;

                    8
                }

                0xab => {
                    println!("RES 5 E");

                    let value = cpu.alu_res(cpu.reg.e, 5);

                    cpu.reg.e = value;

                    8
                }

                0xac => {
                    println!("RES 5 H");

                    let value = cpu.alu_res(cpu.reg.h, 5);

                    cpu.reg.h = value;

                    8
                }

                0xad => {
                    println!("RES 5 L");

                    let value = cpu.alu_res(cpu.reg.l, 5);

                    cpu.reg.l = value;

                    8
                }

                0xae => {
                    println!("RES 5 (HL)");

                    let addr = cpu.reg.hl();
                    let op1 = cpu.mmu.read_u8(addr);

                    let value = cpu.alu_res(op1, 5);

                    cpu.mmu.write_u8(addr, value);

                    16
                }

                0xaf => {
                    println!("RES 5 A");

                    let value = cpu.alu_res(cpu.reg.a, 5);

                    cpu.reg.a = value;

                    8
                }

                0xb0 => {
                    println!("RES 6 B");

                    let value = cpu.alu_res(cpu.reg.b, 6);

                    cpu.reg.b = value;

                    8
                }

                0xb1 => {
                    println!("RES 6 C");

                    let value = cpu.alu_res(cpu.reg.c, 6);

                    cpu.reg.c = value;

                    8
                }

                0xb2 => {
                    println!("RES 6 D");

                    let value = cpu.alu_res(cpu.reg.d, 6);

                    cpu.reg.d = value;

                    8
                }

                0xb3 => {
                    println!("RES 6 E");

                    let value = cpu.alu_res(cpu.reg.e, 6);

                    cpu.reg.e = value;

                    8
                }

                0xb4 => {
                    println!("RES 6 H");

                    let value = cpu.alu_res(cpu.reg.h, 6);

                    cpu.reg.h = value;

                    8
                }

                0xb5 => {
                    println!("RES 6 L");

                    let value = cpu.alu_res(cpu.reg.l, 6);

                    cpu.reg.l = value;

                    8
                }

                0xb6 => {
                    println!("RES 6 (HL)");

                    let addr = cpu.reg.hl();
                    let op1 = cpu.mmu.read_u8(addr);

                    let value = cpu.alu_res(op1, 6);

                    cpu.mmu.write_u8(addr, value);

                    16
                }

                0xb7 => {
                    println!("RES 6 A");

                    let value = cpu.alu_res(cpu.reg.a, 6);

                    cpu.reg.a = value;

                    8
                }

                0xb8 => {
                    println!("RES 7 B");

                    let value = cpu.alu_res(cpu.reg.b, 7);

                    cpu.reg.b = value;

                    8
                }

                0xb9 => {
                    println!("RES 7 C");

                    let value = cpu.alu_res(cpu.reg.c, 7);

                    cpu.reg.c = value;

                    8
                }

                0xba => {
                    println!("RES 7 D");

                    let value = cpu.alu_res(cpu.reg.d, 7);

                    cpu.reg.d = value;

                    8
                }

                0xbb => {
                    println!("RES 7 E");

                    let value = cpu.alu_res(cpu.reg.e, 7);

                    cpu.reg.e = value;

                    8
                }

                0xbc => {
                    println!("RES 7 H");

                    let value = cpu.alu_res(cpu.reg.h, 7);

                    cpu.reg.h = value;

                    8
                }

                0xbd => {
                    println!("RES 7 L");

                    let value = cpu.alu_res(cpu.reg.l, 7);

                    cpu.reg.l = value;

                    8
                }

                0xbe => {
                    println!("RES 7 (HL)");

                    let addr = cpu.reg.hl();
                    let op1 = cpu.mmu.read_u8(addr);

                    let value = cpu.alu_res(op1, 7);

                    cpu.mmu.write_u8(addr, value);

                    16
                }

                0xbf => {
                    println!("RES 7 A");

                    let value = cpu.alu_res(cpu.reg.a, 7);

                    cpu.reg.a = value;

                    8
                }

                0xc0 => {
                    println!("SET 0 B");

                    let op1 = cpu.reg.b;

                    let value = cpu.alu_set(op, 0);

                    cpu.reg.b = value;

                    8
                }

                0xc1 => {
                    println!("SET 0 C");

                    let op1 = cpu.reg.c;

                    let value = cpu.alu_set(op, 0);

                    cpu.reg.c = value;

                    8
                }

                0xc2 => {
                    println!("SET 0 D");

                    let op1 = cpu.reg.d;

                    let value = cpu.alu_set(op, 0);

                    cpu.reg.d = value;

                    8
                }

                0xc3 => {
                    println!("SET 0 E");

                    let op1 = cpu.reg.e;

                    let value = cpu.alu_set(op, 0);

                    cpu.reg.e = value;

                    8
                }

                0xc4 => {
                    println!("SET 0 H");

                    let op1 = cpu.reg.h;

                    let value = cpu.alu_set(op, 0);

                    cpu.reg.h = value;

                    8
                }

                0xc5 => {
                    println!("SET 0 L");

                    let op1 = cpu.reg.l;

                    let value = cpu.alu_set(op, 0);

                    cpu.reg.l = value;

                    8
                }

                0xc6 => {
                    println!("SET 0 (HL)");

                    let addr = cpu.reg.hl();
                    let op1 = cpu.mmu.read_u8(addr);

                    let value = cpu.alu_set(op, 0);

                    cpu.mmu.write_u8(addr, value);

                    16
                }

                0xc7 => {
                    println!("SET 0 A");

                    let op1 = cpu.reg.a;

                    let value = cpu.alu_set(op, 0);

                    cpu.reg.a = value;

                    8
                }

                0xc8 => {
                    println!("SET 1 B");

                    let op1 = cpu.reg.b;

                    let value = cpu.alu_set(op, 1);

                    cpu.reg.b = value;

                    8
                }

                0xc9 => {
                    println!("SET 1 C");

                    let op1 = cpu.reg.c;

                    let value = cpu.alu_set(op, 1);

                    cpu.reg.c = value;

                    8
                }

                0xca => {
                    println!("SET 1 D");

                    let op1 = cpu.reg.d;

                    let value = cpu.alu_set(op, 1);

                    cpu.reg.d = value;

                    8
                }

                0xcb => {
                    println!("SET 1 E");

                    let op1 = cpu.reg.e;

                    let value = cpu.alu_set(op, 1);

                    cpu.reg.e = value;

                    8
                }

                0xcc => {
                    println!("SET 1 H");

                    let op1 = cpu.reg.h;

                    let value = cpu.alu_set(op, 1);

                    cpu.reg.h = value;

                    8
                }

                0xcd => {
                    println!("SET 1 L");

                    let op1 = cpu.reg.l;

                    let value = cpu.alu_set(op, 1);

                    cpu.reg.l = value;

                    8
                }

                0xce => {
                    println!("SET 1 (HL)");

                    let addr = cpu.reg.hl();
                    let op1 = cpu.mmu.read_u8(addr);

                    let value = cpu.alu_set(op, 1);

                    cpu.mmu.write_u8(addr, value);

                    16
                }

                0xcf => {
                    println!("SET 1 A");

                    let op1 = cpu.reg.a;

                    let value = cpu.alu_set(op, 1);

                    cpu.reg.a = value;

                    8
                }

                0xd0 => {
                    println!("SET 2 B");

                    let op1 = cpu.reg.b;

                    let value = cpu.alu_set(op, 2);

                    cpu.reg.b = value;

                    8
                }

                0xd1 => {
                    println!("SET 2 C");

                    let op1 = cpu.reg.c;

                    let value = cpu.alu_set(op, 2);

                    cpu.reg.c = value;

                    8
                }

                0xd2 => {
                    println!("SET 2 D");

                    let op1 = cpu.reg.d;

                    let value = cpu.alu_set(op, 2);

                    cpu.reg.d = value;

                    8
                }

                0xd3 => {
                    println!("SET 2 E");

                    let op1 = cpu.reg.e;

                    let value = cpu.alu_set(op, 2);

                    cpu.reg.e = value;

                    8
                }

                0xd4 => {
                    println!("SET 2 H");

                    let op1 = cpu.reg.h;

                    let value = cpu.alu_set(op, 2);

                    cpu.reg.h = value;

                    8
                }

                0xd5 => {
                    println!("SET 2 L");

                    let op1 = cpu.reg.l;

                    let value = cpu.alu_set(op, 2);

                    cpu.reg.l = value;

                    8
                }

                0xd6 => {
                    println!("SET 2 (HL)");

                    let addr = cpu.reg.hl();
                    let op1 = cpu.mmu.read_u8(addr);

                    let value = cpu.alu_set(op, 2);

                    cpu.mmu.write_u8(addr, value);

                    16
                }

                0xd7 => {
                    println!("SET 2 A");

                    let op1 = cpu.reg.a;

                    let value = cpu.alu_set(op, 2);

                    cpu.reg.a = value;

                    8
                }

                0xd8 => {
                    println!("SET 3 B");

                    let op1 = cpu.reg.b;

                    let value = cpu.alu_set(op, 3);

                    cpu.reg.b = value;

                    8
                }

                0xd9 => {
                    println!("SET 3 C");

                    let op1 = cpu.reg.c;

                    let value = cpu.alu_set(op, 3);

                    cpu.reg.c = value;

                    8
                }

                0xda => {
                    println!("SET 3 D");

                    let op1 = cpu.reg.d;

                    let value = cpu.alu_set(op, 3);

                    cpu.reg.d = value;

                    8
                }

                0xdb => {
                    println!("SET 3 E");

                    let op1 = cpu.reg.e;

                    let value = cpu.alu_set(op, 3);

                    cpu.reg.e = value;

                    8
                }

                0xdc => {
                    println!("SET 3 H");

                    let op1 = cpu.reg.h;

                    let value = cpu.alu_set(op, 3);

                    cpu.reg.h = value;

                    8
                }

                0xdd => {
                    println!("SET 3 L");

                    let op1 = cpu.reg.l;

                    let value = cpu.alu_set(op, 3);

                    cpu.reg.l = value;

                    8
                }

                0xde => {
                    println!("SET 3 (HL)");

                    let addr = cpu.reg.hl();
                    let op1 = cpu.mmu.read_u8(addr);

                    let value = cpu.alu_set(op, 3);

                    cpu.mmu.write_u8(addr, value);

                    16
                }

                0xdf => {
                    println!("SET 3 A");

                    let op1 = cpu.reg.a;

                    let value = cpu.alu_set(op, 3);

                    cpu.reg.a = value;

                    8
                }

                0xe0 => {
                    println!("SET 4 B");

                    let op1 = cpu.reg.b;

                    let value = cpu.alu_set(op, 4);

                    cpu.reg.b = value;

                    8
                }

                0xe1 => {
                    println!("SET 4 C");

                    let op1 = cpu.reg.c;

                    let value = cpu.alu_set(op, 4);

                    cpu.reg.c = value;

                    8
                }

                0xe2 => {
                    println!("SET 4 D");

                    let op1 = cpu.reg.d;

                    let value = cpu.alu_set(op, 4);

                    cpu.reg.d = value;

                    8
                }

                0xe3 => {
                    println!("SET 4 E");

                    let op1 = cpu.reg.e;

                    let value = cpu.alu_set(op, 4);

                    cpu.reg.e = value;

                    8
                }

                0xe4 => {
                    println!("SET 4 H");

                    let op1 = cpu.reg.h;

                    let value = cpu.alu_set(op, 4);

                    cpu.reg.h = value;

                    8
                }

                0xe5 => {
                    println!("SET 4 L");

                    let op1 = cpu.reg.l;

                    let value = cpu.alu_set(op, 4);

                    cpu.reg.l = value;

                    8
                }

                0xe6 => {
                    println!("SET 4 (HL)");

                    let addr = cpu.reg.hl();
                    let op1 = cpu.mmu.read_u8(addr);

                    let value = cpu.alu_set(op, 4);

                    cpu.mmu.write_u8(addr, value);

                    16
                }

                0xe7 => {
                    println!("SET 4 A");

                    let op1 = cpu.reg.a;

                    let value = cpu.alu_set(op, 4);

                    cpu.reg.a = value;

                    8
                }

                0xe8 => {
                    println!("SET 5 B");

                    let op1 = cpu.reg.b;

                    let value = cpu.alu_set(op, 5);

                    cpu.reg.b = value;

                    8
                }

                0xe9 => {
                    println!("SET 5 C");

                    let op1 = cpu.reg.c;

                    let value = cpu.alu_set(op, 5);

                    cpu.reg.c = value;

                    8
                }

                0xea => {
                    println!("SET 5 D");

                    let op1 = cpu.reg.d;

                    let value = cpu.alu_set(op, 5);

                    cpu.reg.d = value;

                    8
                }

                0xeb => {
                    println!("SET 5 E");

                    let op1 = cpu.reg.e;

                    let value = cpu.alu_set(op, 5);

                    cpu.reg.e = value;

                    8
                }

                0xec => {
                    println!("SET 5 H");

                    let op1 = cpu.reg.h;

                    let value = cpu.alu_set(op, 5);

                    cpu.reg.h = value;

                    8
                }

                0xed => {
                    println!("SET 5 L");

                    let op1 = cpu.reg.l;

                    let value = cpu.alu_set(op, 5);

                    cpu.reg.l = value;

                    8
                }

                0xee => {
                    println!("SET 5 (HL)");

                    let addr = cpu.reg.hl();
                    let op1 = cpu.mmu.read_u8(addr);

                    let value = cpu.alu_set(op, 5);

                    cpu.mmu.write_u8(addr, value);

                    16
                }

                0xef => {
                    println!("SET 5 A");

                    let op1 = cpu.reg.a;

                    let value = cpu.alu_set(op, 5);

                    cpu.reg.a = value;

                    8
                }

                0xf0 => {
                    println!("SET 6 B");

                    let op1 = cpu.reg.b;

                    let value = cpu.alu_set(op, 6);

                    cpu.reg.b = value;

                    8
                }

                0xf1 => {
                    println!("SET 6 C");

                    let op1 = cpu.reg.c;

                    let value = cpu.alu_set(op, 6);

                    cpu.reg.c = value;

                    8
                }

                0xf2 => {
                    println!("SET 6 D");

                    let op1 = cpu.reg.d;

                    let value = cpu.alu_set(op, 6);

                    cpu.reg.d = value;

                    8
                }

                0xf3 => {
                    println!("SET 6 E");

                    let op1 = cpu.reg.e;

                    let value = cpu.alu_set(op, 6);

                    cpu.reg.e = value;

                    8
                }

                0xf4 => {
                    println!("SET 6 H");

                    let op1 = cpu.reg.h;

                    let value = cpu.alu_set(op, 6);

                    cpu.reg.h = value;

                    8
                }

                0xf5 => {
                    println!("SET 6 L");

                    let op1 = cpu.reg.l;

                    let value = cpu.alu_set(op, 6);

                    cpu.reg.l = value;

                    8
                }

                0xf6 => {
                    println!("SET 6 (HL)");

                    let addr = cpu.reg.hl();
                    let op1 = cpu.mmu.read_u8(addr);

                    let value = cpu.alu_set(op, 6);

                    cpu.mmu.write_u8(addr, value);

                    16
                }

                0xf7 => {
                    println!("SET 6 A");

                    let op1 = cpu.reg.a;

                    let value = cpu.alu_set(op, 6);

                    cpu.reg.a = value;

                    8
                }

                0xf8 => {
                    println!("SET 7 B");

                    let op1 = cpu.reg.b;

                    let value = cpu.alu_set(op, 7);

                    cpu.reg.b = value;

                    8
                }

                0xf9 => {
                    println!("SET 7 C");

                    let op1 = cpu.reg.c;

                    let value = cpu.alu_set(op, 7);

                    cpu.reg.c = value;

                    8
                }

                0xfa => {
                    println!("SET 7 D");

                    let op1 = cpu.reg.d;

                    let value = cpu.alu_set(op, 7);

                    cpu.reg.d = value;

                    8
                }

                0xfb => {
                    println!("SET 7 E");

                    let op1 = cpu.reg.e;

                    let value = cpu.alu_set(op, 7);

                    cpu.reg.e = value;

                    8
                }

                0xfc => {
                    println!("SET 7 H");

                    let op1 = cpu.reg.h;

                    let value = cpu.alu_set(op, 7);

                    cpu.reg.h = value;

                    8
                }

                0xfd => {
                    println!("SET 7 L");

                    let op1 = cpu.reg.l;

                    let value = cpu.alu_set(op, 7);

                    cpu.reg.l = value;

                    8
                }

                0xfe => {
                    println!("SET 7 (HL)");

                    let addr = cpu.reg.hl();
                    let op1 = cpu.mmu.read_u8(addr);

                    let value = cpu.alu_set(op, 7);

                    cpu.mmu.write_u8(addr, value);

                    16
                }

                0xff => {
                    println!("SET 7 A");

                    let op1 = cpu.reg.a;

                    let value = cpu.alu_set(op, 7);

                    cpu.reg.a = value;

                    8
                }
            }
        }
    }
}
