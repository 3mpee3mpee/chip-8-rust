use std::fs::{File};
use std::{env};
use std::io::Read;

use crate::keyslog::get_keyboard_map;
use crate::cpu::get_cpu;
use crate::display::{get_world, init_display};
use crate::emu8::init;
use crate::instructions::{add_byte_to_reg, clear_screen, jump_inst, ld_mem_to_i, ld_to_reg};
use crate::memory::{get_mem, read_memory, write_memory};
mod display;
mod emu8;
mod cpu;
mod memory;
mod audio;
mod instructions;
mod keyslog;

fn main() -> std::io::Result<()> {
    println!("Program started");

    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 || args[1].is_empty() {
        eprintln!("Error: No filepath argument provided or argument is empty.");
        panic!();
    }
    let filepath = &args[1];

    println!("Filepath: {filepath}");
    
    let mut file = File::open(filepath).expect("Failed to open a file");
    println!("File found");

    let mut contents = Vec::new();

    file.read_to_end(&mut contents).expect("Failed to read file to the end");

    init(&mut contents);

    Ok(())
}
