use rand::Rng;

use crate::{cpu::{get_reg_val, set_reg_val, CPU}, display::{self, World}, memory::{read_memory, write_memory}};

pub fn jump_inst(cpu: &mut CPU, opcode: u16) {
    println!("Executing Jump instruction");
    println!("Current opcode is {:04x}", opcode);
    println!("Jump addr is {:04x}", opcode & 0xFFF);
    cpu.pc = opcode & 0xFFF;
}

pub fn ret(cpu: &mut CPU, opcode: u16) {
    println!("Executing RET instruction");
    cpu.pc = cpu.sp.pop().expect("Should return stack pointer value");
    cpu.pc += 2;
}

pub fn jump_to_loc(cpu: &mut CPU, opcode: u16) {
    println!("Executing Jump to location instruction");
    println!("Current opcode is {:04x}", opcode);
    println!("Jump addr is {:04x}", opcode & 0xFFF);
    cpu.pc = (opcode & 0xFFF) + cpu.regs.v0.value as u16;
}

pub fn rand_op(cpu: &mut CPU, opcode: u16) {
    println!("Executing RND instruction");
    println!("Current opcode is {:04x}", opcode);
    let reg_byte = (0x0F00 & opcode) >> 8;
    let reg_str = "v".to_owned() + &reg_byte.to_string();
    println!("Our reg to load is: {reg_str}");
    let low_byte = 0xFF & opcode;
    println!("Our low byte (value to write) is: {:02x}", low_byte);
    let mut rng = rand::rng();
    let val: u8 = rng.random_range(0..=255);
    println!("Our random value to write is: {:02x}", val);
    set_reg_val(cpu, val, reg_byte as u8);
    cpu.pc += 2;
}

pub fn skp_op(cpu: &mut CPU, opcode: u16, keyboard: &mut std::collections::HashMap<u8, u8>) {
    println!("Executing SKP instruction");
    println!("Current opcode is {:04x}", opcode);
    let reg_byte = (0x0F00 & opcode) >> 8;
    let reg_str = "v".to_owned() + &reg_byte.to_string();
    println!("Our reg to load is: {reg_str}");
    let reg_x_val = get_reg_val(cpu, reg_byte as u8);
    if keyboard[&reg_x_val] > 0 {
        cpu.pc += 2;
    }
    cpu.pc += 2;
}

pub fn sknp_op(cpu: &mut CPU, opcode: u16, keyboard: &mut std::collections::HashMap<u8, u8>) {
    println!("Executing SKNP instruction");
    println!("Current opcode is {:04x}", opcode);
    let reg_byte = (0x0F00 & opcode) >> 8;
    let reg_str = "v".to_owned() + &reg_byte.to_string();
    println!("Our reg to load is: {reg_str}");
    let reg_x_val = get_reg_val(cpu, reg_byte as u8);
    if keyboard[&reg_x_val] == 0 {
        cpu.pc += 2;
    }
    cpu.pc += 2;
}

pub fn ld_to_reg(cpu: &mut CPU, opcode: u16) {
    println!("Executing Load instruction");
    println!("Current opcode is {:04x}", opcode);

    let reg_byte = (0x0F00 & opcode) >> 8;
    let reg_str = "v".to_owned() + &reg_byte.to_string();
    println!("Our reg to load is: {reg_str}");
    let low_byte = 0xFF & opcode;
    println!("Our low byte (value to write) is: {:02x}", low_byte);
    set_reg_val(cpu, low_byte as u8, reg_byte as u8);
    cpu.pc += 2;
}

pub fn ld_dt(cpu: &mut CPU, opcode: u16) {
    println!("Executing Load DT instruction");
    println!("Current opcode is {:04x}", opcode);

    let reg_byte = (0x0F00 & opcode) >> 8;
    let reg_str = "v".to_owned() + &reg_byte.to_string();
    println!("Our reg to load is: {reg_str}");
    println!("Our DT value is: {:02x}", cpu.dt);
    set_reg_val(cpu, cpu.dt as u8, reg_byte as u8);
    cpu.pc += 2;
}

