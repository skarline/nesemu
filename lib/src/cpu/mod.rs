use bitflags::bitflags;
use byteorder::{ByteOrder, LittleEndian};

const INITIAL_STATUS_FLAGS: StatusFlags = StatusFlags::from_bits_truncate(0b0010_0100);

bitflags! {
  pub struct StatusFlags: u8 {
      const CARRY = 1 << 0;
      const ZERO = 1 << 1;
      const INTERRUPT_DISABLE = 1 << 2;
      const DECIMAL = 1 << 3;
      const BREAK = 1 << 4;
      const UNUSED = 1 << 5;
      const OVERFLOW = 1 << 6;
      const NEGATIVE = 1 << 7;
  }
}

struct Registers {
    a: u8,
    x: u8,
    y: u8,
    sp: u8,
    pc: u16,
}

pub struct CPU {
    registers: Registers,
    status: StatusFlags,
    addressed: u16,
    implied: bool,
    memory: [u8; 0xFFFF],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: Registers {
                a: 0,
                x: 0,
                y: 0,
                sp: 0,
                pc: 0,
            },
            status: INITIAL_STATUS_FLAGS,
            addressed: 0,
            implied: false,
            memory: [0; 0xFFFF],
        }
    }

    pub fn reset(&mut self) {
        self.registers = Registers {
            a: 0,
            x: 0,
            y: 0,
            sp: 0,
            pc: self.read_word(0xFFFC),
        };
        self.status = INITIAL_STATUS_FLAGS;
    }

    pub fn load(&mut self, data: Vec<u8>) {
        self.memory[0x8000..(0x8000 + data.len())].copy_from_slice(&data);
        self.write_word(0xFFFC, 0x8000);
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.read(self.registers.pc);
            let instruction = self.fetch_instruction(opcode);

            if opcode == 0x00 {
                break;
            }

            self.registers.pc += 1;
            self.implied = false;

            (instruction.mode)(self);
            (instruction.operate)(self);
        }
    }

    pub fn load_and_run(&mut self, data: Vec<u8>) {
        self.load(data);
        self.reset();
        self.run();
    }

    fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    fn write(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    fn read_word(&self, address: u16) -> u16 {
        LittleEndian::read_u16(&self.memory[address as usize..])
    }

    fn write_word(&mut self, address: u16, value: u16) {
        LittleEndian::write_u16(&mut self.memory[address as usize..], value);
    }

    fn implied(&mut self) {
        self.implied = true;
    }

    fn immediate(&mut self) {
        self.addressed = self.registers.pc;
        self.registers.pc += 1;
    }

    fn absolute(&mut self) {
        self.addressed = self.read_word(self.registers.pc);
        self.registers.pc += 2;
    }

    fn absolute_x(&mut self) {
        self.addressed = self
            .read_word(self.registers.pc)
            .wrapping_add(self.registers.x as u16);
        self.registers.pc += 2;
    }

    fn absolute_y(&mut self) {
        self.addressed = self
            .read_word(self.registers.pc)
            .wrapping_add(self.registers.y as u16);
        self.registers.pc += 2;
    }

    fn zero_page(&mut self) {
        self.addressed = self.read(self.registers.pc) as u16;
        self.registers.pc += 1;
    }

    fn zero_page_x(&mut self) {
        self.addressed = self.read(self.registers.pc).wrapping_add(self.registers.x) as u16;
        self.registers.pc += 1;
    }

    fn zero_page_y(&mut self) {
        self.addressed = self.read(self.registers.pc).wrapping_add(self.registers.y) as u16;
        self.registers.pc += 1;
    }

    fn indirect(&mut self) {
        self.addressed = self.read_word(self.read_word(self.registers.pc));
    }

    fn indirect_x(&mut self) {
        let ptr = self.read(self.registers.pc).wrapping_add(self.registers.x) as u16;
        self.addressed = self.read_word(ptr);
        self.registers.pc += 1;
    }

    fn indirect_y(&mut self) {
        let ptr = self.read_word(self.registers.pc);
        self.addressed = self.read_word(ptr).wrapping_add(self.registers.y as u16);
        self.registers.pc += 1;
    }

    fn mode(&mut self) -> u8 {
        if self.implied {
            return self.registers.a;
        }

        self.read(self.addressed)
    }
}

mod opcodes;

#[cfg(test)]
mod tests;
