#![deny(clippy::all)]
#![forbid(unsafe_code)]

use std::time::{Instant, Duration};
use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use crate::audio::Audio;
use crate::cpu::CPU;
use crate::instructions::{add_byte_to_reg, add_op, add_to_i, and_op, call_addr, clear_screen, draw_sprite, dt_ld, jump_inst, jump_to_loc, ld_b, ld_dt, ld_f, ld_i, ld_mem_to_i, ld_reg_to_reg, ld_to_reg, ld_v, ld_vx_k, or_op, rand_op, ret, shl_op, shr_op, skip_next_eq, skip_next_eq_regs, skip_next_not_eq, sknp_op, skp_op, sne_op, st_ld, sub_op, subn_op, xor_op};
use crate::keyslog::get_keyboard_map;
use crate::memory::read_memory;

pub const CHIP8_WIDTH: u32 = 64;
pub const CHIP8_HEIGHT: u32 = 32;
pub const SCALE: u32 = 10;

/// Representation of the application state. In this example, a box will bounce around the screen.
pub struct World {
    pub px: [u8; (CHIP8_WIDTH * CHIP8_HEIGHT) as usize],
    pub display_redraw: u8
}

pub fn get_world() -> World {
    World::new()
}

pub fn init_display(world: &mut World, mem_arr: &mut [u8; 4096], cpu: &mut CPU, audio: &mut Audio) -> Result<(), Error> {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(
            (CHIP8_WIDTH * SCALE) as f64,
            (CHIP8_HEIGHT * SCALE) as f64,
        );
        WindowBuilder::new()
            .with_title("CHIP8 EMU")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(CHIP8_WIDTH * SCALE, CHIP8_HEIGHT * SCALE, &window);
        Pixels::new(CHIP8_WIDTH, CHIP8_HEIGHT, surface_texture)?
    };

    let mut keyboard = get_keyboard_map();

    let mut last_frame = Instant::now();
    let frame_duration = Duration::from_secs_f64(1.0 / 60.0);

    let res = event_loop.run(|event, elwt| {
        if last_frame.elapsed() > frame_duration {
            last_frame = Instant::now();
            let hi: u16 = (read_memory(&mem_arr, cpu.pc) as u16) << 8;
            let lo: u16 = read_memory(&mem_arr, cpu.pc + 1) as u16;
            let opcode: u16 = hi | lo;

            println!("Opcode val: {:#02x}", opcode);

            if cpu.dt > 0 {
                cpu.dt -= 1;
            }

            if cpu.st > 0 {
                audio.play();
                cpu.st -= 1;
            } else {
                audio.pause();
            }

            // Corrected opcode decoding for CHIP-8
            match opcode & 0xF000 {
                0x0000 => match opcode {
                    0x00E0 => clear_screen(world, cpu, opcode), // CLS
                    0x00EE => ret(cpu, opcode),            // RET
                    _ => println!("Unknown 0x0NNN opcode"),
                },
                0x1000 => jump_inst(cpu, opcode),           // JP addr
                0x2000 => call_addr(cpu, opcode),           // CALL addr
                0x3000 => skip_next_eq(cpu, opcode),        // SE Vx, byte
                0x4000 => skip_next_not_eq(cpu, opcode),    // SNE Vx, byte
                0x5000 => skip_next_eq_regs(cpu, opcode),   // SE Vx, Vy
                0x6000 => ld_to_reg(cpu, opcode),           // LD Vx, byte
                0x7000 => add_byte_to_reg(cpu, opcode),     // ADD Vx, byte
                0x8000 => match opcode & 0x000F {
                    0x0 => ld_reg_to_reg(cpu, opcode),      // LD Vx, Vy
                    0x1 => or_op(cpu, opcode),              // OR Vx, Vy
                    0x2 => and_op(cpu, opcode),             // AND Vx, Vy
                    0x3 => xor_op(cpu, opcode),             // XOR Vx, Vy
                    0x4 => add_op(cpu, opcode),             // ADD Vx, Vy
                    0x5 => sub_op(cpu, opcode),             // SUB Vx, Vy
                    0x6 => shr_op(cpu, opcode),             // SHR Vx {, Vy}
                    0x7 => subn_op(cpu, opcode),            // SUBN Vx, Vy
                    0xE => shl_op(cpu, opcode),             // SHL Vx {, Vy}
                    _ => println!("Unknown 0x8XY? opcode"),
                },
                0x9000 => sne_op(cpu, opcode),              // SNE Vx, Vy
                0xA000 => ld_mem_to_i(cpu, opcode),         // LD I, addr
                0xB000 => jump_to_loc(cpu, opcode),         // JP V0, addr
                0xC000 => rand_op(cpu, opcode),             // RND Vx, byte
                0xD000 => draw_sprite(cpu, mem_arr, world, opcode), // DRW Vx, Vy, nibble
                0xE000 => match opcode & 0x00FF {
                    0x9E => skp_op(cpu, opcode, &mut keyboard),  // SKP Vx
                    0xA1 => sknp_op(cpu, opcode, &mut keyboard), // SKNP Vx
                    _ => println!("Unknown 0xEX?? opcode"),
                },
                0xF000 => match opcode & 0x00FF {
                    0x07 => ld_dt(cpu, opcode),                  // LD Vx, DT
                    0x0A => ld_vx_k(cpu, opcode, &mut keyboard), // LD Vx, K
                    0x15 => dt_ld(cpu, opcode),                  // LD DT, Vx
                    0x18 => st_ld(cpu, opcode),                  // LD ST, Vx
                    0x1E => add_to_i(cpu, opcode),               // ADD I, Vx
                    0x29 => ld_f(cpu, opcode),                   // LD F, Vx
                    0x33 => ld_b(mem_arr, cpu, opcode),          // LD B, Vx
                    0x55 => ld_i(mem_arr, cpu, opcode),          // LD [I], Vx
                    0x65 => ld_v(mem_arr, cpu, opcode),          // LD Vx, [I]
                    _ => println!("Unknown 0xFX?? opcode"),
                },
                _ => println!("Unknown opcode: {:#04x}", opcode),
            }
        }



        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                elwt.exit();
                return;
            }

            if input.key_pressed(KeyCode::Digit1) {
                keyboard.insert(1, 1); 
            }

            if input.key_released(KeyCode::Digit1) {
                keyboard.insert(1, 0);
            }

            if input.key_pressed(KeyCode::Digit2) {
                keyboard.insert(2, 1) ;
            }

            if input.key_released(KeyCode::Digit2) {
                keyboard.insert(2, 0);
            }

            if input.key_pressed(KeyCode::Digit3) {
                keyboard.insert(3, 1);
            }

            if input.key_released(KeyCode::Digit3) {
                keyboard.insert(3, 0);
            }

            if input.key_pressed(KeyCode::Digit4) {
                keyboard.insert(0xC, 1); 
            }

            if input.key_released(KeyCode::Digit4) {
                keyboard.insert(0xC, 0);
            }

            if input.key_pressed(KeyCode::KeyQ) {
                keyboard.insert(4, 1);
            }

            if input.key_released(KeyCode::KeyQ) {
                keyboard.insert(4, 0);
            }

            if input.key_pressed(KeyCode::KeyW) {
                keyboard.insert(5, 1);
            }

            if input.key_released(KeyCode::KeyW) {
                keyboard.insert(5, 0);
            }

            if input.key_pressed(KeyCode::KeyE) {
                keyboard.insert(6, 1);
            }

            if input.key_released(KeyCode::KeyE) {
                keyboard.insert(6, 0);
            }

            if input.key_pressed(KeyCode::KeyR) {
                keyboard.insert(0xD, 1);
            }

            if input.key_released(KeyCode::KeyR) {
                keyboard.insert(0xD, 0);
            }

            if input.key_pressed(KeyCode::KeyA) {
                keyboard.insert(7, 1);
            }

            if input.key_released(KeyCode::KeyA) {
                keyboard.insert(7, 0);
            }

            if input.key_pressed(KeyCode::KeyS) {
                keyboard.insert(8, 1);
            }

            if input.key_released(KeyCode::KeyS) {
                keyboard.insert(8, 0);
            }

            if input.key_pressed(KeyCode::KeyD) {
                keyboard.insert(9, 1);
            }

            if input.key_released(KeyCode::KeyD) {
                keyboard.insert(9, 0);
            }

            if input.key_pressed(KeyCode::KeyF) {
                keyboard.insert(0xE, 1);
            }

            if input.key_released(KeyCode::KeyF) {
                keyboard.insert(0xE, 0);
            }

            if input.key_pressed(KeyCode::KeyZ) {
                keyboard.insert(0xA, 1);
            }

            if input.key_released(KeyCode::KeyZ) {
                keyboard.insert(0xA, 0);
            }

            if input.key_pressed(KeyCode::KeyX) {
                keyboard.insert(0, 1);
            }

            if input.key_released(KeyCode::KeyX) {
                keyboard.insert(0, 0);
            }

            if input.key_pressed(KeyCode::KeyC) {
                keyboard.insert(0xB, 1);
            }

            if input.key_released(KeyCode::KeyC) {
                keyboard.insert(0xB, 0);
            }

            if input.key_pressed(KeyCode::KeyV) {
                keyboard.insert(0xF, 1);
            }

            if input.key_released(KeyCode::KeyV) {
                keyboard.insert(0xF, 0);
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_error("pixels.resize_surface", err);
                    elwt.exit();
                    return;
                }
            }
        }

        // Draw the current frame
        if let Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } = event
        {
            world.draw(pixels.frame_mut());
            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                elwt.exit();
                return;
            }
        }
        
        // Update internal state and request a redraw
        window.request_redraw();
    });
    res.map_err(|e| Error::UserDefined(Box::new(e)))
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        Self {
            px: [0; (CHIP8_WIDTH * CHIP8_HEIGHT) as usize],
            display_redraw: 0,
        }
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&mut self, frame: &mut [u8]) {
        for (i, pixel) in self.px.iter().enumerate() {
            let mut rgba = [0xff, 0xff, 0xff, 0xff];
            if *pixel == 0 {
                rgba = [0x00, 0x00, 0x00, 0xff];
            } 

            let offset = i * 4;
            frame[offset..offset + 4].copy_from_slice(&rgba);
        }
        self.display_redraw = 0;
    }
}
