use crate::cpu::Cpu;

pub fn exec_opcode(cpu: &mut Cpu) -> (u8, u8) {
  let opcode = emu.read_opcode();

  match opcode {
    0x00 => {
        println!("NOP  ");
        (1, 4)
    }
    0x01 => {
        println!("LD BC d16");
        let addr = cpu.registers.bc();
        let value = cpu.read_nn();

        cpu.mmu.write(addr, value);

        (3, 12)
    }
    0x02 => {
        println!("LD (BC) A");
        let addr = cpu.memory.read_u16(cpu.registers.bc());
        let value = cpu.registers.a;

        cpu.mmu.write(addr, value);

        (1, 8)
    }
    0x03 => {
        println!("INC BC ");
        let value = cpu.registers.bc();

        cpu.registers.set_bc(value + 1);

        (1, 8)
    }
    0x04 => {
        println!("INC B ");
        let value = cpu.registers.b();

        cpu.registers.set_b(value + 1);

        (1, 4)
    }
    0x05 => {
        println!("DEC B ");
        let value = cpu.registers.b();

        cpu.registers.set_b(value - 1);

        (1, 4)
    }
    0x06 => {
        println!("LD B d8");
        let value = cpu.read_n();

        cpu.registers.set_b(value);

        (2, 8)
    }
    0x07 => {
        println!("RLCA  ");

        //  TODO

        (1, 4)
    }
    0x08 => {
        println!("LD (a16) SP");
        let addr = cpu.read_nn();
        let value = cpu.sp;

        cpu.mmu.write(addr, value);

        (3, 20)
    }
    0x09 => {
        println!("ADD HL BC");
        let value = cpu.registers.hl() + cpu.registers.bc();

        //  TODO: set flags
        cpu.registers.set_hl(value);

        (1, 8)
    }
    0x0a => {
        println!("LD A (BC)");
        let value = cpu.mmu.read_u8(cpu.registers.bc());

        cpu.registers.set_a(value);

        (1, 8)
    }
    0x0b => {
        println!("DEC BC ");
        let value = cpu.registers.bc();

        cpu.registers.set_bc(value - 1);

        (1, 8)
    }
    0x0c => {
        println!("INC C ");
        let value = cpu.registers.c();

        cpu.registers.set_c(value + 1);

        (1, 4)
    }
    0x0d => {
        println!("DEC C ");
        let value = cpu.registers.c();

        cpu.registers.set_c(value - 1);

        (1, 4)
    }
    0x0e => {
        println!("LD C d8");
        (2, 8)
    }
    0x0f => {
        println!("RRCA  ");
        (1, 4)
    }
    0x10 => {
        println!("STOP 0 ");
        (1, 4)
    }
    0x11 => {
        println!("LD DE d16");
        (3, 12)
    }
    0x12 => {
        println!("LD (DE) A");
        (1, 8)
    }
    0x13 => {
        println!("INC DE ");
        (1, 8)
    }
    0x14 => {
        println!("INC D ");
        (1, 4)
    }
    0x15 => {
        println!("DEC D ");
        (1, 4)
    }
    0x16 => {
        println!("LD D d8");
        (2, 8)
    }
    0x17 => {
        println!("RLA  ");
        (1, 4)
    }
    0x18 => {
        println!("JR r8 ");
        (2, 12)
    }
    0x19 => {
        println!("ADD HL DE");
        (1, 8)
    }
    0x1a => {
        println!("LD A (DE)");
        (1, 8)
    }
    0x1b => {
        println!("DEC DE ");
        (1, 8)
    }
    0x1c => {
        println!("INC E ");
        (1, 4)
    }
    0x1d => {
        println!("DEC E ");
        (1, 4)
    }
    0x1e => {
        println!("LD E d8");
        (2, 8)
    }
    0x1f => {
        println!("RRA  ");
        (1, 4)
    }
    0x20 => {
        println!("JR NZ r8");
        (2, 12)
    }
    0x21 => {
        println!("LD HL d16");
        (3, 12)
    }
    0x22 => {
        println!("LD (HL+) A");
        (1, 8)
    }
    0x23 => {
        println!("INC HL ");
        (1, 8)
    }
    0x24 => {
        println!("INC H ");
        (1, 4)
    }
    0x25 => {
        println!("DEC H ");
        (1, 4)
    }
    0x26 => {
        println!("LD H d8");
        (2, 8)
    }
    0x27 => {
        println!("DAA  ");
        (1, 4)
    }
    0x28 => {
        println!("JR Z r8");
        (2, 12)
    }
    0x29 => {
        println!("ADD HL HL");
        (1, 8)
    }
    0x2a => {
        println!("LD A (HL+)");
        (1, 8)
    }
    0x2b => {
        println!("DEC HL ");
        (1, 8)
    }
    0x2c => {
        println!("INC L ");
        (1, 4)
    }
    0x2d => {
        println!("DEC L ");
        (1, 4)
    }
    0x2e => {
        println!("LD L d8");
        (2, 8)
    }
    0x2f => {
        println!("CPL  ");
        (1, 4)
    }
    0x30 => {
        println!("JR NC r8");
        (2, 12)
    }
    0x31 => {
        println!("LD SP d16");
        (3, 12)
    }
    0x32 => {
        println!("LD (HL-) A");
        (1, 8)
    }
    0x33 => {
        println!("INC SP ");
        (1, 8)
    }
    0x34 => {
        println!("INC (HL) ");
        (1, 12)
    }
    0x35 => {
        println!("DEC (HL) ");
        (1, 12)
    }
    0x36 => {
        println!("LD (HL) d8");
        (2, 12)
    }
    0x37 => {
        println!("SCF  ");
        (1, 4)
    }
    0x38 => {
        println!("JR C r8");
        (2, 12)
    }
    0x39 => {
        println!("ADD HL SP");
        (1, 8)
    }
    0x3a => {
        println!("LD A (HL-)");
        (1, 8)
    }
    0x3b => {
        println!("DEC SP ");
        (1, 8)
    }
    0x3c => {
        println!("INC A ");
        (1, 4)
    }
    0x3d => {
        println!("DEC A ");
        (1, 4)
    }
    0x3e => {
        println!("LD A d8");
        (2, 8)
    }
    0x3f => {
        println!("CCF  ");
        (1, 4)
    }
    0x40 => {
        println!("LD B B");
        (1, 4)
    }
    0x41 => {
        println!("LD B C");
        (1, 4)
    }
    0x42 => {
        println!("LD B D");
        (1, 4)
    }
    0x43 => {
        println!("LD B E");
        (1, 4)
    }
    0x44 => {
        println!("LD B H");
        (1, 4)
    }
    0x45 => {
        println!("LD B L");
        (1, 4)
    }
    0x46 => {
        println!("LD B (HL)");
        (1, 8)
    }
    0x47 => {
        println!("LD B A");
        (1, 4)
    }
    0x48 => {
        println!("LD C B");
        (1, 4)
    }
    0x49 => {
        println!("LD C C");
        (1, 4)
    }
    0x4a => {
        println!("LD C D");
        (1, 4)
    }
    0x4b => {
        println!("LD C E");
        (1, 4)
    }
    0x4c => {
        println!("LD C H");
        (1, 4)
    }
    0x4d => {
        println!("LD C L");
        (1, 4)
    }
    0x4e => {
        println!("LD C (HL)");
        (1, 8)
    }
    0x4f => {
        println!("LD C A");
        (1, 4)
    }
    0x50 => {
        println!("LD D B");
        (1, 4)
    }
    0x51 => {
        println!("LD D C");
        (1, 4)
    }
    0x52 => {
        println!("LD D D");
        (1, 4)
    }
    0x53 => {
        println!("LD D E");
        (1, 4)
    }
    0x54 => {
        println!("LD D H");
        (1, 4)
    }
    0x55 => {
        println!("LD D L");
        (1, 4)
    }
    0x56 => {
        println!("LD D (HL)");
        (1, 8)
    }
    0x57 => {
        println!("LD D A");
        (1, 4)
    }
    0x58 => {
        println!("LD E B");
        (1, 4)
    }
    0x59 => {
        println!("LD E C");
        (1, 4)
    }
    0x5a => {
        println!("LD E D");
        (1, 4)
    }
    0x5b => {
        println!("LD E E");
        (1, 4)
    }
    0x5c => {
        println!("LD E H");
        (1, 4)
    }
    0x5d => {
        println!("LD E L");
        (1, 4)
    }
    0x5e => {
        println!("LD E (HL)");
        (1, 8)
    }
    0x5f => {
        println!("LD E A");
        (1, 4)
    }
    0x60 => {
        println!("LD H B");
        (1, 4)
    }
    0x61 => {
        println!("LD H C");
        (1, 4)
    }
    0x62 => {
        println!("LD H D");
        (1, 4)
    }
    0x63 => {
        println!("LD H E");
        (1, 4)
    }
    0x64 => {
        println!("LD H H");
        (1, 4)
    }
    0x65 => {
        println!("LD H L");
        (1, 4)
    }
    0x66 => {
        println!("LD H (HL)");
        (1, 8)
    }
    0x67 => {
        println!("LD H A");
        (1, 4)
    }
    0x68 => {
        println!("LD L B");
        (1, 4)
    }
    0x69 => {
        println!("LD L C");
        (1, 4)
    }
    0x6a => {
        println!("LD L D");
        (1, 4)
    }
    0x6b => {
        println!("LD L E");
        (1, 4)
    }
    0x6c => {
        println!("LD L H");
        (1, 4)
    }
    0x6d => {
        println!("LD L L");
        (1, 4)
    }
    0x6e => {
        println!("LD L (HL)");
        (1, 8)
    }
    0x6f => {
        println!("LD L A");
        (1, 4)
    }
    0x70 => {
        println!("LD (HL) B");
        (1, 8)
    }
    0x71 => {
        println!("LD (HL) C");
        (1, 8)
    }
    0x72 => {
        println!("LD (HL) D");
        (1, 8)
    }
    0x73 => {
        println!("LD (HL) E");
        (1, 8)
    }
    0x74 => {
        println!("LD (HL) H");
        (1, 8)
    }
    0x75 => {
        println!("LD (HL) L");
        (1, 8)
    }
    0x76 => {
        println!("HALT  ");
        (1, 4)
    }
    0x77 => {
        println!("LD (HL) A");
        (1, 8)
    }
    0x78 => {
        println!("LD A B");
        (1, 4)
    }
    0x79 => {
        println!("LD A C");
        (1, 4)
    }
    0x7a => {
        println!("LD A D");
        (1, 4)
    }
    0x7b => {
        println!("LD A E");
        (1, 4)
    }
    0x7c => {
        println!("LD A H");
        (1, 4)
    }
    0x7d => {
        println!("LD A L");
        (1, 4)
    }
    0x7e => {
        println!("LD A (HL)");
        (1, 8)
    }
    0x7f => {
        println!("LD A A");
        (1, 4)
    }
    0x80 => {
        println!("ADD A B");
        (1, 4)
    }
    0x81 => {
        println!("ADD A C");
        (1, 4)
    }
    0x82 => {
        println!("ADD A D");
        (1, 4)
    }
    0x83 => {
        println!("ADD A E");
        (1, 4)
    }
    0x84 => {
        println!("ADD A H");
        (1, 4)
    }
    0x85 => {
        println!("ADD A L");
        (1, 4)
    }
    0x86 => {
        println!("ADD A (HL)");
        (1, 8)
    }
    0x87 => {
        println!("ADD A A");
        (1, 4)
    }
    0x88 => {
        println!("ADC A B");
        (1, 4)
    }
    0x89 => {
        println!("ADC A C");
        (1, 4)
    }
    0x8a => {
        println!("ADC A D");
        (1, 4)
    }
    0x8b => {
        println!("ADC A E");
        (1, 4)
    }
    0x8c => {
        println!("ADC A H");
        (1, 4)
    }
    0x8d => {
        println!("ADC A L");
        (1, 4)
    }
    0x8e => {
        println!("ADC A (HL)");
        (1, 8)
    }
    0x8f => {
        println!("ADC A A");
        (1, 4)
    }
    0x90 => {
        println!("SUB B ");
        (1, 4)
    }
    0x91 => {
        println!("SUB C ");
        (1, 4)
    }
    0x92 => {
        println!("SUB D ");
        (1, 4)
    }
    0x93 => {
        println!("SUB E ");
        (1, 4)
    }
    0x94 => {
        println!("SUB H ");
        (1, 4)
    }
    0x95 => {
        println!("SUB L ");
        (1, 4)
    }
    0x96 => {
        println!("SUB (HL) ");
        (1, 8)
    }
    0x97 => {
        println!("SUB A ");
        (1, 4)
    }
    0x98 => {
        println!("SBC A B");
        (1, 4)
    }
    0x99 => {
        println!("SBC A C");
        (1, 4)
    }
    0x9a => {
        println!("SBC A D");
        (1, 4)
    }
    0x9b => {
        println!("SBC A E");
        (1, 4)
    }
    0x9c => {
        println!("SBC A H");
        (1, 4)
    }
    0x9d => {
        println!("SBC A L");
        (1, 4)
    }
    0x9e => {
        println!("SBC A (HL)");
        (1, 8)
    }
    0x9f => {
        println!("SBC A A");
        (1, 4)
    }
    0xa0 => {
        println!("AND B ");
        (1, 4)
    }
    0xa1 => {
        println!("AND C ");
        (1, 4)
    }
    0xa2 => {
        println!("AND D ");
        (1, 4)
    }
    0xa3 => {
        println!("AND E ");
        (1, 4)
    }
    0xa4 => {
        println!("AND H ");
        (1, 4)
    }
    0xa5 => {
        println!("AND L ");
        (1, 4)
    }
    0xa6 => {
        println!("AND (HL) ");
        (1, 8)
    }
    0xa7 => {
        println!("AND A ");
        (1, 4)
    }
    0xa8 => {
        println!("XOR B ");
        (1, 4)
    }
    0xa9 => {
        println!("XOR C ");
        (1, 4)
    }
    0xaa => {
        println!("XOR D ");
        (1, 4)
    }
    0xab => {
        println!("XOR E ");
        (1, 4)
    }
    0xac => {
        println!("XOR H ");
        (1, 4)
    }
    0xad => {
        println!("XOR L ");
        (1, 4)
    }
    0xae => {
        println!("XOR (HL) ");
        (1, 8)
    }
    0xaf => {
        println!("XOR A ");
        (1, 4)
    }
    0xb0 => {
        println!("OR B ");
        (1, 4)
    }
    0xb1 => {
        println!("OR C ");
        (1, 4)
    }
    0xb2 => {
        println!("OR D ");
        (1, 4)
    }
    0xb3 => {
        println!("OR E ");
        (1, 4)
    }
    0xb4 => {
        println!("OR H ");
        (1, 4)
    }
    0xb5 => {
        println!("OR L ");
        (1, 4)
    }
    0xb6 => {
        println!("OR (HL) ");
        (1, 8)
    }
    0xb7 => {
        println!("OR A ");
        (1, 4)
    }
    0xb8 => {
        println!("CP B ");
        (1, 4)
    }
    0xb9 => {
        println!("CP C ");
        (1, 4)
    }
    0xba => {
        println!("CP D ");
        (1, 4)
    }
    0xbb => {
        println!("CP E ");
        (1, 4)
    }
    0xbc => {
        println!("CP H ");
        (1, 4)
    }
    0xbd => {
        println!("CP L ");
        (1, 4)
    }
    0xbe => {
        println!("CP (HL) ");
        (1, 8)
    }
    0xbf => {
        println!("CP A ");
        (1, 4)
    }
    0xc0 => {
        println!("RET NZ ");
        (1, 20)
    }
    0xc1 => {
        println!("POP BC ");
        (1, 12)
    }
    0xc2 => {
        println!("JP NZ a16");
        (3, 16)
    }
    0xc3 => {
        println!("JP a16 ");
        (3, 16)
    }
    0xc4 => {
        println!("CALL NZ a16");
        (3, 24)
    }
    0xc5 => {
        println!("PUSH BC ");
        (1, 16)
    }
    0xc6 => {
        println!("ADD A d8");
        (2, 8)
    }
    0xc7 => {
        println!("RST 00H ");
        (1, 16)
    }
    0xc8 => {
        println!("RET Z ");
        (1, 20)
    }
    0xc9 => {
        println!("RET  ");
        (1, 16)
    }
    0xca => {
        println!("JP Z a16");
        (3, 16)
    }
    0xcb => {
        let cb_opcode = emu.next_opcode();
        match cb_opcode {
            0x00 => {
                println!("RLC B ");
                (2, 4)
            }
            0x01 => {
                println!("RLC C ");
                (2, 4)
            }
            0x02 => {
                println!("RLC D ");
                (2, 4)
            }
            0x03 => {
                println!("RLC E ");
                (2, 4)
            }
            0x04 => {
                println!("RLC H ");
                (2, 4)
            }
            0x05 => {
                println!("RLC L ");
                (2, 4)
            }
            0x06 => {
                println!("RLC (HL) ");
                (2, 4)
            }
            0x07 => {
                println!("RLC A ");
                (2, 4)
            }
            0x08 => {
                println!("RRC B ");
                (2, 4)
            }
            0x09 => {
                println!("RRC C ");
                (2, 4)
            }
            0x0a => {
                println!("RRC D ");
                (2, 4)
            }
            0x0b => {
                println!("RRC E ");
                (2, 4)
            }
            0x0c => {
                println!("RRC H ");
                (2, 4)
            }
            0x0d => {
                println!("RRC L ");
                (2, 4)
            }
            0x0e => {
                println!("RRC (HL) ");
                (2, 4)
            }
            0x0f => {
                println!("RRC A ");
                (2, 4)
            }
            0x10 => {
                println!("RL B ");
                (2, 4)
            }
            0x11 => {
                println!("RL C ");
                (2, 4)
            }
            0x12 => {
                println!("RL D ");
                (2, 4)
            }
            0x13 => {
                println!("RL E ");
                (2, 4)
            }
            0x14 => {
                println!("RL H ");
                (2, 4)
            }
            0x15 => {
                println!("RL L ");
                (2, 4)
            }
            0x16 => {
                println!("RL (HL) ");
                (2, 4)
            }
            0x17 => {
                println!("RL A ");
                (2, 4)
            }
            0x18 => {
                println!("RR B ");
                (2, 4)
            }
            0x19 => {
                println!("RR C ");
                (2, 4)
            }
            0x1a => {
                println!("RR D ");
                (2, 4)
            }
            0x1b => {
                println!("RR E ");
                (2, 4)
            }
            0x1c => {
                println!("RR H ");
                (2, 4)
            }
            0x1d => {
                println!("RR L ");
                (2, 4)
            }
            0x1e => {
                println!("RR (HL) ");
                (2, 4)
            }
            0x1f => {
                println!("RR A ");
                (2, 4)
            }
            0x20 => {
                println!("SLA B ");
                (2, 4)
            }
            0x21 => {
                println!("SLA C ");
                (2, 4)
            }
            0x22 => {
                println!("SLA D ");
                (2, 4)
            }
            0x23 => {
                println!("SLA E ");
                (2, 4)
            }
            0x24 => {
                println!("SLA H ");
                (2, 4)
            }
            0x25 => {
                println!("SLA L ");
                (2, 4)
            }
            0x26 => {
                println!("SLA (HL) ");
                (2, 4)
            }
            0x27 => {
                println!("SLA A ");
                (2, 4)
            }
            0x28 => {
                println!("SRA B ");
                (2, 4)
            }
            0x29 => {
                println!("SRA C ");
                (2, 4)
            }
            0x2a => {
                println!("SRA D ");
                (2, 4)
            }
            0x2b => {
                println!("SRA E ");
                (2, 4)
            }
            0x2c => {
                println!("SRA H ");
                (2, 4)
            }
            0x2d => {
                println!("SRA L ");
                (2, 4)
            }
            0x2e => {
                println!("SRA (HL) ");
                (2, 4)
            }
            0x2f => {
                println!("SRA A ");
                (2, 4)
            }
            0x30 => {
                println!("SWAP B ");
                (2, 4)
            }
            0x31 => {
                println!("SWAP C ");
                (2, 4)
            }
            0x32 => {
                println!("SWAP D ");
                (2, 4)
            }
            0x33 => {
                println!("SWAP E ");
                (2, 4)
            }
            0x34 => {
                println!("SWAP H ");
                (2, 4)
            }
            0x35 => {
                println!("SWAP L ");
                (2, 4)
            }
            0x36 => {
                println!("SWAP (HL) ");
                (2, 4)
            }
            0x37 => {
                println!("SWAP A ");
                (2, 4)
            }
            0x38 => {
                println!("SRL B ");
                (2, 4)
            }
            0x39 => {
                println!("SRL C ");
                (2, 4)
            }
            0x3a => {
                println!("SRL D ");
                (2, 4)
            }
            0x3b => {
                println!("SRL E ");
                (2, 4)
            }
            0x3c => {
                println!("SRL H ");
                (2, 4)
            }
            0x3d => {
                println!("SRL L ");
                (2, 4)
            }
            0x3e => {
                println!("SRL (HL) ");
                (2, 4)
            }
            0x3f => {
                println!("SRL A ");
                (2, 4)
            }
            0x40 => {
                println!("BIT 0 B");
                (2, 4)
            }
            0x41 => {
                println!("BIT 0 C");
                (2, 4)
            }
            0x42 => {
                println!("BIT 0 D");
                (2, 4)
            }
            0x43 => {
                println!("BIT 0 E");
                (2, 4)
            }
            0x44 => {
                println!("BIT 0 H");
                (2, 4)
            }
            0x45 => {
                println!("BIT 0 L");
                (2, 4)
            }
            0x46 => {
                println!("BIT 0 (HL)");
                (2, 4)
            }
            0x47 => {
                println!("BIT 0 A");
                (2, 4)
            }
            0x48 => {
                println!("BIT 1 B");
                (2, 4)
            }
            0x49 => {
                println!("BIT 1 C");
                (2, 4)
            }
            0x4a => {
                println!("BIT 1 D");
                (2, 4)
            }
            0x4b => {
                println!("BIT 1 E");
                (2, 4)
            }
            0x4c => {
                println!("BIT 1 H");
                (2, 4)
            }
            0x4d => {
                println!("BIT 1 L");
                (2, 4)
            }
            0x4e => {
                println!("BIT 1 (HL)");
                (2, 4)
            }
            0x4f => {
                println!("BIT 1 A");
                (2, 4)
            }
            0x50 => {
                println!("BIT 2 B");
                (2, 4)
            }
            0x51 => {
                println!("BIT 2 C");
                (2, 4)
            }
            0x52 => {
                println!("BIT 2 D");
                (2, 4)
            }
            0x53 => {
                println!("BIT 2 E");
                (2, 4)
            }
            0x54 => {
                println!("BIT 2 H");
                (2, 4)
            }
            0x55 => {
                println!("BIT 2 L");
                (2, 4)
            }
            0x56 => {
                println!("BIT 2 (HL)");
                (2, 4)
            }
            0x57 => {
                println!("BIT 2 A");
                (2, 4)
            }
            0x58 => {
                println!("BIT 3 B");
                (2, 4)
            }
            0x59 => {
                println!("BIT 3 C");
                (2, 4)
            }
            0x5a => {
                println!("BIT 3 D");
                (2, 4)
            }
            0x5b => {
                println!("BIT 3 E");
                (2, 4)
            }
            0x5c => {
                println!("BIT 3 H");
                (2, 4)
            }
            0x5d => {
                println!("BIT 3 L");
                (2, 4)
            }
            0x5e => {
                println!("BIT 3 (HL)");
                (2, 4)
            }
            0x5f => {
                println!("BIT 3 A");
                (2, 4)
            }
            0x60 => {
                println!("BIT 4 B");
                (2, 4)
            }
            0x61 => {
                println!("BIT 4 C");
                (2, 4)
            }
            0x62 => {
                println!("BIT 4 D");
                (2, 4)
            }
            0x63 => {
                println!("BIT 4 E");
                (2, 4)
            }
            0x64 => {
                println!("BIT 4 H");
                (2, 4)
            }
            0x65 => {
                println!("BIT 4 L");
                (2, 4)
            }
            0x66 => {
                println!("BIT 4 (HL)");
                (2, 4)
            }
            0x67 => {
                println!("BIT 4 A");
                (2, 4)
            }
            0x68 => {
                println!("BIT 5 B");
                (2, 4)
            }
            0x69 => {
                println!("BIT 5 C");
                (2, 4)
            }
            0x6a => {
                println!("BIT 5 D");
                (2, 4)
            }
            0x6b => {
                println!("BIT 5 E");
                (2, 4)
            }
            0x6c => {
                println!("BIT 5 H");
                (2, 4)
            }
            0x6d => {
                println!("BIT 5 L");
                (2, 4)
            }
            0x6e => {
                println!("BIT 5 (HL)");
                (2, 4)
            }
            0x6f => {
                println!("BIT 5 A");
                (2, 4)
            }
            0x70 => {
                println!("BIT 6 B");
                (2, 4)
            }
            0x71 => {
                println!("BIT 6 C");
                (2, 4)
            }
            0x72 => {
                println!("BIT 6 D");
                (2, 4)
            }
            0x73 => {
                println!("BIT 6 E");
                (2, 4)
            }
            0x74 => {
                println!("BIT 6 H");
                (2, 4)
            }
            0x75 => {
                println!("BIT 6 L");
                (2, 4)
            }
            0x76 => {
                println!("BIT 6 (HL)");
                (2, 4)
            }
            0x77 => {
                println!("BIT 6 A");
                (2, 4)
            }
            0x78 => {
                println!("BIT 7 B");
                (2, 4)
            }
            0x79 => {
                println!("BIT 7 C");
                (2, 4)
            }
            0x7a => {
                println!("BIT 7 D");
                (2, 4)
            }
            0x7b => {
                println!("BIT 7 E");
                (2, 4)
            }
            0x7c => {
                println!("BIT 7 H");
                (2, 4)
            }
            0x7d => {
                println!("BIT 7 L");
                (2, 4)
            }
            0x7e => {
                println!("BIT 7 (HL)");
                (2, 4)
            }
            0x7f => {
                println!("BIT 7 A");
                (2, 4)
            }
            0x80 => {
                println!("RES 0 B");
                (2, 4)
            }
            0x81 => {
                println!("RES 0 C");
                (2, 4)
            }
            0x82 => {
                println!("RES 0 D");
                (2, 4)
            }
            0x83 => {
                println!("RES 0 E");
                (2, 4)
            }
            0x84 => {
                println!("RES 0 H");
                (2, 4)
            }
            0x85 => {
                println!("RES 0 L");
                (2, 4)
            }
            0x86 => {
                println!("RES 0 (HL)");
                (2, 4)
            }
            0x87 => {
                println!("RES 0 A");
                (2, 4)
            }
            0x88 => {
                println!("RES 1 B");
                (2, 4)
            }
            0x89 => {
                println!("RES 1 C");
                (2, 4)
            }
            0x8a => {
                println!("RES 1 D");
                (2, 4)
            }
            0x8b => {
                println!("RES 1 E");
                (2, 4)
            }
            0x8c => {
                println!("RES 1 H");
                (2, 4)
            }
            0x8d => {
                println!("RES 1 L");
                (2, 4)
            }
            0x8e => {
                println!("RES 1 (HL)");
                (2, 4)
            }
            0x8f => {
                println!("RES 1 A");
                (2, 4)
            }
            0x90 => {
                println!("RES 2 B");
                (2, 4)
            }
            0x91 => {
                println!("RES 2 C");
                (2, 4)
            }
            0x92 => {
                println!("RES 2 D");
                (2, 4)
            }
            0x93 => {
                println!("RES 2 E");
                (2, 4)
            }
            0x94 => {
                println!("RES 2 H");
                (2, 4)
            }
            0x95 => {
                println!("RES 2 L");
                (2, 4)
            }
            0x96 => {
                println!("RES 2 (HL)");
                (2, 4)
            }
            0x97 => {
                println!("RES 2 A");
                (2, 4)
            }
            0x98 => {
                println!("RES 3 B");
                (2, 4)
            }
            0x99 => {
                println!("RES 3 C");
                (2, 4)
            }
            0x9a => {
                println!("RES 3 D");
                (2, 4)
            }
            0x9b => {
                println!("RES 3 E");
                (2, 4)
            }
            0x9c => {
                println!("RES 3 H");
                (2, 4)
            }
            0x9d => {
                println!("RES 3 L");
                (2, 4)
            }
            0x9e => {
                println!("RES 3 (HL)");
                (2, 4)
            }
            0x9f => {
                println!("RES 3 A");
                (2, 4)
            }
            0xa0 => {
                println!("RES 4 B");
                (2, 4)
            }
            0xa1 => {
                println!("RES 4 C");
                (2, 4)
            }
            0xa2 => {
                println!("RES 4 D");
                (2, 4)
            }
            0xa3 => {
                println!("RES 4 E");
                (2, 4)
            }
            0xa4 => {
                println!("RES 4 H");
                (2, 4)
            }
            0xa5 => {
                println!("RES 4 L");
                (2, 4)
            }
            0xa6 => {
                println!("RES 4 (HL)");
                (2, 4)
            }
            0xa7 => {
                println!("RES 4 A");
                (2, 4)
            }
            0xa8 => {
                println!("RES 5 B");
                (2, 4)
            }
            0xa9 => {
                println!("RES 5 C");
                (2, 4)
            }
            0xaa => {
                println!("RES 5 D");
                (2, 4)
            }
            0xab => {
                println!("RES 5 E");
                (2, 4)
            }
            0xac => {
                println!("RES 5 H");
                (2, 4)
            }
            0xad => {
                println!("RES 5 L");
                (2, 4)
            }
            0xae => {
                println!("RES 5 (HL)");
                (2, 4)
            }
            0xaf => {
                println!("RES 5 A");
                (2, 4)
            }
            0xb0 => {
                println!("RES 6 B");
                (2, 4)
            }
            0xb1 => {
                println!("RES 6 C");
                (2, 4)
            }
            0xb2 => {
                println!("RES 6 D");
                (2, 4)
            }
            0xb3 => {
                println!("RES 6 E");
                (2, 4)
            }
            0xb4 => {
                println!("RES 6 H");
                (2, 4)
            }
            0xb5 => {
                println!("RES 6 L");
                (2, 4)
            }
            0xb6 => {
                println!("RES 6 (HL)");
                (2, 4)
            }
            0xb7 => {
                println!("RES 6 A");
                (2, 4)
            }
            0xb8 => {
                println!("RES 7 B");
                (2, 4)
            }
            0xb9 => {
                println!("RES 7 C");
                (2, 4)
            }
            0xba => {
                println!("RES 7 D");
                (2, 4)
            }
            0xbb => {
                println!("RES 7 E");
                (2, 4)
            }
            0xbc => {
                println!("RES 7 H");
                (2, 4)
            }
            0xbd => {
                println!("RES 7 L");
                (2, 4)
            }
            0xbe => {
                println!("RES 7 (HL)");
                (2, 4)
            }
            0xbf => {
                println!("RES 7 A");
                (2, 4)
            }
            0xc0 => {
                println!("SET 0 B");
                (2, 4)
            }
            0xc1 => {
                println!("SET 0 C");
                (2, 4)
            }
            0xc2 => {
                println!("SET 0 D");
                (2, 4)
            }
            0xc3 => {
                println!("SET 0 E");
                (2, 4)
            }
            0xc4 => {
                println!("SET 0 H");
                (2, 4)
            }
            0xc5 => {
                println!("SET 0 L");
                (2, 4)
            }
            0xc6 => {
                println!("SET 0 (HL)");
                (2, 4)
            }
            0xc7 => {
                println!("SET 0 A");
                (2, 4)
            }
            0xc8 => {
                println!("SET 1 B");
                (2, 4)
            }
            0xc9 => {
                println!("SET 1 C");
                (2, 4)
            }
            0xca => {
                println!("SET 1 D");
                (2, 4)
            }
            0xcb => {
                println!("SET 1 E");
                (2, 4)
            }
            0xcc => {
                println!("SET 1 H");
                (2, 4)
            }
            0xcd => {
                println!("SET 1 L");
                (2, 4)
            }
            0xce => {
                println!("SET 1 (HL)");
                (2, 4)
            }
            0xcf => {
                println!("SET 1 A");
                (2, 4)
            }
            0xd0 => {
                println!("SET 2 B");
                (2, 4)
            }
            0xd1 => {
                println!("SET 2 C");
                (2, 4)
            }
            0xd2 => {
                println!("SET 2 D");
                (2, 4)
            }
            0xd3 => {
                println!("SET 2 E");
                (2, 4)
            }
            0xd4 => {
                println!("SET 2 H");
                (2, 4)
            }
            0xd5 => {
                println!("SET 2 L");
                (2, 4)
            }
            0xd6 => {
                println!("SET 2 (HL)");
                (2, 4)
            }
            0xd7 => {
                println!("SET 2 A");
                (2, 4)
            }
            0xd8 => {
                println!("SET 3 B");
                (2, 4)
            }
            0xd9 => {
                println!("SET 3 C");
                (2, 4)
            }
            0xda => {
                println!("SET 3 D");
                (2, 4)
            }
            0xdb => {
                println!("SET 3 E");
                (2, 4)
            }
            0xdc => {
                println!("SET 3 H");
                (2, 4)
            }
            0xdd => {
                println!("SET 3 L");
                (2, 4)
            }
            0xde => {
                println!("SET 3 (HL)");
                (2, 4)
            }
            0xdf => {
                println!("SET 3 A");
                (2, 4)
            }
            0xe0 => {
                println!("SET 4 B");
                (2, 4)
            }
            0xe1 => {
                println!("SET 4 C");
                (2, 4)
            }
            0xe2 => {
                println!("SET 4 D");
                (2, 4)
            }
            0xe3 => {
                println!("SET 4 E");
                (2, 4)
            }
            0xe4 => {
                println!("SET 4 H");
                (2, 4)
            }
            0xe5 => {
                println!("SET 4 L");
                (2, 4)
            }
            0xe6 => {
                println!("SET 4 (HL)");
                (2, 4)
            }
            0xe7 => {
                println!("SET 4 A");
                (2, 4)
            }
            0xe8 => {
                println!("SET 5 B");
                (2, 4)
            }
            0xe9 => {
                println!("SET 5 C");
                (2, 4)
            }
            0xea => {
                println!("SET 5 D");
                (2, 4)
            }
            0xeb => {
                println!("SET 5 E");
                (2, 4)
            }
            0xec => {
                println!("SET 5 H");
                (2, 4)
            }
            0xed => {
                println!("SET 5 L");
                (2, 4)
            }
            0xee => {
                println!("SET 5 (HL)");
                (2, 4)
            }
            0xef => {
                println!("SET 5 A");
                (2, 4)
            }
            0xf0 => {
                println!("SET 6 B");
                (2, 4)
            }
            0xf1 => {
                println!("SET 6 C");
                (2, 4)
            }
            0xf2 => {
                println!("SET 6 D");
                (2, 4)
            }
            0xf3 => {
                println!("SET 6 E");
                (2, 4)
            }
            0xf4 => {
                println!("SET 6 H");
                (2, 4)
            }
            0xf5 => {
                println!("SET 6 L");
                (2, 4)
            }
            0xf6 => {
                println!("SET 6 (HL)");
                (2, 4)
            }
            0xf7 => {
                println!("SET 6 A");
                (2, 4)
            }
            0xf8 => {
                println!("SET 7 B");
                (2, 4)
            }
            0xf9 => {
                println!("SET 7 C");
                (2, 4)
            }
            0xfa => {
                println!("SET 7 D");
                (2, 4)
            }
            0xfb => {
                println!("SET 7 E");
                (2, 4)
            }
            0xfc => {
                println!("SET 7 H");
                (2, 4)
            }
            0xfd => {
                println!("SET 7 L");
                (2, 4)
            }
            0xfe => {
                println!("SET 7 (HL)");
                (2, 4)
            }
            0xff => {
                println!("SET 7 A");
                (2, 4)
            }
            _ => {
              println!("INVALID OPCODE: {:x}", cb_opcode);
              (2, 0)
            }
        }
    }
    0xcc => {
        println!("CALL Z a16");
        (3, 24)
    }
    0xcd => {
        println!("CALL a16 ");
        (3, 24)
    }
    0xce => {
        println!("ADC A d8");
        (2, 8)
    }
    0xcf => {
        println!("RST 08H ");
        (1, 16)
    }
    0xd0 => {
        println!("RET NC ");
        (1, 20)
    }
    0xd1 => {
        println!("POP DE ");
        (1, 12)
    }
    0xd2 => {
        println!("JP NC a16");
        (3, 16)
    }
    0xd4 => {
        println!("CALL NC a16");
        (3, 24)
    }
    0xd5 => {
        println!("PUSH DE ");
        (1, 16)
    }
    0xd6 => {
        println!("SUB d8 ");
        (2, 8)
    }
    0xd7 => {
        println!("RST 10H ");
        (1, 16)
    }
    0xd8 => {
        println!("RET C ");
        (1, 20)
    }
    0xd9 => {
        println!("RETI  ");
        (1, 16)
    }
    0xda => {
        println!("JP C a16");
        (3, 16)
    }
    0xdc => {
        println!("CALL C a16");
        (3, 24)
    }
    0xde => {
        println!("SBC A d8");
        (2, 8)
    }
    0xdf => {
        println!("RST 18H ");
        (1, 16)
    }
    0xe0 => {
        println!("LDH (a8) A");
        (2, 12)
    }
    0xe1 => {
        println!("POP HL ");
        (1, 12)
    }
    0xe2 => {
        println!("LD (C) A");
        (1, 8)
    }
    0xe5 => {
        println!("PUSH HL ");
        (1, 16)
    }
    0xe6 => {
        println!("AND d8 ");
        (2, 8)
    }
    0xe7 => {
        println!("RST 20H ");
        (1, 16)
    }
    0xe8 => {
        println!("ADD SP r8");
        (2, 16)
    }
    0xe9 => {
        println!("JP (HL) ");
        (1, 4)
    }
    0xea => {
        println!("LD (a16) A");
        (3, 16)
    }
    0xee => {
        println!("XOR d8 ");
        (2, 8)
    }
    0xef => {
        println!("RST 28H ");
        (1, 16)
    }
    0xf0 => {
        println!("LDH A (a8)");
        (2, 12)
    }
    0xf1 => {
        println!("POP AF ");
        (1, 12)
    }
    0xf2 => {
        println!("LD A (C)");
        (1, 8)
    }
    0xf3 => {
        println!("DI  ");
        (1, 4)
    }
    0xf5 => {
        println!("PUSH AF ");
        (1, 16)
    }
    0xf6 => {
        println!("OR d8 ");
        (2, 8)
    }
    0xf7 => {
        println!("RST 30H ");
        (1, 16)
    }
    0xf8 => {
        println!("LD HL SP+r8");
        (2, 12)
    }
    0xf9 => {
        println!("LD SP HL");
        (1, 8)
    }
    0xfa => {
        println!("LD A (a16)");
        (3, 16)
    }
    0xfb => {
        println!("EI  ");
        (1, 4)
    }
    0xfe => {
        println!("CP d8 ");
        (2, 8)
    }
    0xff => {
        println!("RST 38H ");
        (1, 16)
    }
    _ => {
      println!("INVALID OPCODE: {:x}", opcode);
      (1, 0)
    }
  }
}
