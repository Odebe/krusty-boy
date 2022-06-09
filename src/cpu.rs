// // pub struct Emulator {
// //     mmu: MMU,
// //     pc: u16,
// //     registers: Registers,
// //
// //     z_flag: bool,
// //     b_flag: bool,
// //     h_flag: bool,
// //     c_flag: bool,
// // }
// use std::cell::RefCell;
// use std::rc::Rc;
//
// pub struct Cpu {
//     mmu: Ack<ReffCell<Mmu>>,
// }
//
// pub struct Flags {
//     z: u8,
//     b: u8,
//     h: u8,
//     c: u8
// }
//
// pub struct Mmu {
//     memory: [u8; 0xFFFF],
// }
//
// pub struct MmuMemoryIterator {
//     position: u16,
//     memory: [u8; 0xFFFF],
// }
//
// pub struct Ppu {
// }
//
// pub struct Apu {
// }
//
// impl FLags {
//     pub fn new() -> Self {
//         Self { 0, 0, 0, 0}
//     }
// }
//
// impl Mmu {
//     pub fn new() -> Self {
//         Self {
//             memory: [0; 0xFFFF]
//         }
//     }
//
//     pub fn iter(&self) -> MmuMemoryIterator {
//         MmuMemoryIterator { position: 0, memory: self.memory }
//     }
//
//     pub fn load_rom(&mut self, rom: &Vec<u8>) {
//         for (i, e) in rom.iter().enumerate() {
//             if ustart + i < Self::MEMORY_SIZE as usize {
//                 self.memory[ustart + i] = *e;
//             }
//         }
//     }
// }
//
// impl Cpu {
//     pub fn new(mmu : &Mmu) -> Self { Self { mmu } }
// }
//
// impl Ppu {
//     pub fn new() -> Self { Self {} }
// }
//
// impl Apu {
//     pub fn new() -> Self { Self {} }
// }
//
// pub struct Emulator {
//     pc: u16,
//     cpu: Cpu,
//     mmu: Rc<RefCell<Mmu>>,
//     ppu: Ppu,
//     apu: Apu,
//     registers: Registers,
//     flags: Flags,
// }
//
// impl Emulator {
//     const PROG_START: u16 = 0;
//     const MEMORY_SIZE: u16 = 0xFFFF;
//
//     pub fn new() -> Self {
//         let mmu = Rc::new(RefCell::new(Mmu::new()));
//
//         Self {
//             pc: Self::PROG_START,
//             cpu: Cpu::new(mmu.copy),
//             mmu: mmu.copy,
//             apu: Apu::New(),
//             ppu: Apu::New(),
//             registers: Registers::new(),
//             flags: Flags::new(),
//         }
//     }
//
//     pub fn load_rom(&mut self, rom: &Vec<u8>) {
//         mmu.load_rom(rom);
//     }
//
//     pub fn running(&self) -> bool {
//         self.pc < Self::MEMORY_SIZE
//     }
//
//     fn memory_read(&self, dest: u16) -> u8 {
//         self.memory[dest as usize]
//     }
//
//     pub fn current_opcode(&self) -> u8 {
//         self.memory_read(self.pc)
//     }
//
//     fn read_u8(&self, from: u16) -> u8 {
//         self.memory_read(from)
//     }
//
//     fn read_u16(&self, from: u16) -> u16 {
//         let value_fn = self.memory_read(from);
//         let value_sn = self.memory_read(from + 1);
//         ((value_sn as u16 ) << 8) | value_fn as u16
//     }
//
//     pub fn inc_pc_by(&mut self, bytes: u8) {
//         self.pc = self.pc + bytes as u16;
//     }
// }