pub fn dt_ld(cpu: &mut CPU, opcode: u16) {
    println!("Executing DT LD instruction");
    println!("Current opcode is {:04x}", opcode);

    println!("Our DT value is: {:02x}", cpu.dt);
    let reg_byte = (0x0F00 & opcode) >> 8;
    let reg_str = "v".to_owned() + &reg_byte.to_string();
    println!("Our reg to load is: {reg_str}");
    let reg_x_val = get_reg_val(cpu, reg_byte as u8);
    println!("Our reg value to load in dt is: {reg_x_val}");
    
    cpu.dt = reg_x_val;
    cpu.pc += 2;
}

pub fn st_ld(cpu: &mut CPU, opcode: u16) {
    println!("Executing ST LD instruction");
    println!("Current opcode is {:04x}", opcode);

    println!("Our ST value is: {:02x}", cpu.st);
    let reg_byte = (0x0F00 & opcode) >> 8;
    let reg_str = "v".to_owned() + &reg_byte.to_string();
    println!("Our reg to load is: {reg_str}");
    let reg_x_val = get_reg_val(cpu, reg_byte as u8);
    println!("Our reg value to load in dt is: {reg_x_val}");
    
    cpu.st = reg_x_val;
    cpu.pc += 2;
}

pub fn ld_reg_to_reg(cpu: &mut CPU, opcode: u16) {
    println!("Executing Load instruction");
    println!("Current opcode is {:04x}", opcode);
    let y_reg = (0x00F0 & opcode) >> 4 as u8;
    println!("Our VY Reg: {:02x}", y_reg);
    let x_reg = (0x0F00 & opcode) >> 8;
    println!("Our VX Reg: {:02x}", x_reg);
    let y_reg_val = get_reg_val(cpu, y_reg as u8);
    println!("Our VY Reg Value: {:02x}", y_reg_val);
    set_reg_val(cpu, y_reg_val as u8, x_reg as u8);
    cpu.pc += 2;
}

pub fn ld_mem_to_i(cpu: &mut CPU, opcode: u16) {
    println!("Executing load memory to I REG instruction");
    println!("Current opcode is {:04x}", opcode);

    let val = opcode & 0xFFF;

    println!("Addr value to write in I Reg: {:04x}", val);

    cpu.regs.i.value = val;
    cpu.pc += 2;
}

pub fn add_byte_to_reg(cpu: &mut CPU, opcode: u16) {
    println!("Executing ADD byte to REG");
    println!("Current opcode is {:04x}", opcode);

    let reg_byte = (0x0F00 & opcode) >> 8;
    let reg_str = "v".to_owned() + &reg_byte.to_string();
    println!("Our reg to load is: {reg_str}");
    let low_byte = 0xFF & opcode;
    println!("Our low byte is: {:02x}", low_byte);
    let (val_to_write, _carry) = get_reg_val(cpu, reg_byte as u8).overflowing_add(low_byte as u8);
    println!("Our value to write is: {:02x}", val_to_write);
    set_reg_val(cpu, val_to_write as u8, reg_byte as u8);
    cpu.pc += 2;
}

pub fn add_to_i(cpu: &mut CPU, opcode: u16) {
    println!("Executing ADD to I");
    println!("Current opcode is {:04x}", opcode);

    let reg_byte = (0x0F00 & opcode) >> 8;
    let reg_str = "v".to_owned() + &reg_byte.to_string();
    println!("Our reg to load is: {reg_str}");
    let reg_x_val = get_reg_val(cpu, reg_byte as u8);
    println!("Our reg value is: {reg_x_val}");
    cpu.regs.i.value = unsafe { cpu.regs.i.value } + reg_x_val as u16;
    cpu.pc += 2;
}

pub fn ld_f(cpu: &mut CPU, opcode: u16) {
    println!("Executing LD F to I");
    println!("Current opcode is {:04x}", opcode);

    let reg_byte = (0x0F00 & opcode) >> 8;
    let reg_str = "v".to_owned() + &reg_byte.to_string();
    println!("Our reg to load is: {reg_str}");
    let reg_x_val = get_reg_val(cpu, reg_byte as u8);
    println!("Our reg value is: {reg_x_val}");
    cpu.regs.i.value = 0x50 + (reg_x_val * 5) as u16;
    cpu.pc += 2;
}

