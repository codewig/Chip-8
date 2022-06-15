use rand::Rng;
use crate::memory;

use std::ops::Not;
use bitmatch::bitmatch;


pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

struct Sprite {
    x: usize,
    y: usize,
    rows: usize,
}

pub struct Chip8 {
    pub opcode: u16,
    pub memory: memory::Memory,
    pub v: Vec<u8>,
    pub i: u16,
    pub pc: usize,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub stack: Vec<usize>,
    pub sp: usize,
    pub key_pressed: Vec<bool>,
    pub gfx: Vec<bool>,
    pub draw_flag: bool,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            opcode: 0,
            memory: memory::Memory::new(),
            v: vec![0; 16],
            i: 0,
            pc: 0x200,
            delay_timer: 0,
            sound_timer: 0,
            stack: vec![0; 16],
            sp: 0,
            key_pressed: vec![false; 16],
            gfx: vec![false; WIDTH * HEIGHT],
            draw_flag: false,
        }
    }

    pub fn load_game(&mut self, file: &str) -> std::io::Result<()> {
        self.memory.load_game(file)
    }

    pub fn draw(&mut self) -> Option<&Vec<bool>> {
        let df = self.draw_flag;
        self.draw_flag = false;
        match df {
            true => Some(&self.gfx),
            false => None,
        }
    }

    pub fn emulate_cycle(&mut self) {
        self.handle_opcode(); // decode and execute opcode
        self.update_timers(); // update delay and sound timers
    }

    #[bitmatch]
    fn handle_opcode(&mut self) {
        let opcode = ((self.memory[self.pc] as u16) << 8) | (self.memory[self.pc + 1] as u16);
        self.opcode = opcode;

        println!("OPCODE: {:#4X}", opcode);

        #[bitmatch]
        match opcode {
            "0000_0000_1110_1110" => self.opcode_00ee(),
            "0000_0000_1110_0000" => self.opcode_00e0(),
            "0001_nnnn_nnnn_nnnn" => self.opcode_1nnn(n.into()),
            "0010_nnnn_nnnn_nnnn" => self.opcode_2nnn(n.into()),
            "0011_xxxx_nnnn_nnnn" => self.opcode_3xnn(x.into(), n as u8),
            "0100_xxxx_nnnn_nnnn" => self.opcode_4xnn(x.into(), n as u8),
            "0101_xxxx_yyyy_0000" => self.opcode_5xy0(x.into(), y.into()),
            "0110_xxxx_nnnn_nnnn" => self.opcode_6xnn(x.into(), n as u8),
            "0111_xxxx_nnnn_nnnn" => self.opcode_7xnn(x.into(), n as u8),
            "1000_xxxx_yyyy_0000" => self.opcode_8xy0(x.into(), y.into()),
            "1000_xxxx_yyyy_0001" => self.opcode_8xy1(x.into(), y.into()),
            "1000_xxxx_yyyy_0010" => self.opcode_8xy2(x.into(), y.into()),
            "1000_xxxx_yyyy_0011" => self.opcode_8xy3(x.into(), y.into()),
            "1000_xxxx_yyyy_0100" => self.opcode_8xy4(x.into(), y.into()),
            "1000_xxxx_yyyy_0101" => self.opcode_8xy5(x.into(), y.into()),
            "1000_xxxx_yyyy_0110" => self.opcode_8xy6(x.into(), y.into()),
            "1000_xxxx_yyyy_0111" => self.opcode_8xy7(x.into(), y.into()),
            "1000_xxxx_yyyy_1110" => self.opcode_8xye(x.into(), y.into()),
            "1001_xxxx_yyyy_0000" => self.opcode_9xy0(x.into(), y.into()),
            "1010_nnnn_nnnn_nnnn" => self.opcode_annn(n.into()),
            "1011_nnnn_nnnn_nnnn" => self.opcode_bnnn(n.into()),
            "1100_xxxx_nnnn_nnnn" => self.opcode_cxnn(x.into(), n as u8),
            "1101_xxxx_yyyy_nnnn" => self.opcode_dxyn(x.into(), y.into(), n.into()),
            "1110_xxxx_1001_1110" => self.opcode_ex9e(x.into()),
            "1110_xxxx_1010_0001" => self.opcode_exa1(x.into()),
            "1111_xxxx_0000_0111" => self.opcode_fx07(x.into()),
            "1111_xxxx_0000_1010" => self.opcode_fx0a(x.into()),
            "1111_xxxx_0001_0101" => self.opcode_fx15(x.into()),
            "1111_xxxx_0001_1000" => self.opcode_fx18(x.into()),
            "1111_xxxx_0001_1110" => self.opcode_fx1e(x.into()),
            "1111_xxxx_0010_1001" => self.opcode_fx29(x.into()),
            "1111_xxxx_0011_0011" => self.opcode_fx33(x.into()),
            "1111_xxxx_0101_0101" => self.opcode_fx55(x.into()),
            "1111_xxxx_0110_0101" => self.opcode_fx65(x.into()),
            "aaaa_aaaa_aaaa_aaaa" => panic!("Unknown instruction: {}", a),
            _ => {}
        }
    }

    fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer = self.delay_timer - 1;
        }
        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("BEEP");
                self.sound_timer = self.sound_timer - 1;
            }
        }
    }

    fn opcode_00e0(&mut self) { // 0x00E0: Clears the screen
        for pixel in self.gfx.iter_mut() {
            *pixel = false;
        }
        self.pc += 2;
        self.draw_flag = true;
    }

    fn opcode_00ee(&mut self) { // 0x00EE: Returns from subroutine
        self.pc = self.stack.pop().unwrap();
    }

    fn opcode_1nnn(&mut self, address: usize) { // 0x1NNN: Jumps to address NNN
        self.pc = address;
    }

    fn opcode_2nnn(&mut self, address: usize) { // 0x2NNN: Calls subroutine at NNN
        self.stack.push(self.pc);
        self.pc = address;
    }

    fn opcode_3xnn(&mut self, x: usize, nn: u8) { // 0x3XNN: Skips the next instruction if VX equals NN. (Usually the next instruction is a jump to skip a code block)
        match self.v[x] == nn {
            true => self.pc += 4,
            false => self.pc += 2,
        }
    }

    fn opcode_4xnn(&mut self, x: usize, nn: u8) { // 0x4XNN: Skips the next instruction if VX does not equal NN. (Usually the next instruction is a jump to skip a code block)
        match self.v[x] != nn {
            true => self.pc += 4,
            false => self.pc += 2,
        }
    }

    fn opcode_5xy0(&mut self, x: usize, y: usize) { // 0x5XY0: Skips the next instruction if VX equals VY. (Usually the next instruction is a jump to skip a code block)
        match self.v[x] == self.v[y] {
            true => self.pc += 4,
            false => self.pc += 2,
        }
    }

    fn opcode_6xnn(&mut self, x: usize, nn: u8) { // 0x6XNN: Sets VX to NN
        self.v[x] = nn;
        self.pc += 2;
    }

    fn opcode_7xnn(&mut self, x: usize, nn: u8) { // 0x7XNN: Adds NN to VX. (Carry flag is not changed)
        self.v[x] += nn;
        self.pc += 2;
    }

    fn opcode_8xy0(&mut self, x: usize, y: usize) { // 0x8XY0: Sets VX to the value of VY
        self.v[x] = self.v[y];
        self.pc += 2;
    }

    fn opcode_8xy1(&mut self, x: usize, y: usize) { // 0x8XY1: Sets VX to VX or VY. (Bitwise OR operation)
        self.v[x] = self.v[x] | self.v[y];
        self.pc += 2;
    }

    fn opcode_8xy2(&mut self, x: usize, y: usize) { // 0x8XY2: Sets VX to VX and VY. (Bitwise AND operation)
        self.v[x] = self.v[x] & self.v[y];
        self.pc += 2;
    }

    fn opcode_8xy3(&mut self, x: usize, y: usize) { // 0x8XY3: Sets VX to VX xor VY
        self.v[x] = self.v[x] ^ self.v[y];
        self.pc += 2;
    }

    fn opcode_8xy4(&mut self, x: usize, y: usize) { // 0x8XY4: Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there is not
        self.pc += 2;
        unimplemented!();
    }

    fn opcode_8xy5(&mut self, x: usize, y: usize) { // 0x8XY5: VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there is not
        self.pc += 2;
        unimplemented!();
    }

    fn opcode_8xy6(&mut self, x: usize, y: usize) { // 0x8XY6: Stores the least significant bit of VX in VF and then shifts VX to the right by 1
        self.pc += 2;
        unimplemented!();
    }

    fn opcode_8xy7(&mut self, x: usize, y: usize) { // 0x8XY7: Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there is not
        self.pc += 2;
        unimplemented!();
    }

    fn opcode_8xye(&mut self, x: usize, y: usize) { // 0x8XYE: Stores the most significant bit of VX in VF and then shifts VX to the left by 1
        self.pc += 2;
        unimplemented!();
    }

    fn opcode_9xy0(&mut self, x: usize, y: usize) { // 0x9XY0: Skips the next instruction if VX does not equal VY. (Usually the next instruction is a jump to skip a code block)
        match self.v[x] != self.v[y] {
            true => self.pc += 4,
            false => self.pc += 2,
        }
    }

    fn opcode_annn(&mut self, address: usize) { // 0xANNN: Sets i to the address NNN
        self.i = address as u16;
        self.pc += 2;
    }

    fn opcode_bnnn(&mut self, address: usize) { // 0xBNNN: Jumps to the address NNN plus V0
        self.pc = address + self.v[0] as usize;
        self.pc += 2;
    }

    fn opcode_cxnn(&mut self, x: usize, nn: u8) { // 0xCXNN: Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN
        let random: u8 = rand::thread_rng().gen_range(0..=255);
        self.v[x] = nn & random;
        self.pc += 2;
    }

    // 0xDXYN: Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels.
    // Each row of 8 pixels is read as bit-coded starting from memory location i; i value does not change
    // after the execution of this instruction. As described above, VF is set to 1 if any screen pixels are
    // flipped from set to unset when the sprite is drawn, and to 0 if that does not happen
    fn opcode_dxyn(&mut self, x: usize, y: usize, n: usize) {
        let sprite = Sprite { x: self.v[x] as usize, y: self.v[y] as usize, rows: self.v[n] as usize };

        self.v[0xF] = 0; // cf to 0

        for y_line in 0..sprite.rows {
            let pixel = self.memory[self.i as usize + y_line as usize];
            for x_line in 0..8 {
                if (pixel & (0x80 >> x_line)) != 0 {
                    let position: usize = (sprite.x + x_line + (sprite.y + y_line) * 64) % 2048;

                    if self.gfx[position]  {
                        self.v[0xF] = 1;
                    }

                    self.gfx[position] = !self.gfx[position];
                }
            }
        }

        self.pc += 2;
        self.draw_flag = true;
    }

    fn opcode_ex9e(&mut self, x: usize) { // 0xEX9E: Skips the next instruction if the key stored in VX is pressed. (Usually the next instruction is a jump to skip a code block)
        if self.key_pressed[self.v[x] as usize] {
            self.pc += 2;
        }
        self.pc += 2;
    }

    fn opcode_exa1(&mut self, x: usize) { // 0xEX9E: Skips the next instruction if the key stored in VX is not pressed. (Usually the next instruction is a jump to skip a code block)
        if self.key_pressed[self.v[x] as usize].not() {
            self.pc += 2;
        }
        self.pc += 2;
    }

    fn opcode_fx07(&mut self, x: usize) { // 0xFX07: Sets VX to the value of the delay timer
        self.v[x] = self.delay_timer;
        self.pc += 2;
    }

    fn opcode_fx0a(&mut self, x: usize) { // 0xFX0A: A key press is awaited, and then stored in VX. (Blocking Operation. All instruction halted until next key event)
        self.pc += 2;
        unimplemented!();
    }

    fn opcode_fx15(&mut self, x: usize) { // 0xFX15: Sets the delay timer to VX
        self.delay_timer = self.v[x];
        self.pc += 2;
    }

    fn opcode_fx18(&mut self, x: usize) { // 0xFX18: Sets the sound timer to VX
        self.sound_timer = self.v[x];
        self.pc += 2;
    }

    fn opcode_fx1e(&mut self, x: usize) { // 0xFX1E: Adds VX to i. VF is not affected
        self.i += self.v[x] as u16;
        self.pc += 2;
    }

    fn opcode_fx29(&mut self, x: usize) { // 0xFX29: Sets i to the location of the sprite for the character in VX. Characters 0-F (in hexadecimal) are represented by a 4x5 font
        let offset: u16 = self.v[x] as u16 * 5;
        self.i = 0x50 + offset;
        self.pc += 2;
    }

    fn opcode_fx33(&mut self, x: usize) { // 0xFX33: Stores the binary-coded decimal representation of VX, with the most significant of three digits at the address in i, the middle digit at i plus 1, and the least significant digit at i plus 2. (In other words, take the decimal representation of VX, place the hundreds digit in memory at location in i, the tens digit at location i+1, and the ones digit at location i+2.);
        self.memory[self.i] = self.v[x] / 100;
        self.memory[self.i + 1] = (self.v[x] % 100) / 10;
        self.memory[self.i + 2] = self.v[x] % 10;
        self.pc += 2;
    }

    fn opcode_fx55(&mut self, x: usize) { // 0xFX55: Stores from V0 to VX (including VX) in memory, starting at address i. The offset from i is increased by 1 for each value written, but i itself is left unmodified
        for i in 0..=x {
            self.memory[self.i as usize + i] = self.v[i];
        }
        self.pc += 2;
    }

    fn opcode_fx65(&mut self, x: usize) { // 0xFX65: Fills from V0 to VX (including VX) with values from memory, starting at address i. The offset from i is increased by 1 for each value written, but i itself is left unmodified
        for i in 0..=x {
            self.v[i] = self.memory[self.i as usize + i];
        }
        self.pc += 2;
    }
}
