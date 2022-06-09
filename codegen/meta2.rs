use crate::gameboy::{Emulator, Mmu};

pub fn exec_opcode(emu: &Emulator, mmu: &mut Mmu) {
  let opcode = emu.current_opcode();

  match opcode { 
    0x00 => { 
        println!("NOP  ");
        emu.inc_pc_by(1); 
    } 
    0x01 => { 
        println!("LD BC d16");
        emu.inc_pc_by(3);
    } 
    0x02 => { 
        println!("LD (BC) A");
        emu.inc_pc_by(1); 
    } 
    0x03 => { 
        println!("INC BC ");
        emu.inc_pc_by(1); 
    } 
    0x04 => { 
        println!("INC B ");
        emu.inc_pc_by(1); 
    } 
    0x05 => { 
        println!("DEC B ");
        emu.inc_pc_by(1); 
    } 
    0x06 => { 
        println!("LD B d8");
        emu.inc_pc_by(2); 
    } 
    0x07 => { 
        println!("RLCA  ");
        emu.inc_pc_by(1); 
    } 
    0x08 => { 
        println!("LD (a16) SP");
        emu.inc_pc_by(3); 
    } 
    0x09 => { 
        println!("ADD HL BC");
        emu.inc_pc_by(1); 
    } 
    0x0a => { 
        println!("LD A (BC)");
        emu.inc_pc_by(1); 
    } 
    0x0b => { 
        println!("DEC BC ");
        emu.inc_pc_by(1); 
    } 
    0x0c => { 
        println!("INC C ");
        emu.inc_pc_by(1); 
    } 
    0x0d => { 
        println!("DEC C ");
        emu.inc_pc_by(1); 
    } 
    0x0e => { 
        println!("LD C d8");
        emu.inc_pc_by(2); 
    } 
    0x0f => { 
        println!("RRCA  ");
        emu.inc_pc_by(1); 
    } 
    0x10 => { 
        println!("STOP 0 ");
        emu.inc_pc_by(1); 
    } 
    0x11 => { 
        println!("LD DE d16");
        emu.inc_pc_by(3); 
    } 
    0x12 => { 
        println!("LD (DE) A");
        emu.inc_pc_by(1); 
    } 
    0x13 => { 
        println!("INC DE ");
        emu.inc_pc_by(1); 
    } 
    0x14 => { 
        println!("INC D ");
        emu.inc_pc_by(1); 
    } 
    0x15 => { 
        println!("DEC D ");
        emu.inc_pc_by(1); 
    } 
    0x16 => { 
        println!("LD D d8");
        emu.inc_pc_by(2); 
    } 
    0x17 => { 
        println!("RLA  ");
        emu.inc_pc_by(1); 
    } 
    0x18 => { 
        println!("JR r8 ");
        emu.inc_pc_by(2); 
    } 
    0x19 => { 
        println!("ADD HL DE");
        emu.inc_pc_by(1); 
    } 
    0x1a => { 
        println!("LD A (DE)");
        emu.inc_pc_by(1); 
    } 
    0x1b => { 
        println!("DEC DE ");
        emu.inc_pc_by(1); 
    } 
    0x1c => { 
        println!("INC E ");
        emu.inc_pc_by(1); 
    } 
    0x1d => { 
        println!("DEC E ");
        emu.inc_pc_by(1); 
    } 
    0x1e => { 
        println!("LD E d8");
        emu.inc_pc_by(2); 
    } 
    0x1f => { 
        println!("RRA  ");
        emu.inc_pc_by(1); 
    } 
    0x20 => { 
        println!("JR NZ r8");
        emu.inc_pc_by(2); 
    } 
    0x21 => { 
        println!("LD HL d16");
        emu.inc_pc_by(3); 
    } 
    0x22 => { 
        println!("LD (HL+) A");
        emu.inc_pc_by(1); 
    } 
    0x23 => { 
        println!("INC HL ");
        emu.inc_pc_by(1); 
    } 
    0x24 => { 
        println!("INC H ");
        emu.inc_pc_by(1); 
    } 
    0x25 => { 
        println!("DEC H ");
        emu.inc_pc_by(1); 
    } 
    0x26 => { 
        println!("LD H d8");
        emu.inc_pc_by(2); 
    } 
    0x27 => { 
        println!("DAA  ");
        emu.inc_pc_by(1); 
    } 
    0x28 => { 
        println!("JR Z r8");
        emu.inc_pc_by(2); 
    } 
    0x29 => { 
        println!("ADD HL HL");
        emu.inc_pc_by(1); 
    } 
    0x2a => { 
        println!("LD A (HL+)");
        emu.inc_pc_by(1); 
    } 
    0x2b => { 
        println!("DEC HL ");
        emu.inc_pc_by(1); 
    } 
    0x2c => { 
        println!("INC L ");
        emu.inc_pc_by(1); 
    } 
    0x2d => { 
        println!("DEC L ");
        emu.inc_pc_by(1); 
    } 
    0x2e => { 
        println!("LD L d8");
        emu.inc_pc_by(2); 
    } 
    0x2f => { 
        println!("CPL  ");
        emu.inc_pc_by(1); 
    } 
    0x30 => { 
        println!("JR NC r8");
        emu.inc_pc_by(2); 
    } 
    0x31 => { 
        println!("LD SP d16");
        emu.inc_pc_by(3); 
    } 
    0x32 => { 
        println!("LD (HL-) A");
        emu.inc_pc_by(1); 
    } 
    0x33 => { 
        println!("INC SP ");
        emu.inc_pc_by(1); 
    } 
    0x34 => { 
        println!("INC (HL) ");
        emu.inc_pc_by(1); 
    } 
    0x35 => { 
        println!("DEC (HL) ");
        emu.inc_pc_by(1); 
    } 
    0x36 => { 
        println!("LD (HL) d8");
        emu.inc_pc_by(2); 
    } 
    0x37 => { 
        println!("SCF  ");
        emu.inc_pc_by(1); 
    } 
    0x38 => { 
        println!("JR C r8");
        emu.inc_pc_by(2); 
    } 
    0x39 => { 
        println!("ADD HL SP");
        emu.inc_pc_by(1); 
    } 
    0x3a => { 
        println!("LD A (HL-)");
        emu.inc_pc_by(1); 
    } 
    0x3b => { 
        println!("DEC SP ");
        emu.inc_pc_by(1); 
    } 
    0x3c => { 
        println!("INC A ");
        emu.inc_pc_by(1); 
    } 
    0x3d => { 
        println!("DEC A ");
        emu.inc_pc_by(1); 
    } 
    0x3e => { 
        println!("LD A d8");
        emu.inc_pc_by(2); 
    } 
    0x3f => { 
        println!("CCF  ");
        emu.inc_pc_by(1); 
    } 
    0x40 => { 
        println!("LD B B");
        emu.inc_pc_by(1); 
    } 
    0x41 => { 
        println!("LD B C");
        emu.inc_pc_by(1); 
    } 
    0x42 => { 
        println!("LD B D");
        emu.inc_pc_by(1); 
    } 
    0x43 => { 
        println!("LD B E");
        emu.inc_pc_by(1); 
    } 
    0x44 => { 
        println!("LD B H");
        emu.inc_pc_by(1); 
    } 
    0x45 => { 
        println!("LD B L");
        emu.inc_pc_by(1); 
    } 
    0x46 => { 
        println!("LD B (HL)");
        emu.inc_pc_by(1); 
    } 
    0x47 => { 
        println!("LD B A");
        emu.inc_pc_by(1); 
    } 
    0x48 => { 
        println!("LD C B");
        emu.inc_pc_by(1); 
    } 
    0x49 => { 
        println!("LD C C");
        emu.inc_pc_by(1); 
    } 
    0x4a => { 
        println!("LD C D");
        emu.inc_pc_by(1); 
    } 
    0x4b => { 
        println!("LD C E");
        emu.inc_pc_by(1); 
    } 
    0x4c => { 
        println!("LD C H");
        emu.inc_pc_by(1); 
    } 
    0x4d => { 
        println!("LD C L");
        emu.inc_pc_by(1); 
    } 
    0x4e => { 
        println!("LD C (HL)");
        emu.inc_pc_by(1); 
    } 
    0x4f => { 
        println!("LD C A");
        emu.inc_pc_by(1); 
    } 
    0x50 => { 
        println!("LD D B");
        emu.inc_pc_by(1); 
    } 
    0x51 => { 
        println!("LD D C");
        emu.inc_pc_by(1); 
    } 
    0x52 => { 
        println!("LD D D");
        emu.inc_pc_by(1); 
    } 
    0x53 => { 
        println!("LD D E");
        emu.inc_pc_by(1); 
    } 
    0x54 => { 
        println!("LD D H");
        emu.inc_pc_by(1); 
    } 
    0x55 => { 
        println!("LD D L");
        emu.inc_pc_by(1); 
    } 
    0x56 => { 
        println!("LD D (HL)");
        emu.inc_pc_by(1); 
    } 
    0x57 => { 
        println!("LD D A");
        emu.inc_pc_by(1); 
    } 
    0x58 => { 
        println!("LD E B");
        emu.inc_pc_by(1); 
    } 
    0x59 => { 
        println!("LD E C");
        emu.inc_pc_by(1); 
    } 
    0x5a => { 
        println!("LD E D");
        emu.inc_pc_by(1); 
    } 
    0x5b => { 
        println!("LD E E");
        emu.inc_pc_by(1); 
    } 
    0x5c => { 
        println!("LD E H");
        emu.inc_pc_by(1); 
    } 
    0x5d => { 
        println!("LD E L");
        emu.inc_pc_by(1); 
    } 
    0x5e => { 
        println!("LD E (HL)");
        emu.inc_pc_by(1); 
    } 
    0x5f => { 
        println!("LD E A");
        emu.inc_pc_by(1); 
    } 
    0x60 => { 
        println!("LD H B");
        emu.inc_pc_by(1); 
    } 
    0x61 => { 
        println!("LD H C");
        emu.inc_pc_by(1); 
    } 
    0x62 => { 
        println!("LD H D");
        emu.inc_pc_by(1); 
    } 
    0x63 => { 
        println!("LD H E");
        emu.inc_pc_by(1); 
    } 
    0x64 => { 
        println!("LD H H");
        emu.inc_pc_by(1); 
    } 
    0x65 => { 
        println!("LD H L");
        emu.inc_pc_by(1); 
    } 
    0x66 => { 
        println!("LD H (HL)");
        emu.inc_pc_by(1); 
    } 
    0x67 => { 
        println!("LD H A");
        emu.inc_pc_by(1); 
    } 
    0x68 => { 
        println!("LD L B");
        emu.inc_pc_by(1); 
    } 
    0x69 => { 
        println!("LD L C");
        emu.inc_pc_by(1); 
    } 
    0x6a => { 
        println!("LD L D");
        emu.inc_pc_by(1); 
    } 
    0x6b => { 
        println!("LD L E");
        emu.inc_pc_by(1); 
    } 
    0x6c => { 
        println!("LD L H");
        emu.inc_pc_by(1); 
    } 
    0x6d => { 
        println!("LD L L");
        emu.inc_pc_by(1); 
    } 
    0x6e => { 
        println!("LD L (HL)");
        emu.inc_pc_by(1); 
    } 
    0x6f => { 
        println!("LD L A");
        emu.inc_pc_by(1); 
    } 
    0x70 => { 
        println!("LD (HL) B");
        emu.inc_pc_by(1); 
    } 
    0x71 => { 
        println!("LD (HL) C");
        emu.inc_pc_by(1); 
    } 
    0x72 => { 
        println!("LD (HL) D");
        emu.inc_pc_by(1); 
    } 
    0x73 => { 
        println!("LD (HL) E");
        emu.inc_pc_by(1); 
    } 
    0x74 => { 
        println!("LD (HL) H");
        emu.inc_pc_by(1); 
    } 
    0x75 => { 
        println!("LD (HL) L");
        emu.inc_pc_by(1); 
    } 
    0x76 => { 
        println!("HALT  ");
        emu.inc_pc_by(1); 
    } 
    0x77 => { 
        println!("LD (HL) A");
        emu.inc_pc_by(1); 
    } 
    0x78 => { 
        println!("LD A B");
        emu.inc_pc_by(1); 
    } 
    0x79 => { 
        println!("LD A C");
        emu.inc_pc_by(1); 
    } 
    0x7a => { 
        println!("LD A D");
        emu.inc_pc_by(1); 
    } 
    0x7b => { 
        println!("LD A E");
        emu.inc_pc_by(1); 
    } 
    0x7c => { 
        println!("LD A H");
        emu.inc_pc_by(1); 
    } 
    0x7d => { 
        println!("LD A L");
        emu.inc_pc_by(1); 
    } 
    0x7e => { 
        println!("LD A (HL)");
        emu.inc_pc_by(1); 
    } 
    0x7f => { 
        println!("LD A A");
        emu.inc_pc_by(1); 
    } 
    0x80 => { 
        println!("ADD A B");
        emu.inc_pc_by(1); 
    } 
    0x81 => { 
        println!("ADD A C");
        emu.inc_pc_by(1); 
    } 
    0x82 => { 
        println!("ADD A D");
        emu.inc_pc_by(1); 
    } 
    0x83 => { 
        println!("ADD A E");
        emu.inc_pc_by(1); 
    } 
    0x84 => { 
        println!("ADD A H");
        emu.inc_pc_by(1); 
    } 
    0x85 => { 
        println!("ADD A L");
        emu.inc_pc_by(1); 
    } 
    0x86 => { 
        println!("ADD A (HL)");
        emu.inc_pc_by(1); 
    } 
    0x87 => { 
        println!("ADD A A");
        emu.inc_pc_by(1); 
    } 
    0x88 => { 
        println!("ADC A B");
        emu.inc_pc_by(1); 
    } 
    0x89 => { 
        println!("ADC A C");
        emu.inc_pc_by(1); 
    } 
    0x8a => { 
        println!("ADC A D");
        emu.inc_pc_by(1); 
    } 
    0x8b => { 
        println!("ADC A E");
        emu.inc_pc_by(1); 
    } 
    0x8c => { 
        println!("ADC A H");
        emu.inc_pc_by(1); 
    } 
    0x8d => { 
        println!("ADC A L");
        emu.inc_pc_by(1); 
    } 
    0x8e => { 
        println!("ADC A (HL)");
        emu.inc_pc_by(1); 
    } 
    0x8f => { 
        println!("ADC A A");
        emu.inc_pc_by(1); 
    } 
    0x90 => { 
        println!("SUB B ");
        emu.inc_pc_by(1); 
    } 
    0x91 => { 
        println!("SUB C ");
        emu.inc_pc_by(1); 
    } 
    0x92 => { 
        println!("SUB D ");
        emu.inc_pc_by(1); 
    } 
    0x93 => { 
        println!("SUB E ");
        emu.inc_pc_by(1); 
    } 
    0x94 => { 
        println!("SUB H ");
        emu.inc_pc_by(1); 
    } 
    0x95 => { 
        println!("SUB L ");
        emu.inc_pc_by(1); 
    } 
    0x96 => { 
        println!("SUB (HL) ");
        emu.inc_pc_by(1); 
    } 
    0x97 => { 
        println!("SUB A ");
        emu.inc_pc_by(1); 
    } 
    0x98 => { 
        println!("SBC A B");
        emu.inc_pc_by(1); 
    } 
    0x99 => { 
        println!("SBC A C");
        emu.inc_pc_by(1); 
    } 
    0x9a => { 
        println!("SBC A D");
        emu.inc_pc_by(1); 
    } 
    0x9b => { 
        println!("SBC A E");
        emu.inc_pc_by(1); 
    } 
    0x9c => { 
        println!("SBC A H");
        emu.inc_pc_by(1); 
    } 
    0x9d => { 
        println!("SBC A L");
        emu.inc_pc_by(1); 
    } 
    0x9e => { 
        println!("SBC A (HL)");
        emu.inc_pc_by(1); 
    } 
    0x9f => { 
        println!("SBC A A");
        emu.inc_pc_by(1); 
    } 
    0xa0 => { 
        println!("AND B ");
        emu.inc_pc_by(1); 
    } 
    0xa1 => { 
        println!("AND C ");
        emu.inc_pc_by(1); 
    } 
    0xa2 => { 
        println!("AND D ");
        emu.inc_pc_by(1); 
    } 
    0xa3 => { 
        println!("AND E ");
        emu.inc_pc_by(1); 
    } 
    0xa4 => { 
        println!("AND H ");
        emu.inc_pc_by(1); 
    } 
    0xa5 => { 
        println!("AND L ");
        emu.inc_pc_by(1); 
    } 
    0xa6 => { 
        println!("AND (HL) ");
        emu.inc_pc_by(1); 
    } 
    0xa7 => { 
        println!("AND A ");
        emu.inc_pc_by(1); 
    } 
    0xa8 => { 
        println!("XOR B ");
        emu.inc_pc_by(1); 
    } 
    0xa9 => { 
        println!("XOR C ");
        emu.inc_pc_by(1); 
    } 
    0xaa => { 
        println!("XOR D ");
        emu.inc_pc_by(1); 
    } 
    0xab => { 
        println!("XOR E ");
        emu.inc_pc_by(1); 
    } 
    0xac => { 
        println!("XOR H ");
        emu.inc_pc_by(1); 
    } 
    0xad => { 
        println!("XOR L ");
        emu.inc_pc_by(1); 
    } 
    0xae => { 
        println!("XOR (HL) ");
        emu.inc_pc_by(1); 
    } 
    0xaf => { 
        println!("XOR A ");
        emu.inc_pc_by(1); 
    } 
    0xb0 => { 
        println!("OR B ");
        emu.inc_pc_by(1); 
    } 
    0xb1 => { 
        println!("OR C ");
        emu.inc_pc_by(1); 
    } 
    0xb2 => { 
        println!("OR D ");
        emu.inc_pc_by(1); 
    } 
    0xb3 => { 
        println!("OR E ");
        emu.inc_pc_by(1); 
    } 
    0xb4 => { 
        println!("OR H ");
        emu.inc_pc_by(1); 
    } 
    0xb5 => { 
        println!("OR L ");
        emu.inc_pc_by(1); 
    } 
    0xb6 => { 
        println!("OR (HL) ");
        emu.inc_pc_by(1); 
    } 
    0xb7 => { 
        println!("OR A ");
        emu.inc_pc_by(1); 
    } 
    0xb8 => { 
        println!("CP B ");
        emu.inc_pc_by(1); 
    } 
    0xb9 => { 
        println!("CP C ");
        emu.inc_pc_by(1); 
    } 
    0xba => { 
        println!("CP D ");
        emu.inc_pc_by(1); 
    } 
    0xbb => { 
        println!("CP E ");
        emu.inc_pc_by(1); 
    } 
    0xbc => { 
        println!("CP H ");
        emu.inc_pc_by(1); 
    } 
    0xbd => { 
        println!("CP L ");
        emu.inc_pc_by(1); 
    } 
    0xbe => { 
        println!("CP (HL) ");
        emu.inc_pc_by(1); 
    } 
    0xbf => { 
        println!("CP A ");
        emu.inc_pc_by(1); 
    } 
    0xc0 => { 
        println!("RET NZ ");
        emu.inc_pc_by(1); 
    } 
    0xc1 => { 
        println!("POP BC ");
        emu.inc_pc_by(1); 
    } 
    0xc2 => { 
        println!("JP NZ a16");
        emu.inc_pc_by(3); 
    } 
    0xc3 => { 
        println!("JP a16 ");
        emu.inc_pc_by(3); 
    } 
    0xc4 => { 
        println!("CALL NZ a16");
        emu.inc_pc_by(3); 
    } 
    0xc5 => { 
        println!("PUSH BC ");
        emu.inc_pc_by(1); 
    } 
    0xc6 => { 
        println!("ADD A d8");
        emu.inc_pc_by(2); 
    } 
    0xc7 => { 
        println!("RST 00H ");
        emu.inc_pc_by(1); 
    } 
    0xc8 => { 
        println!("RET Z ");
        emu.inc_pc_by(1); 
    } 
    0xc9 => { 
        println!("RET  ");
        emu.inc_pc_by(1); 
    } 
    0xca => { 
        println!("JP Z a16");
        emu.inc_pc_by(3); 
    } 
    0xcb => { 
        emu.inc_pc_by(1);
        let cb_opcode = emu.current_opcode();
        match cb_opcode { 
            0x00 => {
                println!("RLC B ");
                emu.inc_pc_by(1);
            } 
            0x01 => {
                println!("RLC C ");
                emu.inc_pc_by(1);
            } 
            0x02 => {
                println!("RLC D ");
                emu.inc_pc_by(1);
            } 
            0x03 => {
                println!("RLC E ");
                emu.inc_pc_by(1);
            } 
            0x04 => {
                println!("RLC H ");
                emu.inc_pc_by(1);
            } 
            0x05 => {
                println!("RLC L ");
                emu.inc_pc_by(1);
            } 
            0x06 => {
                println!("RLC (HL) ");
                emu.inc_pc_by(1);
            } 
            0x07 => {
                println!("RLC A ");
                emu.inc_pc_by(1);
            } 
            0x08 => {
                println!("RRC B ");
                emu.inc_pc_by(1);
            } 
            0x09 => {
                println!("RRC C ");
                emu.inc_pc_by(1);
            } 
            0x0a => {
                println!("RRC D ");
                emu.inc_pc_by(1);
            } 
            0x0b => {
                println!("RRC E ");
                emu.inc_pc_by(1);
            } 
            0x0c => {
                println!("RRC H ");
                emu.inc_pc_by(1);
            } 
            0x0d => {
                println!("RRC L ");
                emu.inc_pc_by(1);
            } 
            0x0e => {
                println!("RRC (HL) ");
                emu.inc_pc_by(1);
            } 
            0x0f => {
                println!("RRC A ");
                emu.inc_pc_by(1);
            } 
            0x10 => {
                println!("RL B ");
                emu.inc_pc_by(1);
            } 
            0x11 => {
                println!("RL C ");
                emu.inc_pc_by(1);
            } 
            0x12 => {
                println!("RL D ");
                emu.inc_pc_by(1);
            } 
            0x13 => {
                println!("RL E ");
                emu.inc_pc_by(1);
            } 
            0x14 => {
                println!("RL H ");
                emu.inc_pc_by(1);
            } 
            0x15 => {
                println!("RL L ");
                emu.inc_pc_by(1);
            } 
            0x16 => {
                println!("RL (HL) ");
                emu.inc_pc_by(1);
            } 
            0x17 => {
                println!("RL A ");
                emu.inc_pc_by(1);
            } 
            0x18 => {
                println!("RR B ");
                emu.inc_pc_by(1);
            } 
            0x19 => {
                println!("RR C ");
                emu.inc_pc_by(1);
            } 
            0x1a => {
                println!("RR D ");
                emu.inc_pc_by(1);
            } 
            0x1b => {
                println!("RR E ");
                emu.inc_pc_by(1);
            } 
            0x1c => {
                println!("RR H ");
                emu.inc_pc_by(1);
            } 
            0x1d => {
                println!("RR L ");
                emu.inc_pc_by(1);
            } 
            0x1e => {
                println!("RR (HL) ");
                emu.inc_pc_by(1);
            } 
            0x1f => {
                println!("RR A ");
                emu.inc_pc_by(1);
            } 
            0x20 => {
                println!("SLA B ");
                emu.inc_pc_by(1);
            } 
            0x21 => {
                println!("SLA C ");
                emu.inc_pc_by(1);
            } 
            0x22 => {
                println!("SLA D ");
                emu.inc_pc_by(1);
            } 
            0x23 => {
                println!("SLA E ");
                emu.inc_pc_by(1);
            } 
            0x24 => {
                println!("SLA H ");
                emu.inc_pc_by(1);
            } 
            0x25 => {
                println!("SLA L ");
                emu.inc_pc_by(1);
            } 
            0x26 => {
                println!("SLA (HL) ");
                emu.inc_pc_by(1);
            } 
            0x27 => {
                println!("SLA A ");
                emu.inc_pc_by(1);
            } 
            0x28 => {
                println!("SRA B ");
                emu.inc_pc_by(1);
            } 
            0x29 => {
                println!("SRA C ");
                emu.inc_pc_by(1);
            } 
            0x2a => {
                println!("SRA D ");
                emu.inc_pc_by(1);
            } 
            0x2b => {
                println!("SRA E ");
                emu.inc_pc_by(1);
            } 
            0x2c => {
                println!("SRA H ");
                emu.inc_pc_by(1);
            } 
            0x2d => {
                println!("SRA L ");
                emu.inc_pc_by(1);
            } 
            0x2e => {
                println!("SRA (HL) ");
                emu.inc_pc_by(1);
            } 
            0x2f => {
                println!("SRA A ");
                emu.inc_pc_by(1);
            } 
            0x30 => {
                println!("SWAP B ");
                emu.inc_pc_by(1);
            } 
            0x31 => {
                println!("SWAP C ");
                emu.inc_pc_by(1);
            } 
            0x32 => {
                println!("SWAP D ");
                emu.inc_pc_by(1);
            } 
            0x33 => {
                println!("SWAP E ");
                emu.inc_pc_by(1);
            } 
            0x34 => {
                println!("SWAP H ");
                emu.inc_pc_by(1);
            } 
            0x35 => {
                println!("SWAP L ");
                emu.inc_pc_by(1);
            } 
            0x36 => {
                println!("SWAP (HL) ");
                emu.inc_pc_by(1);
            } 
            0x37 => {
                println!("SWAP A ");
                emu.inc_pc_by(1);
            } 
            0x38 => {
                println!("SRL B ");
                emu.inc_pc_by(1);
            } 
            0x39 => {
                println!("SRL C ");
                emu.inc_pc_by(1);
            } 
            0x3a => {
                println!("SRL D ");
                emu.inc_pc_by(1);
            } 
            0x3b => {
                println!("SRL E ");
                emu.inc_pc_by(1);
            } 
            0x3c => {
                println!("SRL H ");
                emu.inc_pc_by(1);
            } 
            0x3d => {
                println!("SRL L ");
                emu.inc_pc_by(1);
            } 
            0x3e => {
                println!("SRL (HL) ");
                emu.inc_pc_by(1);
            } 
            0x3f => {
                println!("SRL A ");
                emu.inc_pc_by(1);
            } 
            0x40 => {
                println!("BIT 0 B");
                emu.inc_pc_by(1);
            } 
            0x41 => {
                println!("BIT 0 C");
                emu.inc_pc_by(1);
            } 
            0x42 => {
                println!("BIT 0 D");
                emu.inc_pc_by(1);
            } 
            0x43 => {
                println!("BIT 0 E");
                emu.inc_pc_by(1);
            } 
            0x44 => {
                println!("BIT 0 H");
                emu.inc_pc_by(1);
            } 
            0x45 => {
                println!("BIT 0 L");
                emu.inc_pc_by(1);
            } 
            0x46 => {
                println!("BIT 0 (HL)");
                emu.inc_pc_by(1);
            } 
            0x47 => {
                println!("BIT 0 A");
                emu.inc_pc_by(1);
            } 
            0x48 => {
                println!("BIT 1 B");
                emu.inc_pc_by(1);
            } 
            0x49 => {
                println!("BIT 1 C");
                emu.inc_pc_by(1);
            } 
            0x4a => {
                println!("BIT 1 D");
                emu.inc_pc_by(1);
            } 
            0x4b => {
                println!("BIT 1 E");
                emu.inc_pc_by(1);
            } 
            0x4c => {
                println!("BIT 1 H");
                emu.inc_pc_by(1);
            } 
            0x4d => {
                println!("BIT 1 L");
                emu.inc_pc_by(1);
            } 
            0x4e => {
                println!("BIT 1 (HL)");
                emu.inc_pc_by(1);
            } 
            0x4f => {
                println!("BIT 1 A");
                emu.inc_pc_by(1);
            } 
            0x50 => {
                println!("BIT 2 B");
                emu.inc_pc_by(1);
            } 
            0x51 => {
                println!("BIT 2 C");
                emu.inc_pc_by(1);
            } 
            0x52 => {
                println!("BIT 2 D");
                emu.inc_pc_by(1);
            } 
            0x53 => {
                println!("BIT 2 E");
                emu.inc_pc_by(1);
            } 
            0x54 => {
                println!("BIT 2 H");
                emu.inc_pc_by(1);
            } 
            0x55 => {
                println!("BIT 2 L");
                emu.inc_pc_by(1);
            } 
            0x56 => {
                println!("BIT 2 (HL)");
                emu.inc_pc_by(1);
            } 
            0x57 => {
                println!("BIT 2 A");
                emu.inc_pc_by(1);
            } 
            0x58 => {
                println!("BIT 3 B");
                emu.inc_pc_by(1);
            } 
            0x59 => {
                println!("BIT 3 C");
                emu.inc_pc_by(1);
            } 
            0x5a => {
                println!("BIT 3 D");
                emu.inc_pc_by(1);
            } 
            0x5b => {
                println!("BIT 3 E");
                emu.inc_pc_by(1);
            } 
            0x5c => {
                println!("BIT 3 H");
                emu.inc_pc_by(1);
            } 
            0x5d => {
                println!("BIT 3 L");
                emu.inc_pc_by(1);
            } 
            0x5e => {
                println!("BIT 3 (HL)");
                emu.inc_pc_by(1);
            } 
            0x5f => {
                println!("BIT 3 A");
                emu.inc_pc_by(1);
            } 
            0x60 => {
                println!("BIT 4 B");
                emu.inc_pc_by(1);
            } 
            0x61 => {
                println!("BIT 4 C");
                emu.inc_pc_by(1);
            } 
            0x62 => {
                println!("BIT 4 D");
                emu.inc_pc_by(1);
            } 
            0x63 => {
                println!("BIT 4 E");
                emu.inc_pc_by(1);
            } 
            0x64 => {
                println!("BIT 4 H");
                emu.inc_pc_by(1);
            } 
            0x65 => {
                println!("BIT 4 L");
                emu.inc_pc_by(1);
            } 
            0x66 => {
                println!("BIT 4 (HL)");
                emu.inc_pc_by(1);
            } 
            0x67 => {
                println!("BIT 4 A");
                emu.inc_pc_by(1);
            } 
            0x68 => {
                println!("BIT 5 B");
                emu.inc_pc_by(1);
            } 
            0x69 => {
                println!("BIT 5 C");
                emu.inc_pc_by(1);
            } 
            0x6a => {
                println!("BIT 5 D");
                emu.inc_pc_by(1);
            } 
            0x6b => {
                println!("BIT 5 E");
                emu.inc_pc_by(1);
            } 
            0x6c => {
                println!("BIT 5 H");
                emu.inc_pc_by(1);
            } 
            0x6d => {
                println!("BIT 5 L");
                emu.inc_pc_by(1);
            } 
            0x6e => {
                println!("BIT 5 (HL)");
                emu.inc_pc_by(1);
            } 
            0x6f => {
                println!("BIT 5 A");
                emu.inc_pc_by(1);
            } 
            0x70 => {
                println!("BIT 6 B");
                emu.inc_pc_by(1);
            } 
            0x71 => {
                println!("BIT 6 C");
                emu.inc_pc_by(1);
            } 
            0x72 => {
                println!("BIT 6 D");
                emu.inc_pc_by(1);
            } 
            0x73 => {
                println!("BIT 6 E");
                emu.inc_pc_by(1);
            } 
            0x74 => {
                println!("BIT 6 H");
                emu.inc_pc_by(1);
            } 
            0x75 => {
                println!("BIT 6 L");
                emu.inc_pc_by(1);
            } 
            0x76 => {
                println!("BIT 6 (HL)");
                emu.inc_pc_by(1);
            } 
            0x77 => {
                println!("BIT 6 A");
                emu.inc_pc_by(1);
            } 
            0x78 => {
                println!("BIT 7 B");
                emu.inc_pc_by(1);
            } 
            0x79 => {
                println!("BIT 7 C");
                emu.inc_pc_by(1);
            } 
            0x7a => {
                println!("BIT 7 D");
                emu.inc_pc_by(1);
            } 
            0x7b => {
                println!("BIT 7 E");
                emu.inc_pc_by(1);
            } 
            0x7c => {
                println!("BIT 7 H");
                emu.inc_pc_by(1);
            } 
            0x7d => {
                println!("BIT 7 L");
                emu.inc_pc_by(1);
            } 
            0x7e => {
                println!("BIT 7 (HL)");
                emu.inc_pc_by(1);
            } 
            0x7f => {
                println!("BIT 7 A");
                emu.inc_pc_by(1);
            } 
            0x80 => {
                println!("RES 0 B");
                emu.inc_pc_by(1);
            } 
            0x81 => {
                println!("RES 0 C");
                emu.inc_pc_by(1);
            } 
            0x82 => {
                println!("RES 0 D");
                emu.inc_pc_by(1);
            } 
            0x83 => {
                println!("RES 0 E");
                emu.inc_pc_by(1);
            } 
            0x84 => {
                println!("RES 0 H");
                emu.inc_pc_by(1);
            } 
            0x85 => {
                println!("RES 0 L");
                emu.inc_pc_by(1);
            } 
            0x86 => {
                println!("RES 0 (HL)");
                emu.inc_pc_by(1);
            } 
            0x87 => {
                println!("RES 0 A");
                emu.inc_pc_by(1);
            } 
            0x88 => {
                println!("RES 1 B");
                emu.inc_pc_by(1);
            } 
            0x89 => {
                println!("RES 1 C");
                emu.inc_pc_by(1);
            } 
            0x8a => {
                println!("RES 1 D");
                emu.inc_pc_by(1);
            } 
            0x8b => {
                println!("RES 1 E");
                emu.inc_pc_by(1);
            } 
            0x8c => {
                println!("RES 1 H");
                emu.inc_pc_by(1);
            } 
            0x8d => {
                println!("RES 1 L");
                emu.inc_pc_by(1);
            } 
            0x8e => {
                println!("RES 1 (HL)");
                emu.inc_pc_by(1);
            } 
            0x8f => {
                println!("RES 1 A");
                emu.inc_pc_by(1);
            } 
            0x90 => {
                println!("RES 2 B");
                emu.inc_pc_by(1);
            } 
            0x91 => {
                println!("RES 2 C");
                emu.inc_pc_by(1);
            } 
            0x92 => {
                println!("RES 2 D");
                emu.inc_pc_by(1);
            } 
            0x93 => {
                println!("RES 2 E");
                emu.inc_pc_by(1);
            } 
            0x94 => {
                println!("RES 2 H");
                emu.inc_pc_by(1);
            } 
            0x95 => {
                println!("RES 2 L");
                emu.inc_pc_by(1);
            } 
            0x96 => {
                println!("RES 2 (HL)");
                emu.inc_pc_by(1);
            } 
            0x97 => {
                println!("RES 2 A");
                emu.inc_pc_by(1);
            } 
            0x98 => {
                println!("RES 3 B");
                emu.inc_pc_by(1);
            } 
            0x99 => {
                println!("RES 3 C");
                emu.inc_pc_by(1);
            } 
            0x9a => {
                println!("RES 3 D");
                emu.inc_pc_by(1);
            } 
            0x9b => {
                println!("RES 3 E");
                emu.inc_pc_by(1);
            } 
            0x9c => {
                println!("RES 3 H");
                emu.inc_pc_by(1);
            } 
            0x9d => {
                println!("RES 3 L");
                emu.inc_pc_by(1);
            } 
            0x9e => {
                println!("RES 3 (HL)");
                emu.inc_pc_by(1);
            } 
            0x9f => {
                println!("RES 3 A");
                emu.inc_pc_by(1);
            } 
            0xa0 => {
                println!("RES 4 B");
                emu.inc_pc_by(1);
            } 
            0xa1 => {
                println!("RES 4 C");
                emu.inc_pc_by(1);
            } 
            0xa2 => {
                println!("RES 4 D");
                emu.inc_pc_by(1);
            } 
            0xa3 => {
                println!("RES 4 E");
                emu.inc_pc_by(1);
            } 
            0xa4 => {
                println!("RES 4 H");
                emu.inc_pc_by(1);
            } 
            0xa5 => {
                println!("RES 4 L");
                emu.inc_pc_by(1);
            } 
            0xa6 => {
                println!("RES 4 (HL)");
                emu.inc_pc_by(1);
            } 
            0xa7 => {
                println!("RES 4 A");
                emu.inc_pc_by(1);
            } 
            0xa8 => {
                println!("RES 5 B");
                emu.inc_pc_by(1);
            } 
            0xa9 => {
                println!("RES 5 C");
                emu.inc_pc_by(1);
            } 
            0xaa => {
                println!("RES 5 D");
                emu.inc_pc_by(1);
            } 
            0xab => {
                println!("RES 5 E");
                emu.inc_pc_by(1);
            } 
            0xac => {
                println!("RES 5 H");
                emu.inc_pc_by(1);
            } 
            0xad => {
                println!("RES 5 L");
                emu.inc_pc_by(1);
            } 
            0xae => {
                println!("RES 5 (HL)");
                emu.inc_pc_by(1);
            } 
            0xaf => {
                println!("RES 5 A");
                emu.inc_pc_by(1);
            } 
            0xb0 => {
                println!("RES 6 B");
                emu.inc_pc_by(1);
            } 
            0xb1 => {
                println!("RES 6 C");
                emu.inc_pc_by(1);
            } 
            0xb2 => {
                println!("RES 6 D");
                emu.inc_pc_by(1);
            } 
            0xb3 => {
                println!("RES 6 E");
                emu.inc_pc_by(1);
            } 
            0xb4 => {
                println!("RES 6 H");
                emu.inc_pc_by(1);
            } 
            0xb5 => {
                println!("RES 6 L");
                emu.inc_pc_by(1);
            } 
            0xb6 => {
                println!("RES 6 (HL)");
                emu.inc_pc_by(1);
            } 
            0xb7 => {
                println!("RES 6 A");
                emu.inc_pc_by(1);
            } 
            0xb8 => {
                println!("RES 7 B");
                emu.inc_pc_by(1);
            } 
            0xb9 => {
                println!("RES 7 C");
                emu.inc_pc_by(1);
            } 
            0xba => {
                println!("RES 7 D");
                emu.inc_pc_by(1);
            } 
            0xbb => {
                println!("RES 7 E");
                emu.inc_pc_by(1);
            } 
            0xbc => {
                println!("RES 7 H");
                emu.inc_pc_by(1);
            } 
            0xbd => {
                println!("RES 7 L");
                emu.inc_pc_by(1);
            } 
            0xbe => {
                println!("RES 7 (HL)");
                emu.inc_pc_by(1);
            } 
            0xbf => {
                println!("RES 7 A");
                emu.inc_pc_by(1);
            } 
            0xc0 => {
                println!("SET 0 B");
                emu.inc_pc_by(1);
            } 
            0xc1 => {
                println!("SET 0 C");
                emu.inc_pc_by(1);
            } 
            0xc2 => {
                println!("SET 0 D");
                emu.inc_pc_by(1);
            } 
            0xc3 => {
                println!("SET 0 E");
                emu.inc_pc_by(1);
            } 
            0xc4 => {
                println!("SET 0 H");
                emu.inc_pc_by(1);
            } 
            0xc5 => {
                println!("SET 0 L");
                emu.inc_pc_by(1);
            } 
            0xc6 => {
                println!("SET 0 (HL)");
                emu.inc_pc_by(1);
            } 
            0xc7 => {
                println!("SET 0 A");
                emu.inc_pc_by(1);
            } 
            0xc8 => {
                println!("SET 1 B");
                emu.inc_pc_by(1);
            } 
            0xc9 => {
                println!("SET 1 C");
                emu.inc_pc_by(1);
            } 
            0xca => {
                println!("SET 1 D");
                emu.inc_pc_by(1);
            } 
            0xcb => {
                println!("SET 1 E");
                emu.inc_pc_by(1);
            } 
            0xcc => {
                println!("SET 1 H");
                emu.inc_pc_by(1);
            } 
            0xcd => {
                println!("SET 1 L");
                emu.inc_pc_by(1);
            } 
            0xce => {
                println!("SET 1 (HL)");
                emu.inc_pc_by(1);
            } 
            0xcf => {
                println!("SET 1 A");
                emu.inc_pc_by(1);
            } 
            0xd0 => {
                println!("SET 2 B");
                emu.inc_pc_by(1);
            } 
            0xd1 => {
                println!("SET 2 C");
                emu.inc_pc_by(1);
            } 
            0xd2 => {
                println!("SET 2 D");
                emu.inc_pc_by(1);
            } 
            0xd3 => {
                println!("SET 2 E");
                emu.inc_pc_by(1);
            } 
            0xd4 => {
                println!("SET 2 H");
                emu.inc_pc_by(1);
            } 
            0xd5 => {
                println!("SET 2 L");
                emu.inc_pc_by(1);
            } 
            0xd6 => {
                println!("SET 2 (HL)");
                emu.inc_pc_by(1);
            } 
            0xd7 => {
                println!("SET 2 A");
                emu.inc_pc_by(1);
            } 
            0xd8 => {
                println!("SET 3 B");
                emu.inc_pc_by(1);
            } 
            0xd9 => {
                println!("SET 3 C");
                emu.inc_pc_by(1);
            } 
            0xda => {
                println!("SET 3 D");
                emu.inc_pc_by(1);
            } 
            0xdb => {
                println!("SET 3 E");
                emu.inc_pc_by(1);
            } 
            0xdc => {
                println!("SET 3 H");
                emu.inc_pc_by(1);
            } 
            0xdd => {
                println!("SET 3 L");
                emu.inc_pc_by(1);
            } 
            0xde => {
                println!("SET 3 (HL)");
                emu.inc_pc_by(1);
            } 
            0xdf => {
                println!("SET 3 A");
                emu.inc_pc_by(1);
            } 
            0xe0 => {
                println!("SET 4 B");
                emu.inc_pc_by(1);
            } 
            0xe1 => {
                println!("SET 4 C");
                emu.inc_pc_by(1);
            } 
            0xe2 => {
                println!("SET 4 D");
                emu.inc_pc_by(1);
            } 
            0xe3 => {
                println!("SET 4 E");
                emu.inc_pc_by(1);
            } 
            0xe4 => {
                println!("SET 4 H");
                emu.inc_pc_by(1);
            } 
            0xe5 => {
                println!("SET 4 L");
                emu.inc_pc_by(1);
            } 
            0xe6 => {
                println!("SET 4 (HL)");
                emu.inc_pc_by(1);
            } 
            0xe7 => {
                println!("SET 4 A");
                emu.inc_pc_by(1);
            } 
            0xe8 => {
                println!("SET 5 B");
                emu.inc_pc_by(1);
            } 
            0xe9 => {
                println!("SET 5 C");
                emu.inc_pc_by(1);
            } 
            0xea => {
                println!("SET 5 D");
                emu.inc_pc_by(1);
            } 
            0xeb => {
                println!("SET 5 E");
                emu.inc_pc_by(1);
            } 
            0xec => {
                println!("SET 5 H");
                emu.inc_pc_by(1);
            } 
            0xed => {
                println!("SET 5 L");
                emu.inc_pc_by(1);
            } 
            0xee => {
                println!("SET 5 (HL)");
                emu.inc_pc_by(1);
            } 
            0xef => {
                println!("SET 5 A");
                emu.inc_pc_by(1);
            } 
            0xf0 => {
                println!("SET 6 B");
                emu.inc_pc_by(1);
            } 
            0xf1 => {
                println!("SET 6 C");
                emu.inc_pc_by(1);
            } 
            0xf2 => {
                println!("SET 6 D");
                emu.inc_pc_by(1);
            } 
            0xf3 => {
                println!("SET 6 E");
                emu.inc_pc_by(1);
            } 
            0xf4 => {
                println!("SET 6 H");
                emu.inc_pc_by(1);
            } 
            0xf5 => {
                println!("SET 6 L");
                emu.inc_pc_by(1);
            } 
            0xf6 => {
                println!("SET 6 (HL)");
                emu.inc_pc_by(1);
            } 
            0xf7 => {
                println!("SET 6 A");
                emu.inc_pc_by(1);
            } 
            0xf8 => {
                println!("SET 7 B");
                emu.inc_pc_by(1);
            } 
            0xf9 => {
                println!("SET 7 C");
                emu.inc_pc_by(1);
            } 
            0xfa => {
                println!("SET 7 D");
                emu.inc_pc_by(1);
            } 
            0xfb => {
                println!("SET 7 E");
                emu.inc_pc_by(1);
            } 
            0xfc => {
                println!("SET 7 H");
                emu.inc_pc_by(1);
            } 
            0xfd => {
                println!("SET 7 L");
                emu.inc_pc_by(1);
            } 
            0xfe => {
                println!("SET 7 (HL)");
                emu.inc_pc_by(1);
            } 
            0xff => {
                println!("SET 7 A");
                emu.inc_pc_by(1);
            } 
            _ => {
              println!("INVALID OPCODE: {:x}", cb_opcode);
              emu.inc_pc_by(1);
            }
        } 
    } 
    0xcc => { 
        println!("CALL Z a16");
        emu.inc_pc_by(3); 
    } 
    0xcd => { 
        println!("CALL a16 ");
        emu.inc_pc_by(3); 
    } 
    0xce => { 
        println!("ADC A d8");
        emu.inc_pc_by(2); 
    } 
    0xcf => { 
        println!("RST 08H ");
        emu.inc_pc_by(1); 
    } 
    0xd0 => { 
        println!("RET NC ");
        emu.inc_pc_by(1); 
    } 
    0xd1 => { 
        println!("POP DE ");
        emu.inc_pc_by(1); 
    } 
    0xd2 => { 
        println!("JP NC a16");
        emu.inc_pc_by(3); 
    } 
    0xd4 => { 
        println!("CALL NC a16");
        emu.inc_pc_by(3); 
    } 
    0xd5 => { 
        println!("PUSH DE ");
        emu.inc_pc_by(1); 
    } 
    0xd6 => { 
        println!("SUB d8 ");
        emu.inc_pc_by(2); 
    } 
    0xd7 => { 
        println!("RST 10H ");
        emu.inc_pc_by(1); 
    } 
    0xd8 => { 
        println!("RET C ");
        emu.inc_pc_by(1); 
    } 
    0xd9 => { 
        println!("RETI  ");
        emu.inc_pc_by(1); 
    } 
    0xda => { 
        println!("JP C a16");
        emu.inc_pc_by(3); 
    } 
    0xdc => { 
        println!("CALL C a16");
        emu.inc_pc_by(3); 
    } 
    0xde => { 
        println!("SBC A d8");
        emu.inc_pc_by(2); 
    } 
    0xdf => { 
        println!("RST 18H ");
        emu.inc_pc_by(1); 
    } 
    0xe0 => { 
        println!("LDH (a8) A");
        emu.inc_pc_by(2); 
    } 
    0xe1 => { 
        println!("POP HL ");
        emu.inc_pc_by(1); 
    } 
    0xe2 => { 
        println!("LD (C) A");
        emu.inc_pc_by(1); 
    } 
    0xe5 => { 
        println!("PUSH HL ");
        emu.inc_pc_by(1); 
    } 
    0xe6 => { 
        println!("AND d8 ");
        emu.inc_pc_by(2); 
    } 
    0xe7 => { 
        println!("RST 20H ");
        emu.inc_pc_by(1); 
    } 
    0xe8 => { 
        println!("ADD SP r8");
        emu.inc_pc_by(2); 
    } 
    0xe9 => { 
        println!("JP (HL) ");
        emu.inc_pc_by(1); 
    } 
    0xea => { 
        println!("LD (a16) A");
        emu.inc_pc_by(3); 
    } 
    0xee => { 
        println!("XOR d8 ");
        emu.inc_pc_by(2); 
    } 
    0xef => { 
        println!("RST 28H ");
        emu.inc_pc_by(1); 
    } 
    0xf0 => { 
        println!("LDH A (a8)");
        emu.inc_pc_by(2); 
    } 
    0xf1 => { 
        println!("POP AF ");
        emu.inc_pc_by(1); 
    } 
    0xf2 => { 
        println!("LD A (C)");
        emu.inc_pc_by(1); 
    } 
    0xf3 => { 
        println!("DI  ");
        emu.inc_pc_by(1); 
    } 
    0xf5 => { 
        println!("PUSH AF ");
        emu.inc_pc_by(1); 
    } 
    0xf6 => { 
        println!("OR d8 ");
        emu.inc_pc_by(2); 
    } 
    0xf7 => { 
        println!("RST 30H ");
        emu.inc_pc_by(1); 
    } 
    0xf8 => { 
        println!("LD HL SP+r8");
        emu.inc_pc_by(2); 
    } 
    0xf9 => { 
        println!("LD SP HL");
        emu.inc_pc_by(1); 
    } 
    0xfa => { 
        println!("LD A (a16)");
        emu.inc_pc_by(3); 
    } 
    0xfb => { 
        println!("EI  ");
        emu.inc_pc_by(1); 
    } 
    0xfe => { 
        println!("CP d8 ");
        emu.inc_pc_by(2); 
    } 
    0xff => { 
        println!("RST 38H ");
        emu.inc_pc_by(1); 
    } 
    _ => {
      println!("INVALID OPCODE: {:x}", opcode);
      emu.inc_pc_by(1);
    }
  }
}