pub fn ld_b(mem_arr: &mut [u8; 4096], cpu: &mut CPU, opcode: u16) {
    println!("Executing LD B to I");
    println!("Current opcode is {:04x}", opcode);

    let reg_byte = (0x0F00 & opcode) >> 8;
    let reg_str = "v".to_owned() + &reg_byte.to_string();
    println!("Our reg to load is: {reg_str}");
    let reg_x_val = get_reg_val(cpu, reg_byte as u8);
    println!("Our reg value is: {reg_x_val}");
    let first_digit = reg_x_val / 100;
    let second_digit = (reg_x_val % 100) / 10;
    let third_digit = reg_x_val % 10;
    println!("First digit to write: {first_digit}");
    println!("Second digit to write: {second_digit}");
    println!("Third digit to write: {third_digit}");
    write_memory(mem_arr, unsafe { cpu.regs.i.value }, first_digit);
    write_memory(mem_arr, unsafe { cpu.regs.i.value } + 1, second_digit);
    write_memory(mem_arr, unsafe { cpu.regs.i.value } + 2, third_digit);
    cpu.pc += 2;
}

pub fn ld_i(mem_arr: &mut [u8; 4096], cpu: &mut CPU, opcode: u16) {
    println!("Executing LD I to I");
    println!("Current opcode is {:04x}", opcode);

    let reg_byte = (0x0F00 & opcode) >> 8;
    for i in 0..=reg_byte {
        let reg_str = "v".to_owned() + &i.to_string();
        println!("Our reg to load is: {reg_str}");
        let val = get_reg_val(cpu, i as u8);
        println!("Our value from reg is: {val}");
        let addr = unsafe { cpu.regs.i.value } + i as u16;
        println!("Our address to write in: {addr}");
        write_memory(mem_arr, addr, val);
    }
    cpu.pc += 2;
}

pub fn ld_v(mem_arr: &mut [u8; 4096], cpu: &mut CPU, opcode: u16) {
    println!("Executing LD V to I");
    println!("Current opcode is {:04x}", opcode);

    let reg_byte = (0x0F00 & opcode) >> 8;
    for i in 0..=reg_byte {
        let addr = unsafe { cpu.regs.i.value } + i as u16;
        println!("Our address to read from: {addr}");
        let val = read_memory(mem_arr, addr);
        let reg_str = "v".to_owned() + &i.to_string();
        println!("Our reg to write in is: {reg_str}");
        set_reg_val(cpu, val, i as u8);
    }
    cpu.pc += 2;
}

pub fn ld_vx_k(cpu: &mut CPU, opcode: u16, keyboard: &mut std::collections::HashMap<u8,u8>) {
    println!("Executing LD Vx K");
    println!("Current opcode is {:04x}", opcode);
    let reg_byte = (0x0F00 & opcode) >> 8;
    let reg_str = "v".to_owned() + &reg_byte.to_string();
    println!("Our reg to load is: {reg_str}");

    // Debug: print the current keyboard map
    println!("Current keyboard map: {:?}", keyboard);

    // Find if any key is pressed (value != 0)
    if let Some((&key, &val)) = keyboard.iter().find(|(_, &v)| v != 0) {
        println!("Key pressed: {key} (value: {val})");
        set_reg_val(cpu, key, reg_byte as u8);
        cpu.pc += 2; // Advance to next instruction
    } else {
        println!("No key pressed, waiting...");
        // Do not advance pc, so this instruction will be retried
    }
    cpu.pc += 2; // Advance to next instruction
}


pub fn clear_screen(world: &mut World, cpu: &mut CPU, opcode: u16) {
    println!("Executing Clear Screen");
    println!("Current opcode is {:04x}", opcode);

    world.px = [0; (display::CHIP8_WIDTH * display::CHIP8_HEIGHT) as usize];
    world.display_redraw = 1;
    cpu.pc += 2;
}

