match opcode { 
  0x00 => { 
      println!("NOP  ");
      self.inc_pc_by(1); 
  } 
  0x01 => { 
      println!("LD BC d16");
      self.inc_pc_by(3); 
  } 
  0x02 => { 
      println!("LD (BC) A");
      self.inc_pc_by(1); 
  } 
  0x03 => { 
      println!("INC BC ");
      self.inc_pc_by(1); 
  } 
  0x04 => { 
      println!("INC B ");
      self.inc_pc_by(1); 
  } 
  0x05 => { 
      println!("DEC B ");
      self.inc_pc_by(1); 
  } 
  0x06 => { 
      println!("LD B d8");
      self.inc_pc_by(2); 
  } 
  0x07 => { 
      println!("RLCA  ");
      self.inc_pc_by(1); 
  } 
  0x08 => { 
      println!("LD (a16) SP");
      self.inc_pc_by(3); 
  } 
  0x09 => { 
      println!("ADD HL BC");
      self.inc_pc_by(1); 
  } 
  0x0a => { 
      println!("LD A (BC)");
      self.inc_pc_by(1); 
  } 
  0x0b => { 
      println!("DEC BC ");
      self.inc_pc_by(1); 
  } 
  0x0c => { 
      println!("INC C ");
      self.inc_pc_by(1); 
  } 
  0x0d => { 
      println!("DEC C ");
      self.inc_pc_by(1); 
  } 
  0x0e => { 
      println!("LD C d8");
      self.inc_pc_by(2); 
  } 
  0x0f => { 
      println!("RRCA  ");
      self.inc_pc_by(1); 
  } 
  0x10 => { 
      println!("STOP 0 ");
      self.inc_pc_by(1); 
  } 
  0x11 => { 
      println!("LD DE d16");
      self.inc_pc_by(3); 
  } 
  0x12 => { 
      println!("LD (DE) A");
      self.inc_pc_by(1); 
  } 
  0x13 => { 
      println!("INC DE ");
      self.inc_pc_by(1); 
  } 
  0x14 => { 
      println!("INC D ");
      self.inc_pc_by(1); 
  } 
  0x15 => { 
      println!("DEC D ");
      self.inc_pc_by(1); 
  } 
  0x16 => { 
      println!("LD D d8");
      self.inc_pc_by(2); 
  } 
  0x17 => { 
      println!("RLA  ");
      self.inc_pc_by(1); 
  } 
  0x18 => { 
      println!("JR r8 ");
      self.inc_pc_by(2); 
  } 
  0x19 => { 
      println!("ADD HL DE");
      self.inc_pc_by(1); 
  } 
  0x1a => { 
      println!("LD A (DE)");
      self.inc_pc_by(1); 
  } 
  0x1b => { 
      println!("DEC DE ");
      self.inc_pc_by(1); 
  } 
  0x1c => { 
      println!("INC E ");
      self.inc_pc_by(1); 
  } 
  0x1d => { 
      println!("DEC E ");
      self.inc_pc_by(1); 
  } 
  0x1e => { 
      println!("LD E d8");
      self.inc_pc_by(2); 
  } 
  0x1f => { 
      println!("RRA  ");
      self.inc_pc_by(1); 
  } 
  0x20 => { 
      println!("JR NZ r8");
      self.inc_pc_by(2); 
  } 
  0x21 => { 
      println!("LD HL d16");
      self.inc_pc_by(3); 
  } 
  0x22 => { 
      println!("LD (HL+) A");
      self.inc_pc_by(1); 
  } 
  0x23 => { 
      println!("INC HL ");
      self.inc_pc_by(1); 
  } 
  0x24 => { 
      println!("INC H ");
      self.inc_pc_by(1); 
  } 
  0x25 => { 
      println!("DEC H ");
      self.inc_pc_by(1); 
  } 
  0x26 => { 
      println!("LD H d8");
      self.inc_pc_by(2); 
  } 
  0x27 => { 
      println!("DAA  ");
      self.inc_pc_by(1); 
  } 
  0x28 => { 
      println!("JR Z r8");
      self.inc_pc_by(2); 
  } 
  0x29 => { 
      println!("ADD HL HL");
      self.inc_pc_by(1); 
  } 
  0x2a => { 
      println!("LD A (HL+)");
      self.inc_pc_by(1); 
  } 
  0x2b => { 
      println!("DEC HL ");
      self.inc_pc_by(1); 
  } 
  0x2c => { 
      println!("INC L ");
      self.inc_pc_by(1); 
  } 
  0x2d => { 
      println!("DEC L ");
      self.inc_pc_by(1); 
  } 
  0x2e => { 
      println!("LD L d8");
      self.inc_pc_by(2); 
  } 
  0x2f => { 
      println!("CPL  ");
      self.inc_pc_by(1); 
  } 
  0x30 => { 
      println!("JR NC r8");
      self.inc_pc_by(2); 
  } 
  0x31 => { 
      println!("LD SP d16");
      self.inc_pc_by(3); 
  } 
  0x32 => { 
      println!("LD (HL-) A");
      self.inc_pc_by(1); 
  } 
  0x33 => { 
      println!("INC SP ");
      self.inc_pc_by(1); 
  } 
  0x34 => { 
      println!("INC (HL) ");
      self.inc_pc_by(1); 
  } 
  0x35 => { 
      println!("DEC (HL) ");
      self.inc_pc_by(1); 
  } 
  0x36 => { 
      println!("LD (HL) d8");
      self.inc_pc_by(2); 
  } 
  0x37 => { 
      println!("SCF  ");
      self.inc_pc_by(1); 
  } 
  0x38 => { 
      println!("JR C r8");
      self.inc_pc_by(2); 
  } 
  0x39 => { 
      println!("ADD HL SP");
      self.inc_pc_by(1); 
  } 
  0x3a => { 
      println!("LD A (HL-)");
      self.inc_pc_by(1); 
  } 
  0x3b => { 
      println!("DEC SP ");
      self.inc_pc_by(1); 
  } 
  0x3c => { 
      println!("INC A ");
      self.inc_pc_by(1); 
  } 
  0x3d => { 
      println!("DEC A ");
      self.inc_pc_by(1); 
  } 
  0x3e => { 
      println!("LD A d8");
      self.inc_pc_by(2); 
  } 
  0x3f => { 
      println!("CCF  ");
      self.inc_pc_by(1); 
  } 
  0x40 => { 
      println!("LD B B");
      self.inc_pc_by(1); 
  } 
  0x41 => { 
      println!("LD B C");
      self.inc_pc_by(1); 
  } 
  0x42 => { 
      println!("LD B D");
      self.inc_pc_by(1); 
  } 
  0x43 => { 
      println!("LD B E");
      self.inc_pc_by(1); 
  } 
  0x44 => { 
      println!("LD B H");
      self.inc_pc_by(1); 
  } 
  0x45 => { 
      println!("LD B L");
      self.inc_pc_by(1); 
  } 
  0x46 => { 
      println!("LD B (HL)");
      self.inc_pc_by(1); 
  } 
  0x47 => { 
      println!("LD B A");
      self.inc_pc_by(1); 
  } 
  0x48 => { 
      println!("LD C B");
      self.inc_pc_by(1); 
  } 
  0x49 => { 
      println!("LD C C");
      self.inc_pc_by(1); 
  } 
  0x4a => { 
      println!("LD C D");
      self.inc_pc_by(1); 
  } 
  0x4b => { 
      println!("LD C E");
      self.inc_pc_by(1); 
  } 
  0x4c => { 
      println!("LD C H");
      self.inc_pc_by(1); 
  } 
  0x4d => { 
      println!("LD C L");
      self.inc_pc_by(1); 
  } 
  0x4e => { 
      println!("LD C (HL)");
      self.inc_pc_by(1); 
  } 
  0x4f => { 
      println!("LD C A");
      self.inc_pc_by(1); 
  } 
  0x50 => { 
      println!("LD D B");
      self.inc_pc_by(1); 
  } 
  0x51 => { 
      println!("LD D C");
      self.inc_pc_by(1); 
  } 
  0x52 => { 
      println!("LD D D");
      self.inc_pc_by(1); 
  } 
  0x53 => { 
      println!("LD D E");
      self.inc_pc_by(1); 
  } 
  0x54 => { 
      println!("LD D H");
      self.inc_pc_by(1); 
  } 
  0x55 => { 
      println!("LD D L");
      self.inc_pc_by(1); 
  } 
  0x56 => { 
      println!("LD D (HL)");
      self.inc_pc_by(1); 
  } 
  0x57 => { 
      println!("LD D A");
      self.inc_pc_by(1); 
  } 
  0x58 => { 
      println!("LD E B");
      self.inc_pc_by(1); 
  } 
  0x59 => { 
      println!("LD E C");
      self.inc_pc_by(1); 
  } 
  0x5a => { 
      println!("LD E D");
      self.inc_pc_by(1); 
  } 
  0x5b => { 
      println!("LD E E");
      self.inc_pc_by(1); 
  } 
  0x5c => { 
      println!("LD E H");
      self.inc_pc_by(1); 
  } 
  0x5d => { 
      println!("LD E L");
      self.inc_pc_by(1); 
  } 
  0x5e => { 
      println!("LD E (HL)");
      self.inc_pc_by(1); 
  } 
  0x5f => { 
      println!("LD E A");
      self.inc_pc_by(1); 
  } 
  0x60 => { 
      println!("LD H B");
      self.inc_pc_by(1); 
  } 
  0x61 => { 
      println!("LD H C");
      self.inc_pc_by(1); 
  } 
  0x62 => { 
      println!("LD H D");
      self.inc_pc_by(1); 
  } 
  0x63 => { 
      println!("LD H E");
      self.inc_pc_by(1); 
  } 
  0x64 => { 
      println!("LD H H");
      self.inc_pc_by(1); 
  } 
  0x65 => { 
      println!("LD H L");
      self.inc_pc_by(1); 
  } 
  0x66 => { 
      println!("LD H (HL)");
      self.inc_pc_by(1); 
  } 
  0x67 => { 
      println!("LD H A");
      self.inc_pc_by(1); 
  } 
  0x68 => { 
      println!("LD L B");
      self.inc_pc_by(1); 
  } 
  0x69 => { 
      println!("LD L C");
      self.inc_pc_by(1); 
  } 
  0x6a => { 
      println!("LD L D");
      self.inc_pc_by(1); 
  } 
  0x6b => { 
      println!("LD L E");
      self.inc_pc_by(1); 
  } 
  0x6c => { 
      println!("LD L H");
      self.inc_pc_by(1); 
  } 
  0x6d => { 
      println!("LD L L");
      self.inc_pc_by(1); 
  } 
  0x6e => { 
      println!("LD L (HL)");
      self.inc_pc_by(1); 
  } 
  0x6f => { 
      println!("LD L A");
      self.inc_pc_by(1); 
  } 
  0x70 => { 
      println!("LD (HL) B");
      self.inc_pc_by(1); 
  } 
  0x71 => { 
      println!("LD (HL) C");
      self.inc_pc_by(1); 
  } 
  0x72 => { 
      println!("LD (HL) D");
      self.inc_pc_by(1); 
  } 
  0x73 => { 
      println!("LD (HL) E");
      self.inc_pc_by(1); 
  } 
  0x74 => { 
      println!("LD (HL) H");
      self.inc_pc_by(1); 
  } 
  0x75 => { 
      println!("LD (HL) L");
      self.inc_pc_by(1); 
  } 
  0x76 => { 
      println!("HALT  ");
      self.inc_pc_by(1); 
  } 
  0x77 => { 
      println!("LD (HL) A");
      self.inc_pc_by(1); 
  } 
  0x78 => { 
      println!("LD A B");
      self.inc_pc_by(1); 
  } 
  0x79 => { 
      println!("LD A C");
      self.inc_pc_by(1); 
  } 
  0x7a => { 
      println!("LD A D");
      self.inc_pc_by(1); 
  } 
  0x7b => { 
      println!("LD A E");
      self.inc_pc_by(1); 
  } 
  0x7c => { 
      println!("LD A H");
      self.inc_pc_by(1); 
  } 
  0x7d => { 
      println!("LD A L");
      self.inc_pc_by(1); 
  } 
  0x7e => { 
      println!("LD A (HL)");
      self.inc_pc_by(1); 
  } 
  0x7f => { 
      println!("LD A A");
      self.inc_pc_by(1); 
  } 
  0x80 => { 
      println!("ADD A B");
      self.inc_pc_by(1); 
  } 
  0x81 => { 
      println!("ADD A C");
      self.inc_pc_by(1); 
  } 
  0x82 => { 
      println!("ADD A D");
      self.inc_pc_by(1); 
  } 
  0x83 => { 
      println!("ADD A E");
      self.inc_pc_by(1); 
  } 
  0x84 => { 
      println!("ADD A H");
      self.inc_pc_by(1); 
  } 
  0x85 => { 
      println!("ADD A L");
      self.inc_pc_by(1); 
  } 
  0x86 => { 
      println!("ADD A (HL)");
      self.inc_pc_by(1); 
  } 
  0x87 => { 
      println!("ADD A A");
      self.inc_pc_by(1); 
  } 
  0x88 => { 
      println!("ADC A B");
      self.inc_pc_by(1); 
  } 
  0x89 => { 
      println!("ADC A C");
      self.inc_pc_by(1); 
  } 
  0x8a => { 
      println!("ADC A D");
      self.inc_pc_by(1); 
  } 
  0x8b => { 
      println!("ADC A E");
      self.inc_pc_by(1); 
  } 
  0x8c => { 
      println!("ADC A H");
      self.inc_pc_by(1); 
  } 
  0x8d => { 
      println!("ADC A L");
      self.inc_pc_by(1); 
  } 
  0x8e => { 
      println!("ADC A (HL)");
      self.inc_pc_by(1); 
  } 
  0x8f => { 
      println!("ADC A A");
      self.inc_pc_by(1); 
  } 
  0x90 => { 
      println!("SUB B ");
      self.inc_pc_by(1); 
  } 
  0x91 => { 
      println!("SUB C ");
      self.inc_pc_by(1); 
  } 
  0x92 => { 
      println!("SUB D ");
      self.inc_pc_by(1); 
  } 
  0x93 => { 
      println!("SUB E ");
      self.inc_pc_by(1); 
  } 
  0x94 => { 
      println!("SUB H ");
      self.inc_pc_by(1); 
  } 
  0x95 => { 
      println!("SUB L ");
      self.inc_pc_by(1); 
  } 
  0x96 => { 
      println!("SUB (HL) ");
      self.inc_pc_by(1); 
  } 
  0x97 => { 
      println!("SUB A ");
      self.inc_pc_by(1); 
  } 
  0x98 => { 
      println!("SBC A B");
      self.inc_pc_by(1); 
  } 
  0x99 => { 
      println!("SBC A C");
      self.inc_pc_by(1); 
  } 
  0x9a => { 
      println!("SBC A D");
      self.inc_pc_by(1); 
  } 
  0x9b => { 
      println!("SBC A E");
      self.inc_pc_by(1); 
  } 
  0x9c => { 
      println!("SBC A H");
      self.inc_pc_by(1); 
  } 
  0x9d => { 
      println!("SBC A L");
      self.inc_pc_by(1); 
  } 
  0x9e => { 
      println!("SBC A (HL)");
      self.inc_pc_by(1); 
  } 
  0x9f => { 
      println!("SBC A A");
      self.inc_pc_by(1); 
  } 
  0xa0 => { 
      println!("AND B ");
      self.inc_pc_by(1); 
  } 
  0xa1 => { 
      println!("AND C ");
      self.inc_pc_by(1); 
  } 
  0xa2 => { 
      println!("AND D ");
      self.inc_pc_by(1); 
  } 
  0xa3 => { 
      println!("AND E ");
      self.inc_pc_by(1); 
  } 
  0xa4 => { 
      println!("AND H ");
      self.inc_pc_by(1); 
  } 
  0xa5 => { 
      println!("AND L ");
      self.inc_pc_by(1); 
  } 
  0xa6 => { 
      println!("AND (HL) ");
      self.inc_pc_by(1); 
  } 
  0xa7 => { 
      println!("AND A ");
      self.inc_pc_by(1); 
  } 
  0xa8 => { 
      println!("XOR B ");
      self.inc_pc_by(1); 
  } 
  0xa9 => { 
      println!("XOR C ");
      self.inc_pc_by(1); 
  } 
  0xaa => { 
      println!("XOR D ");
      self.inc_pc_by(1); 
  } 
  0xab => { 
      println!("XOR E ");
      self.inc_pc_by(1); 
  } 
  0xac => { 
      println!("XOR H ");
      self.inc_pc_by(1); 
  } 
  0xad => { 
      println!("XOR L ");
      self.inc_pc_by(1); 
  } 
  0xae => { 
      println!("XOR (HL) ");
      self.inc_pc_by(1); 
  } 
  0xaf => { 
      println!("XOR A ");
      self.inc_pc_by(1); 
  } 
  0xb0 => { 
      println!("OR B ");
      self.inc_pc_by(1); 
  } 
  0xb1 => { 
      println!("OR C ");
      self.inc_pc_by(1); 
  } 
  0xb2 => { 
      println!("OR D ");
      self.inc_pc_by(1); 
  } 
  0xb3 => { 
      println!("OR E ");
      self.inc_pc_by(1); 
  } 
  0xb4 => { 
      println!("OR H ");
      self.inc_pc_by(1); 
  } 
  0xb5 => { 
      println!("OR L ");
      self.inc_pc_by(1); 
  } 
  0xb6 => { 
      println!("OR (HL) ");
      self.inc_pc_by(1); 
  } 
  0xb7 => { 
      println!("OR A ");
      self.inc_pc_by(1); 
  } 
  0xb8 => { 
      println!("CP B ");
      self.inc_pc_by(1); 
  } 
  0xb9 => { 
      println!("CP C ");
      self.inc_pc_by(1); 
  } 
  0xba => { 
      println!("CP D ");
      self.inc_pc_by(1); 
  } 
  0xbb => { 
      println!("CP E ");
      self.inc_pc_by(1); 
  } 
  0xbc => { 
      println!("CP H ");
      self.inc_pc_by(1); 
  } 
  0xbd => { 
      println!("CP L ");
      self.inc_pc_by(1); 
  } 
  0xbe => { 
      println!("CP (HL) ");
      self.inc_pc_by(1); 
  } 
  0xbf => { 
      println!("CP A ");
      self.inc_pc_by(1); 
  } 
  0xc0 => { 
      println!("RET NZ ");
      self.inc_pc_by(1); 
  } 
  0xc1 => { 
      println!("POP BC ");
      self.inc_pc_by(1); 
  } 
  0xc2 => { 
      println!("JP NZ a16");
      self.inc_pc_by(3); 
  } 
  0xc3 => { 
      println!("JP a16 ");
      self.inc_pc_by(3); 
  } 
  0xc4 => { 
      println!("CALL NZ a16");
      self.inc_pc_by(3); 
  } 
  0xc5 => { 
      println!("PUSH BC ");
      self.inc_pc_by(1); 
  } 
  0xc6 => { 
      println!("ADD A d8");
      self.inc_pc_by(2); 
  } 
  0xc7 => { 
      println!("RST 00H ");
      self.inc_pc_by(1); 
  } 
  0xc8 => { 
      println!("RET Z ");
      self.inc_pc_by(1); 
  } 
  0xc9 => { 
      println!("RET  ");
      self.inc_pc_by(1); 
  } 
  0xca => { 
      println!("JP Z a16");
      self.inc_pc_by(3); 
  } 
  0xcb => { 
      not impl
      self.inc_pc_by(1);
      let cb_opcode = self.current_opcode();
      match cb_opcode { 
          0x00 => {
              println!("RLC B ");
              self.inc_pc_by(1);
          } 
          0x01 => {
              println!("RLC C ");
              self.inc_pc_by(1);
          } 
          0x02 => {
              println!("RLC D ");
              self.inc_pc_by(1);
          } 
          0x03 => {
              println!("RLC E ");
              self.inc_pc_by(1);
          } 
          0x04 => {
              println!("RLC H ");
              self.inc_pc_by(1);
          } 
          0x05 => {
              println!("RLC L ");
              self.inc_pc_by(1);
          } 
          0x06 => {
              println!("RLC (HL) ");
              self.inc_pc_by(1);
          } 
          0x07 => {
              println!("RLC A ");
              self.inc_pc_by(1);
          } 
          0x08 => {
              println!("RRC B ");
              self.inc_pc_by(1);
          } 
          0x09 => {
              println!("RRC C ");
              self.inc_pc_by(1);
          } 
          0x0a => {
              println!("RRC D ");
              self.inc_pc_by(1);
          } 
          0x0b => {
              println!("RRC E ");
              self.inc_pc_by(1);
          } 
          0x0c => {
              println!("RRC H ");
              self.inc_pc_by(1);
          } 
          0x0d => {
              println!("RRC L ");
              self.inc_pc_by(1);
          } 
          0x0e => {
              println!("RRC (HL) ");
              self.inc_pc_by(1);
          } 
          0x0f => {
              println!("RRC A ");
              self.inc_pc_by(1);
          } 
          0x10 => {
              println!("RL B ");
              self.inc_pc_by(1);
          } 
          0x11 => {
              println!("RL C ");
              self.inc_pc_by(1);
          } 
          0x12 => {
              println!("RL D ");
              self.inc_pc_by(1);
          } 
          0x13 => {
              println!("RL E ");
              self.inc_pc_by(1);
          } 
          0x14 => {
              println!("RL H ");
              self.inc_pc_by(1);
          } 
          0x15 => {
              println!("RL L ");
              self.inc_pc_by(1);
          } 
          0x16 => {
              println!("RL (HL) ");
              self.inc_pc_by(1);
          } 
          0x17 => {
              println!("RL A ");
              self.inc_pc_by(1);
          } 
          0x18 => {
              println!("RR B ");
              self.inc_pc_by(1);
          } 
          0x19 => {
              println!("RR C ");
              self.inc_pc_by(1);
          } 
          0x1a => {
              println!("RR D ");
              self.inc_pc_by(1);
          } 
          0x1b => {
              println!("RR E ");
              self.inc_pc_by(1);
          } 
          0x1c => {
              println!("RR H ");
              self.inc_pc_by(1);
          } 
          0x1d => {
              println!("RR L ");
              self.inc_pc_by(1);
          } 
          0x1e => {
              println!("RR (HL) ");
              self.inc_pc_by(1);
          } 
          0x1f => {
              println!("RR A ");
              self.inc_pc_by(1);
          } 
          0x20 => {
              println!("SLA B ");
              self.inc_pc_by(1);
          } 
          0x21 => {
              println!("SLA C ");
              self.inc_pc_by(1);
          } 
          0x22 => {
              println!("SLA D ");
              self.inc_pc_by(1);
          } 
          0x23 => {
              println!("SLA E ");
              self.inc_pc_by(1);
          } 
          0x24 => {
              println!("SLA H ");
              self.inc_pc_by(1);
          } 
          0x25 => {
              println!("SLA L ");
              self.inc_pc_by(1);
          } 
          0x26 => {
              println!("SLA (HL) ");
              self.inc_pc_by(1);
          } 
          0x27 => {
              println!("SLA A ");
              self.inc_pc_by(1);
          } 
          0x28 => {
              println!("SRA B ");
              self.inc_pc_by(1);
          } 
          0x29 => {
              println!("SRA C ");
              self.inc_pc_by(1);
          } 
          0x2a => {
              println!("SRA D ");
              self.inc_pc_by(1);
          } 
          0x2b => {
              println!("SRA E ");
              self.inc_pc_by(1);
          } 
          0x2c => {
              println!("SRA H ");
              self.inc_pc_by(1);
          } 
          0x2d => {
              println!("SRA L ");
              self.inc_pc_by(1);
          } 
          0x2e => {
              println!("SRA (HL) ");
              self.inc_pc_by(1);
          } 
          0x2f => {
              println!("SRA A ");
              self.inc_pc_by(1);
          } 
          0x30 => {
              println!("SWAP B ");
              self.inc_pc_by(1);
          } 
          0x31 => {
              println!("SWAP C ");
              self.inc_pc_by(1);
          } 
          0x32 => {
              println!("SWAP D ");
              self.inc_pc_by(1);
          } 
          0x33 => {
              println!("SWAP E ");
              self.inc_pc_by(1);
          } 
          0x34 => {
              println!("SWAP H ");
              self.inc_pc_by(1);
          } 
          0x35 => {
              println!("SWAP L ");
              self.inc_pc_by(1);
          } 
          0x36 => {
              println!("SWAP (HL) ");
              self.inc_pc_by(1);
          } 
          0x37 => {
              println!("SWAP A ");
              self.inc_pc_by(1);
          } 
          0x38 => {
              println!("SRL B ");
              self.inc_pc_by(1);
          } 
          0x39 => {
              println!("SRL C ");
              self.inc_pc_by(1);
          } 
          0x3a => {
              println!("SRL D ");
              self.inc_pc_by(1);
          } 
          0x3b => {
              println!("SRL E ");
              self.inc_pc_by(1);
          } 
          0x3c => {
              println!("SRL H ");
              self.inc_pc_by(1);
          } 
          0x3d => {
              println!("SRL L ");
              self.inc_pc_by(1);
          } 
          0x3e => {
              println!("SRL (HL) ");
              self.inc_pc_by(1);
          } 
          0x3f => {
              println!("SRL A ");
              self.inc_pc_by(1);
          } 
          0x40 => {
              println!("BIT 0 B");
              self.inc_pc_by(1);
          } 
          0x41 => {
              println!("BIT 0 C");
              self.inc_pc_by(1);
          } 
          0x42 => {
              println!("BIT 0 D");
              self.inc_pc_by(1);
          } 
          0x43 => {
              println!("BIT 0 E");
              self.inc_pc_by(1);
          } 
          0x44 => {
              println!("BIT 0 H");
              self.inc_pc_by(1);
          } 
          0x45 => {
              println!("BIT 0 L");
              self.inc_pc_by(1);
          } 
          0x46 => {
              println!("BIT 0 (HL)");
              self.inc_pc_by(1);
          } 
          0x47 => {
              println!("BIT 0 A");
              self.inc_pc_by(1);
          } 
          0x48 => {
              println!("BIT 1 B");
              self.inc_pc_by(1);
          } 
          0x49 => {
              println!("BIT 1 C");
              self.inc_pc_by(1);
          } 
          0x4a => {
              println!("BIT 1 D");
              self.inc_pc_by(1);
          } 
          0x4b => {
              println!("BIT 1 E");
              self.inc_pc_by(1);
          } 
          0x4c => {
              println!("BIT 1 H");
              self.inc_pc_by(1);
          } 
          0x4d => {
              println!("BIT 1 L");
              self.inc_pc_by(1);
          } 
          0x4e => {
              println!("BIT 1 (HL)");
              self.inc_pc_by(1);
          } 
          0x4f => {
              println!("BIT 1 A");
              self.inc_pc_by(1);
          } 
          0x50 => {
              println!("BIT 2 B");
              self.inc_pc_by(1);
          } 
          0x51 => {
              println!("BIT 2 C");
              self.inc_pc_by(1);
          } 
          0x52 => {
              println!("BIT 2 D");
              self.inc_pc_by(1);
          } 
          0x53 => {
              println!("BIT 2 E");
              self.inc_pc_by(1);
          } 
          0x54 => {
              println!("BIT 2 H");
              self.inc_pc_by(1);
          } 
          0x55 => {
              println!("BIT 2 L");
              self.inc_pc_by(1);
          } 
          0x56 => {
              println!("BIT 2 (HL)");
              self.inc_pc_by(1);
          } 
          0x57 => {
              println!("BIT 2 A");
              self.inc_pc_by(1);
          } 
          0x58 => {
              println!("BIT 3 B");
              self.inc_pc_by(1);
          } 
          0x59 => {
              println!("BIT 3 C");
              self.inc_pc_by(1);
          } 
          0x5a => {
              println!("BIT 3 D");
              self.inc_pc_by(1);
          } 
          0x5b => {
              println!("BIT 3 E");
              self.inc_pc_by(1);
          } 
          0x5c => {
              println!("BIT 3 H");
              self.inc_pc_by(1);
          } 
          0x5d => {
              println!("BIT 3 L");
              self.inc_pc_by(1);
          } 
          0x5e => {
              println!("BIT 3 (HL)");
              self.inc_pc_by(1);
          } 
          0x5f => {
              println!("BIT 3 A");
              self.inc_pc_by(1);
          } 
          0x60 => {
              println!("BIT 4 B");
              self.inc_pc_by(1);
          } 
          0x61 => {
              println!("BIT 4 C");
              self.inc_pc_by(1);
          } 
          0x62 => {
              println!("BIT 4 D");
              self.inc_pc_by(1);
          } 
          0x63 => {
              println!("BIT 4 E");
              self.inc_pc_by(1);
          } 
          0x64 => {
              println!("BIT 4 H");
              self.inc_pc_by(1);
          } 
          0x65 => {
              println!("BIT 4 L");
              self.inc_pc_by(1);
          } 
          0x66 => {
              println!("BIT 4 (HL)");
              self.inc_pc_by(1);
          } 
          0x67 => {
              println!("BIT 4 A");
              self.inc_pc_by(1);
          } 
          0x68 => {
              println!("BIT 5 B");
              self.inc_pc_by(1);
          } 
          0x69 => {
              println!("BIT 5 C");
              self.inc_pc_by(1);
          } 
          0x6a => {
              println!("BIT 5 D");
              self.inc_pc_by(1);
          } 
          0x6b => {
              println!("BIT 5 E");
              self.inc_pc_by(1);
          } 
          0x6c => {
              println!("BIT 5 H");
              self.inc_pc_by(1);
          } 
          0x6d => {
              println!("BIT 5 L");
              self.inc_pc_by(1);
          } 
          0x6e => {
              println!("BIT 5 (HL)");
              self.inc_pc_by(1);
          } 
          0x6f => {
              println!("BIT 5 A");
              self.inc_pc_by(1);
          } 
          0x70 => {
              println!("BIT 6 B");
              self.inc_pc_by(1);
          } 
          0x71 => {
              println!("BIT 6 C");
              self.inc_pc_by(1);
          } 
          0x72 => {
              println!("BIT 6 D");
              self.inc_pc_by(1);
          } 
          0x73 => {
              println!("BIT 6 E");
              self.inc_pc_by(1);
          } 
          0x74 => {
              println!("BIT 6 H");
              self.inc_pc_by(1);
          } 
          0x75 => {
              println!("BIT 6 L");
              self.inc_pc_by(1);
          } 
          0x76 => {
              println!("BIT 6 (HL)");
              self.inc_pc_by(1);
          } 
          0x77 => {
              println!("BIT 6 A");
              self.inc_pc_by(1);
          } 
          0x78 => {
              println!("BIT 7 B");
              self.inc_pc_by(1);
          } 
          0x79 => {
              println!("BIT 7 C");
              self.inc_pc_by(1);
          } 
          0x7a => {
              println!("BIT 7 D");
              self.inc_pc_by(1);
          } 
          0x7b => {
              println!("BIT 7 E");
              self.inc_pc_by(1);
          } 
          0x7c => {
              println!("BIT 7 H");
              self.inc_pc_by(1);
          } 
          0x7d => {
              println!("BIT 7 L");
              self.inc_pc_by(1);
          } 
          0x7e => {
              println!("BIT 7 (HL)");
              self.inc_pc_by(1);
          } 
          0x7f => {
              println!("BIT 7 A");
              self.inc_pc_by(1);
          } 
          0x80 => {
              println!("RES 0 B");
              self.inc_pc_by(1);
          } 
          0x81 => {
              println!("RES 0 C");
              self.inc_pc_by(1);
          } 
          0x82 => {
              println!("RES 0 D");
              self.inc_pc_by(1);
          } 
          0x83 => {
              println!("RES 0 E");
              self.inc_pc_by(1);
          } 
          0x84 => {
              println!("RES 0 H");
              self.inc_pc_by(1);
          } 
          0x85 => {
              println!("RES 0 L");
              self.inc_pc_by(1);
          } 
          0x86 => {
              println!("RES 0 (HL)");
              self.inc_pc_by(1);
          } 
          0x87 => {
              println!("RES 0 A");
              self.inc_pc_by(1);
          } 
          0x88 => {
              println!("RES 1 B");
              self.inc_pc_by(1);
          } 
          0x89 => {
              println!("RES 1 C");
              self.inc_pc_by(1);
          } 
          0x8a => {
              println!("RES 1 D");
              self.inc_pc_by(1);
          } 
          0x8b => {
              println!("RES 1 E");
              self.inc_pc_by(1);
          } 
          0x8c => {
              println!("RES 1 H");
              self.inc_pc_by(1);
          } 
          0x8d => {
              println!("RES 1 L");
              self.inc_pc_by(1);
          } 
          0x8e => {
              println!("RES 1 (HL)");
              self.inc_pc_by(1);
          } 
          0x8f => {
              println!("RES 1 A");
              self.inc_pc_by(1);
          } 
          0x90 => {
              println!("RES 2 B");
              self.inc_pc_by(1);
          } 
          0x91 => {
              println!("RES 2 C");
              self.inc_pc_by(1);
          } 
          0x92 => {
              println!("RES 2 D");
              self.inc_pc_by(1);
          } 
          0x93 => {
              println!("RES 2 E");
              self.inc_pc_by(1);
          } 
          0x94 => {
              println!("RES 2 H");
              self.inc_pc_by(1);
          } 
          0x95 => {
              println!("RES 2 L");
              self.inc_pc_by(1);
          } 
          0x96 => {
              println!("RES 2 (HL)");
              self.inc_pc_by(1);
          } 
          0x97 => {
              println!("RES 2 A");
              self.inc_pc_by(1);
          } 
          0x98 => {
              println!("RES 3 B");
              self.inc_pc_by(1);
          } 
          0x99 => {
              println!("RES 3 C");
              self.inc_pc_by(1);
          } 
          0x9a => {
              println!("RES 3 D");
              self.inc_pc_by(1);
          } 
          0x9b => {
              println!("RES 3 E");
              self.inc_pc_by(1);
          } 
          0x9c => {
              println!("RES 3 H");
              self.inc_pc_by(1);
          } 
          0x9d => {
              println!("RES 3 L");
              self.inc_pc_by(1);
          } 
          0x9e => {
              println!("RES 3 (HL)");
              self.inc_pc_by(1);
          } 
          0x9f => {
              println!("RES 3 A");
              self.inc_pc_by(1);
          } 
          0xa0 => {
              println!("RES 4 B");
              self.inc_pc_by(1);
          } 
          0xa1 => {
              println!("RES 4 C");
              self.inc_pc_by(1);
          } 
          0xa2 => {
              println!("RES 4 D");
              self.inc_pc_by(1);
          } 
          0xa3 => {
              println!("RES 4 E");
              self.inc_pc_by(1);
          } 
          0xa4 => {
              println!("RES 4 H");
              self.inc_pc_by(1);
          } 
          0xa5 => {
              println!("RES 4 L");
              self.inc_pc_by(1);
          } 
          0xa6 => {
              println!("RES 4 (HL)");
              self.inc_pc_by(1);
          } 
          0xa7 => {
              println!("RES 4 A");
              self.inc_pc_by(1);
          } 
          0xa8 => {
              println!("RES 5 B");
              self.inc_pc_by(1);
          } 
          0xa9 => {
              println!("RES 5 C");
              self.inc_pc_by(1);
          } 
          0xaa => {
              println!("RES 5 D");
              self.inc_pc_by(1);
          } 
          0xab => {
              println!("RES 5 E");
              self.inc_pc_by(1);
          } 
          0xac => {
              println!("RES 5 H");
              self.inc_pc_by(1);
          } 
          0xad => {
              println!("RES 5 L");
              self.inc_pc_by(1);
          } 
          0xae => {
              println!("RES 5 (HL)");
              self.inc_pc_by(1);
          } 
          0xaf => {
              println!("RES 5 A");
              self.inc_pc_by(1);
          } 
          0xb0 => {
              println!("RES 6 B");
              self.inc_pc_by(1);
          } 
          0xb1 => {
              println!("RES 6 C");
              self.inc_pc_by(1);
          } 
          0xb2 => {
              println!("RES 6 D");
              self.inc_pc_by(1);
          } 
          0xb3 => {
              println!("RES 6 E");
              self.inc_pc_by(1);
          } 
          0xb4 => {
              println!("RES 6 H");
              self.inc_pc_by(1);
          } 
          0xb5 => {
              println!("RES 6 L");
              self.inc_pc_by(1);
          } 
          0xb6 => {
              println!("RES 6 (HL)");
              self.inc_pc_by(1);
          } 
          0xb7 => {
              println!("RES 6 A");
              self.inc_pc_by(1);
          } 
          0xb8 => {
              println!("RES 7 B");
              self.inc_pc_by(1);
          } 
          0xb9 => {
              println!("RES 7 C");
              self.inc_pc_by(1);
          } 
          0xba => {
              println!("RES 7 D");
              self.inc_pc_by(1);
          } 
          0xbb => {
              println!("RES 7 E");
              self.inc_pc_by(1);
          } 
          0xbc => {
              println!("RES 7 H");
              self.inc_pc_by(1);
          } 
          0xbd => {
              println!("RES 7 L");
              self.inc_pc_by(1);
          } 
          0xbe => {
              println!("RES 7 (HL)");
              self.inc_pc_by(1);
          } 
          0xbf => {
              println!("RES 7 A");
              self.inc_pc_by(1);
          } 
          0xc0 => {
              println!("SET 0 B");
              self.inc_pc_by(1);
          } 
          0xc1 => {
              println!("SET 0 C");
              self.inc_pc_by(1);
          } 
          0xc2 => {
              println!("SET 0 D");
              self.inc_pc_by(1);
          } 
          0xc3 => {
              println!("SET 0 E");
              self.inc_pc_by(1);
          } 
          0xc4 => {
              println!("SET 0 H");
              self.inc_pc_by(1);
          } 
          0xc5 => {
              println!("SET 0 L");
              self.inc_pc_by(1);
          } 
          0xc6 => {
              println!("SET 0 (HL)");
              self.inc_pc_by(1);
          } 
          0xc7 => {
              println!("SET 0 A");
              self.inc_pc_by(1);
          } 
          0xc8 => {
              println!("SET 1 B");
              self.inc_pc_by(1);
          } 
          0xc9 => {
              println!("SET 1 C");
              self.inc_pc_by(1);
          } 
          0xca => {
              println!("SET 1 D");
              self.inc_pc_by(1);
          } 
          0xcb => {
              println!("SET 1 E");
              self.inc_pc_by(1);
          } 
          0xcc => {
              println!("SET 1 H");
              self.inc_pc_by(1);
          } 
          0xcd => {
              println!("SET 1 L");
              self.inc_pc_by(1);
          } 
          0xce => {
              println!("SET 1 (HL)");
              self.inc_pc_by(1);
          } 
          0xcf => {
              println!("SET 1 A");
              self.inc_pc_by(1);
          } 
          0xd0 => {
              println!("SET 2 B");
              self.inc_pc_by(1);
          } 
          0xd1 => {
              println!("SET 2 C");
              self.inc_pc_by(1);
          } 
          0xd2 => {
              println!("SET 2 D");
              self.inc_pc_by(1);
          } 
          0xd3 => {
              println!("SET 2 E");
              self.inc_pc_by(1);
          } 
          0xd4 => {
              println!("SET 2 H");
              self.inc_pc_by(1);
          } 
          0xd5 => {
              println!("SET 2 L");
              self.inc_pc_by(1);
          } 
          0xd6 => {
              println!("SET 2 (HL)");
              self.inc_pc_by(1);
          } 
          0xd7 => {
              println!("SET 2 A");
              self.inc_pc_by(1);
          } 
          0xd8 => {
              println!("SET 3 B");
              self.inc_pc_by(1);
          } 
          0xd9 => {
              println!("SET 3 C");
              self.inc_pc_by(1);
          } 
          0xda => {
              println!("SET 3 D");
              self.inc_pc_by(1);
          } 
          0xdb => {
              println!("SET 3 E");
              self.inc_pc_by(1);
          } 
          0xdc => {
              println!("SET 3 H");
              self.inc_pc_by(1);
          } 
          0xdd => {
              println!("SET 3 L");
              self.inc_pc_by(1);
          } 
          0xde => {
              println!("SET 3 (HL)");
              self.inc_pc_by(1);
          } 
          0xdf => {
              println!("SET 3 A");
              self.inc_pc_by(1);
          } 
          0xe0 => {
              println!("SET 4 B");
              self.inc_pc_by(1);
          } 
          0xe1 => {
              println!("SET 4 C");
              self.inc_pc_by(1);
          } 
          0xe2 => {
              println!("SET 4 D");
              self.inc_pc_by(1);
          } 
          0xe3 => {
              println!("SET 4 E");
              self.inc_pc_by(1);
          } 
          0xe4 => {
              println!("SET 4 H");
              self.inc_pc_by(1);
          } 
          0xe5 => {
              println!("SET 4 L");
              self.inc_pc_by(1);
          } 
          0xe6 => {
              println!("SET 4 (HL)");
              self.inc_pc_by(1);
          } 
          0xe7 => {
              println!("SET 4 A");
              self.inc_pc_by(1);
          } 
          0xe8 => {
              println!("SET 5 B");
              self.inc_pc_by(1);
          } 
          0xe9 => {
              println!("SET 5 C");
              self.inc_pc_by(1);
          } 
          0xea => {
              println!("SET 5 D");
              self.inc_pc_by(1);
          } 
          0xeb => {
              println!("SET 5 E");
              self.inc_pc_by(1);
          } 
          0xec => {
              println!("SET 5 H");
              self.inc_pc_by(1);
          } 
          0xed => {
              println!("SET 5 L");
              self.inc_pc_by(1);
          } 
          0xee => {
              println!("SET 5 (HL)");
              self.inc_pc_by(1);
          } 
          0xef => {
              println!("SET 5 A");
              self.inc_pc_by(1);
          } 
          0xf0 => {
              println!("SET 6 B");
              self.inc_pc_by(1);
          } 
          0xf1 => {
              println!("SET 6 C");
              self.inc_pc_by(1);
          } 
          0xf2 => {
              println!("SET 6 D");
              self.inc_pc_by(1);
          } 
          0xf3 => {
              println!("SET 6 E");
              self.inc_pc_by(1);
          } 
          0xf4 => {
              println!("SET 6 H");
              self.inc_pc_by(1);
          } 
          0xf5 => {
              println!("SET 6 L");
              self.inc_pc_by(1);
          } 
          0xf6 => {
              println!("SET 6 (HL)");
              self.inc_pc_by(1);
          } 
          0xf7 => {
              println!("SET 6 A");
              self.inc_pc_by(1);
          } 
          0xf8 => {
              println!("SET 7 B");
              self.inc_pc_by(1);
          } 
          0xf9 => {
              println!("SET 7 C");
              self.inc_pc_by(1);
          } 
          0xfa => {
              println!("SET 7 D");
              self.inc_pc_by(1);
          } 
          0xfb => {
              println!("SET 7 E");
              self.inc_pc_by(1);
          } 
          0xfc => {
              println!("SET 7 H");
              self.inc_pc_by(1);
          } 
          0xfd => {
              println!("SET 7 L");
              self.inc_pc_by(1);
          } 
          0xfe => {
              println!("SET 7 (HL)");
              self.inc_pc_by(1);
          } 
          0xff => {
              println!("SET 7 A");
              self.inc_pc_by(1);
          } 
          _ => {
            println!("INVALID OPCODE: {:x}", cb_opcode);
            self.inc_pc_by(1);
          }
      } 
  } 
  0xcc => { 
      println!("CALL Z a16");
      self.inc_pc_by(3); 
  } 
  0xcd => { 
      println!("CALL a16 ");
      self.inc_pc_by(3); 
  } 
  0xce => { 
      println!("ADC A d8");
      self.inc_pc_by(2); 
  } 
  0xcf => { 
      println!("RST 08H ");
      self.inc_pc_by(1); 
  } 
  0xd0 => { 
      println!("RET NC ");
      self.inc_pc_by(1); 
  } 
  0xd1 => { 
      println!("POP DE ");
      self.inc_pc_by(1); 
  } 
  0xd2 => { 
      println!("JP NC a16");
      self.inc_pc_by(3); 
  } 
  0xd4 => { 
      println!("CALL NC a16");
      self.inc_pc_by(3); 
  } 
  0xd5 => { 
      println!("PUSH DE ");
      self.inc_pc_by(1); 
  } 
  0xd6 => { 
      println!("SUB d8 ");
      self.inc_pc_by(2); 
  } 
  0xd7 => { 
      println!("RST 10H ");
      self.inc_pc_by(1); 
  } 
  0xd8 => { 
      println!("RET C ");
      self.inc_pc_by(1); 
  } 
  0xd9 => { 
      println!("RETI  ");
      self.inc_pc_by(1); 
  } 
  0xda => { 
      println!("JP C a16");
      self.inc_pc_by(3); 
  } 
  0xdc => { 
      println!("CALL C a16");
      self.inc_pc_by(3); 
  } 
  0xde => { 
      println!("SBC A d8");
      self.inc_pc_by(2); 
  } 
  0xdf => { 
      println!("RST 18H ");
      self.inc_pc_by(1); 
  } 
  0xe0 => { 
      println!("LDH (a8) A");
      self.inc_pc_by(2); 
  } 
  0xe1 => { 
      println!("POP HL ");
      self.inc_pc_by(1); 
  } 
  0xe2 => { 
      println!("LD (C) A");
      self.inc_pc_by(1); 
  } 
  0xe5 => { 
      println!("PUSH HL ");
      self.inc_pc_by(1); 
  } 
  0xe6 => { 
      println!("AND d8 ");
      self.inc_pc_by(2); 
  } 
  0xe7 => { 
      println!("RST 20H ");
      self.inc_pc_by(1); 
  } 
  0xe8 => { 
      println!("ADD SP r8");
      self.inc_pc_by(2); 
  } 
  0xe9 => { 
      println!("JP (HL) ");
      self.inc_pc_by(1); 
  } 
  0xea => { 
      println!("LD (a16) A");
      self.inc_pc_by(3); 
  } 
  0xee => { 
      println!("XOR d8 ");
      self.inc_pc_by(2); 
  } 
  0xef => { 
      println!("RST 28H ");
      self.inc_pc_by(1); 
  } 
  0xf0 => { 
      println!("LDH A (a8)");
      self.inc_pc_by(2); 
  } 
  0xf1 => { 
      println!("POP AF ");
      self.inc_pc_by(1); 
  } 
  0xf2 => { 
      println!("LD A (C)");
      self.inc_pc_by(1); 
  } 
  0xf3 => { 
      println!("DI  ");
      self.inc_pc_by(1); 
  } 
  0xf5 => { 
      println!("PUSH AF ");
      self.inc_pc_by(1); 
  } 
  0xf6 => { 
      println!("OR d8 ");
      self.inc_pc_by(2); 
  } 
  0xf7 => { 
      println!("RST 30H ");
      self.inc_pc_by(1); 
  } 
  0xf8 => { 
      println!("LD HL SP+r8");
      self.inc_pc_by(2); 
  } 
  0xf9 => { 
      println!("LD SP HL");
      self.inc_pc_by(1); 
  } 
  0xfa => { 
      println!("LD A (a16)");
      self.inc_pc_by(3); 
  } 
  0xfb => { 
      println!("EI  ");
      self.inc_pc_by(1); 
  } 
  0xfe => { 
      println!("CP d8 ");
      self.inc_pc_by(2); 
  } 
  0xff => { 
      println!("RST 38H ");
      self.inc_pc_by(1); 
  } 
  _ => {
    println!("INVALID OPCODE: {:x}", opcode);
    self.inc_pc_by(1);
  }
}
