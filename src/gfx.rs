use std::ops::{BitAnd, Not};
use minifb::{Key, KeyRepeat, Window, WindowOptions};

const WHITE: u32 = from_u8_rgb(255, 255, 255);
const BLACK: u32 = from_u8_rgb(0, 0, 0);

pub struct Gfx {
    window: minifb::Window,
    width: usize,
    height: usize,
    buffer: Vec<u32>,
}

impl Gfx {
    pub fn new(width: usize, height: usize) -> Self {
        let mut window = Window::new(
            "Chip8 - ESC to exit",
            width,
            height,
            WindowOptions::default(),
        )
            .unwrap_or_else(|e| {
                panic!("{}", e);
            });
        // Limit to max ~60 fps update rate
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        Gfx {
            window,
            width,
            height,
            buffer: vec![0; width * height],
        }
    }

    pub fn update_screen(&mut self, screen: &Vec<bool>) {

        for (index, i) in screen.iter().enumerate() {
            match i {
                true => self.buffer[index] = WHITE,
                false => self.buffer[index] = BLACK,
            }
        }
        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap_or_else(|e| println!("Couldn't update window: {}", e))
    }

    pub fn handle_state(&mut self) -> bool{
        if self.window.is_open().not() {
            return true
        } else if  self.window.is_key_down(Key::Escape) {
            return true
        }
        false
    }

    pub fn handle_key_event(&mut self, key_pressed: &mut Vec<bool>) {
        /*    Keypad                   Keyboard
              +-+-+-+-+                +-+-+-+-+
              |1|2|3|C|                |1|2|3|4|
              +-+-+-+-+                +-+-+-+-+
              |4|5|6|D|                |Q|W|E|R|
              +-+-+-+-+       =>       +-+-+-+-+
              |7|8|9|E|                |A|S|D|F|
              +-+-+-+-+                +-+-+-+-+
              |A|0|B|F|                |Z|X|C|V|
              +-+-+-+-+                +-+-+-+-+*/
        for key in self.window.get_keys_pressed(KeyRepeat::Yes) {
            match key {
                Key::Key1 => key_pressed[0x1] = true,
                Key::Key2 => key_pressed[0x2] = true,
                Key::Key3 => key_pressed[0x3] = true,
                Key::Key4 => key_pressed[0xC] = true,
                Key::Q => key_pressed[0x4] = true,
                Key::W => key_pressed[0x5] = true,
                Key::E => key_pressed[0x6] = true,
                Key::R => key_pressed[0xD] = true,
                Key::A => key_pressed[0x7] = true,
                Key::S => key_pressed[0x8] = true,
                Key::D => key_pressed[0x9] = true,
                Key::F => key_pressed[0xE] = true,
                Key::Z => key_pressed[0xA] = true,
                Key::X => key_pressed[0x0] = true,
                Key::C => key_pressed[0xB] = true,
                Key::V => key_pressed[0xF] = true,
                _ => {}
            }
        };

        for key in self.window.get_keys_released() {
            match key {
                Key::Key1 => key_pressed[0x1] = false,
                Key::Key2 => key_pressed[0x2] = false,
                Key::Key3 => key_pressed[0x3] = false,
                Key::Key4 => key_pressed[0xC] = false,
                Key::Q => key_pressed[0x4] = false,
                Key::W => key_pressed[0x5] = false,
                Key::E => key_pressed[0x6] = false,
                Key::R => key_pressed[0xD] = false,
                Key::A => key_pressed[0x7] = false,
                Key::S => key_pressed[0x8] = false,
                Key::D => key_pressed[0x9] = false,
                Key::F => key_pressed[0xE] = false,
                Key::Z => key_pressed[0xA] = false,
                Key::X => key_pressed[0x0] = false,
                Key::C => key_pressed[0xB] = false,
                Key::V => key_pressed[0xF] = false,
                _ => {}
            }
        }
    }

}

const fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}