pub fn draw_sprite(cpu: &mut CPU, mem_arr: &[u8], world: &mut World, opcode: u16) {
    println!("Executing Draw Sprite");
    println!("Current opcode is {:04x}", opcode);
    let bytes_to_read = 0xf & (0xFF & opcode);
    println!("Our Bytes to read: {:02x}", bytes_to_read);
    let y_reg = (0x00F0 & opcode) >> 4 as u8;
    println!("Our VY Reg: {:02x}", y_reg);
    let x_reg = (0x0F00 & opcode) >> 8;
    println!("Our VX Reg: {:02x}", x_reg);
    let x_reg_val = get_reg_val(cpu, x_reg as u8);
    println!("Our VX Reg Value: {:02x}", x_reg_val);
    let y_reg_val = get_reg_val(cpu, y_reg as u8);
    println!("Our VY Reg Value: {:02x}", y_reg_val);
    let i_reg_val = unsafe { cpu.regs.i.value };
    let sprite_slice = &mem_arr[i_reg_val as usize..i_reg_val as usize + bytes_to_read as usize];
    cpu.regs.vf.value = 0;
    for (rows, byte) in sprite_slice.iter().enumerate() {
        for bit in 0..8 {
            let pixel = byte >> (7 - bit) & 1;
            let x = (x_reg_val as usize + bit) % display::CHIP8_WIDTH as usize;
            let y = (y_reg_val as usize + rows) % display::CHIP8_HEIGHT as usize;
            let idx = y * display::CHIP8_WIDTH as usize + x;

            let old_px = world.px[idx];
            world.px[idx] ^= pixel;
            
            if old_px == 1 && world.px[idx] == 0 {
                cpu.regs.vf.value = 1;
            }
        }
    }

    world.display_redraw = 1;
    cpu.pc += 2;
}

pub fn call_addr(cpu: &mut CPU, opcode: u16) {
    println!("Executing Call instruction");
    println!("Current opcode is {:04x}", opcode);
    println!("Call addr is {:04x}", opcode & 0xFFF);
    let addr = opcode & 0xFFF;
    cpu.sp.push(cpu.pc);
    cpu.pc = addr;
}

pub fn skip_next_eq(cpu: &mut CPU, opcode: u16) {
    println!("Executing Skip instruction");
    println!("Current opcode is {:04x}", opcode);
    let reg_byte = (0x0F00 & opcode) >> 8;
    let reg_str = "v".to_owned() + &reg_byte.to_string();
    println!("Our reg to compare is: {reg_str}");
    let val_to_compare = get_reg_val(cpu, reg_byte as u8);
    println!("Our value to compare is: {val_to_compare}");
    let low_byte = 0xFF & opcode as u8;
    println!("Our low byte (value to compare) is: {:02x}", low_byte);
    if val_to_compare == low_byte {
        cpu.pc += 2;
    }
    cpu.pc += 2;
}

pub fn skip_next_not_eq(cpu: &mut CPU, opcode: u16) {
    println!("Executing Skip instruction");
    println!("Current opcode is {:04x}", opcode);
    let reg_byte = (0x0F00 & opcode) >> 8;
    let reg_str = "v".to_owned() + &reg_byte.to_string();
    println!("Our reg to compare is: {reg_str}");
    let val_to_compare = get_reg_val(cpu, reg_byte as u8);
    println!("Our value to compare is: {val_to_compare}");
    let low_byte = 0xFF & opcode as u8;
    println!("Our low byte (value to compare) is: {:02x}", low_byte);
    if val_to_compare != low_byte {
        cpu.pc += 2;
    }
    cpu.pc += 2;
}

pub fn skip_next_eq_regs(cpu: &mut CPU, opcode: u16) {
    println!("Executing Skip instruction");
    println!("Current opcode is {:04x}", opcode);
    let y_reg = (0x00F0 & opcode) >> 4 as u8;
    println!("Our VY Reg: {:02x}", y_reg);
    let x_reg = (0x0F00 & opcode) >> 8;
    println!("Our VX Reg: {:02x}", x_reg);
    let x_reg_val = get_reg_val(cpu, x_reg as u8);
    println!("Our VX Reg Value: {:02x}", x_reg_val);
    let y_reg_val = get_reg_val(cpu, y_reg as u8);
    println!("Our VY Reg Value: {:02x}", y_reg_val);
    if x_reg_val == y_reg_val {
        cpu.pc += 2;
    }
    cpu.pc += 2;
}

