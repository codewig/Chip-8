extern crate minifb;

use minifb::{Key, KeyRepeat, Window, WindowOptions};
use crate::chip8::Chip8;
use std::str;

mod chip8;
mod memory;
mod gfx;

const PONG: &str = "chip8-roms/games/Pong [Paul Vervalin, 1990].ch8";
const WIDTH: usize = 64;
const HEIGHT: usize = 32;




fn main() {
    let mut gfx = gfx::Gfx::new(64, 32);
    let mut chip = chip8::Chip8::new();
    chip.load_game(PONG).expect("Failed to load game");

    loop {
        if gfx.handle_state() { break; } // window closed or esc
        chip.emulate_cycle();
        gfx.handle_key_event(&mut chip.key_pressed);

        if let Some(screen) = chip.draw() {
            gfx.update_screen(screen);
        }

    }

}
