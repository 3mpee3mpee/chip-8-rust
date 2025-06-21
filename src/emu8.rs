use crate::{audio::Audio, cpu::get_cpu, display::{get_world, init_display}, instructions::{add_byte_to_reg, clear_screen, jump_inst, ld_mem_to_i, ld_to_reg}, memory::{get_font_arr, get_mem, init_fonts, read_memory, write_memory}};
use std::{sync::{Arc, Mutex}, thread};

pub fn init(contents: &mut Vec<u8>) {
    let mut mem_arr = get_mem();
    let mut fonts_arr = get_font_arr();
    init_fonts(&mut mem_arr, &mut fonts_arr);
    let mut cpu = get_cpu();
    let mut audio = Audio::new().expect("Audio cannot be summoned");
    let mem_idx = 0x200;

    for (i, &byte) in contents.iter().enumerate() {
        write_memory(&mut mem_arr, mem_idx + i as u16, byte);
    }

    let mut world = get_world();

    let _ = init_display(&mut world, &mut mem_arr, &mut cpu, &mut audio).expect("Init display function failed");

}