pub fn or_op(cpu: &mut CPU, opcode: u16) {
    println!("Executing OR instruction");
    println!("Current opcode is {:04x}", opcode);
    let y_reg = (0x00F0 & opcode) >> 4 as u8;
    println!("Our VY Reg: {:02x}", y_reg);
    let x_reg = (0x0F00 & opcode) >> 8;
    println!("Our VX Reg: {:02x}", x_reg);
    let x_reg_val = get_reg_val(cpu, x_reg as u8);
    println!("Our VX Reg Value: {:02x}", x_reg_val);
    let y_reg_val = get_reg_val(cpu, y_reg as u8);
    println!("Our VY Reg Value: {:02x}", y_reg_val);
    let val = x_reg_val | y_reg_val;
    println!("Our Value to write is: {:02x}", val);
    set_reg_val(cpu, val as u8, x_reg as u8);
    cpu.pc += 2;
}

pub fn and_op(cpu: &mut CPU, opcode: u16) {
    println!("Executing AND instruction");
    println!("Current opcode is {:04x}", opcode);
    let y_reg = (0x00F0 & opcode) >> 4 as u8;
    println!("Our VY Reg: {:02x}", y_reg);
    let x_reg = (0x0F00 & opcode) >> 8;
    println!("Our VX Reg: {:02x}", x_reg);
    let x_reg_val = get_reg_val(cpu, x_reg as u8);
    println!("Our VX Reg Value: {:02x}", x_reg_val);
    let y_reg_val = get_reg_val(cpu, y_reg as u8);
    println!("Our VY Reg Value: {:02x}", y_reg_val);
    let val = x_reg_val & y_reg_val;
    println!("Our Value to write is: {:02x}", val);
    set_reg_val(cpu, val as u8, x_reg as u8);
    cpu.pc += 2;
}

pub fn xor_op(cpu: &mut CPU, opcode: u16) {
    println!("Executing XOR instruction");
    println!("Current opcode is {:04x}", opcode);
    let y_reg = (0x00F0 & opcode) >> 4 as u8;
    println!("Our VY Reg: {:02x}", y_reg);
    let x_reg = (0x0F00 & opcode) >> 8;
    println!("Our VX Reg: {:02x}", x_reg);
    let x_reg_val = get_reg_val(cpu, x_reg as u8);
    println!("Our VX Reg Value: {:02x}", x_reg_val);
    let y_reg_val = get_reg_val(cpu, y_reg as u8);
    println!("Our VY Reg Value: {:02x}", y_reg_val);
    let val = x_reg_val ^ y_reg_val;
    println!("Our Value to write is: {:02x}", val);
    set_reg_val(cpu, val as u8, x_reg as u8);
    cpu.pc += 2;
}

pub fn add_op(cpu: &mut CPU, opcode: u16) {
    println!("Executing ADD Logical instruction");
    println!("Current opcode is {:04x}", opcode);
    let y_reg = (0x00F0 & opcode) >> 4 as u8;
    println!("Our VY Reg: {:02x}", y_reg);
    let x_reg = (0x0F00 & opcode) >> 8;
    println!("Our VX Reg: {:02x}", x_reg);
    let x_reg_val = get_reg_val(cpu, x_reg as u8);
    println!("Our VX Reg Value: {:02x}", x_reg_val);
    let y_reg_val = get_reg_val(cpu, y_reg as u8);
    println!("Our VY Reg Value: {:02x}", y_reg_val);
    let (result, carry) = x_reg_val.overflowing_add(y_reg_val);
    cpu.regs.vf.value = if carry { 1 } else { 0 };
    println!("Our Value to write is: {:02x}", result);
    set_reg_val(cpu, result, x_reg as u8);
    cpu.pc += 2;
} 

pub fn sub_op(cpu: &mut CPU, opcode: u16) {
    println!("Executing SUB Logical instruction");
    println!("Current opcode is {:04x}", opcode);
    let y_reg = (0x00F0 & opcode) >> 4 as u8;
    println!("Our VY Reg: {:02x}", y_reg);
    let x_reg = (0x0F00 & opcode) >> 8;
    println!("Our VX Reg: {:02x}", x_reg);
    let x_reg_val = get_reg_val(cpu, x_reg as u8);
    println!("Our VX Reg Value: {:02x}", x_reg_val);
    let y_reg_val = get_reg_val(cpu, y_reg as u8);
    println!("Our VY Reg Value: {:02x}", y_reg_val);
    let (result, carry) = x_reg_val.overflowing_sub(y_reg_val);
    cpu.regs.vf.value = if carry { 1 } else { 0 };
    println!("Our Value to write is: {:02x}", result);
    set_reg_val(cpu, result as u8, x_reg as u8);
    cpu.pc += 2;
} 

