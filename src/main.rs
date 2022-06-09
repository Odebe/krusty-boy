use std::fs::{File, read};
use std::io::Read;

use clap::{App, Arg};
use crate::gameboy::Emulator;

mod gameboy;
mod meta;
mod cpu;

fn init_app() {
    App::new("WIP: Rusty GameBoy emulator")
        .version("0.1.0")
        .author("Mihail Odebe <derpiranha@gmail.com>")
        .about("Emulates GameBoy")
        .arg(Arg::new("rom")
            .short('r')
            .long("rom")
            .value_name("ROM_PATH")
            .about("path to rom file")
            .takes_value(true))
        .get_matches();
}

fn main() {
    let app = init_app();

    if let Some(rom_path) = app.value_of("rom") {
        let rom = read_rom(&rom_path);
        let mut mmu = gameboy::MMU::from_rom(&rom);
        let mut cpu = gameboy::Emulator::new(mmu);

        main_loop(cpu);
    } else {
        println!("ROM file not specified. Try run with --help flag");
    }
}

fn read_rom(path : &str) -> Vec<u8> {
    let mut file = File::open(&path).expect("no rom file found");
    let mut rom_buffer = Vec::new();

    file.read_to_end(&mut rom_buffer).expect("buffer overflow");

    return rom_buffer;
}

fn main_loop(mut cpu : Emulator) {
    while cpu.running() {
        let (_length, cycles) = meta::exec_opcode(&mut cpu);
        println!("Cycles: {}", cycles);
    }
}