pub fn subn_op(cpu: &mut CPU, opcode: u16) {
    println!("Executing SUBN Logical instruction");
    println!("Current opcode is {:04x}", opcode);
    let y_reg = (0x00F0 & opcode) >> 4 as u8;
    println!("Our VY Reg: {:02x}", y_reg);
    let x_reg = (0x0F00 & opcode) >> 8;
    println!("Our VX Reg: {:02x}", x_reg);
    let x_reg_val = get_reg_val(cpu, x_reg as u8);
    println!("Our VX Reg Value: {:02x}", x_reg_val);
    let y_reg_val = get_reg_val(cpu, y_reg as u8);
    println!("Our VY Reg Value: {:02x}", y_reg_val);
    if y_reg_val > x_reg_val {
        cpu.regs.vf.value = 1;
    } else {
        cpu.regs.vf.value = 0;
    }
    let val = y_reg_val - x_reg_val;
    println!("Our Value to write is: {:02x}", val);
    set_reg_val(cpu, val as u8, x_reg as u8);
    cpu.pc += 2;
} 

pub fn shr_op(cpu: &mut CPU, opcode: u16) {
    println!("Executing SHR Logical instruction");
    println!("Current opcode is {:04x}", opcode);
    let y_reg = (0x00F0 & opcode) >> 4 as u8;
    println!("Our VY Reg: {:02x}", y_reg);
    let x_reg = (0x0F00 & opcode) >> 8;
    println!("Our VX Reg: {:02x}", x_reg);
    let x_reg_val = get_reg_val(cpu, x_reg as u8);
    println!("Our VX Reg Value: {:02x}", x_reg_val);
    let y_reg_val = get_reg_val(cpu, y_reg as u8);
    println!("Our VY Reg Value: {:02x}", y_reg_val);
    cpu.regs.vf.value = x_reg_val & 0x1;
    let val = x_reg_val >> 1;

    println!("Our Value to write is: {:02x}", val);
    set_reg_val(cpu, val as u8, x_reg as u8);
    cpu.pc += 2;
} 

pub fn shl_op(cpu: &mut CPU, opcode: u16) {
    println!("Executing SHL Logical instruction");
    println!("Current opcode is {:04x}", opcode);
    let y_reg = (0x00F0 & opcode) >> 4 as u8;
    println!("Our VY Reg: {:02x}", y_reg);
    let x_reg = (0x0F00 & opcode) >> 8;
    println!("Our VX Reg: {:02x}", x_reg);
    let x_reg_val = get_reg_val(cpu, x_reg as u8);
    println!("Our VX Reg Value: {:02x}", x_reg_val);
    let y_reg_val = get_reg_val(cpu, y_reg as u8);
    println!("Our VY Reg Value: {:02x}", y_reg_val);
    cpu.regs.vf.value = (x_reg_val & 0x80) >> 7;
    let val = x_reg_val << 1;
    println!("Our Value to write is: {:02x}", val);
    set_reg_val(cpu, val as u8, x_reg as u8);
    cpu.pc += 2;
} 

pub fn sne_op(cpu: &mut CPU, opcode: u16) {
    println!("Executing SNE Logical instruction");
    println!("Current opcode is {:04x}", opcode);
    let y_reg = (0x00F0 & opcode) >> 4 as u8;
    println!("Our VY Reg: {:02x}", y_reg);
    let x_reg = (0x0F00 & opcode) >> 8;
    println!("Our VX Reg: {:02x}", x_reg);
    let x_reg_val = get_reg_val(cpu, x_reg as u8);
    println!("Our VX Reg Value: {:02x}", x_reg_val);
    let y_reg_val = get_reg_val(cpu, y_reg as u8);
    println!("Our VY Reg Value: {:02x}", y_reg_val);
    if x_reg_val != y_reg_val {
        cpu.pc += 2;
    } 
    cpu.pc += 2;
} 
