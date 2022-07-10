use bitflags::bitflags;
use byteorder::{ByteOrder, LittleEndian};

const INITIAL_STATUS_FLAGS: StatusFlags = StatusFlags::from_bits_truncate(0b1001_0000);

bitflags! {
  pub struct StatusFlags: u8 {
      const CARRY = 0b000;
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
    fetched: u8,
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
            fetched: 0,
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
            let instruction = self.fetch_instruction().expect("Invalid opcode");

            (instruction.mode)(self);
            (instruction.operate)(self);

            self.registers.pc += instruction.cycles as u16;

            if self.status.contains(StatusFlags::INTERRUPT_DISABLE) {
                break;
            }
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

    // Address mode functions
    fn implied(&mut self) {}

    fn immediate(&mut self) {
        self.fetched = self.read(self.registers.pc + 1);
    }
}

mod opcodes;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lda() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x01, 0x00]);
        assert_eq!(cpu.registers.a, 0x01);
    }

    #[test]
    fn test_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x00, 0x00]);
        assert!(cpu.status.contains(StatusFlags::ZERO));
    }

    #[test]
    fn test_lda_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x80, 0x00]);
        assert!(cpu.status.contains(StatusFlags::NEGATIVE));
    }

    #[test]
    fn test_tax() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x01, 0xAA, 0x00]);
        assert_eq!(cpu.registers.x, 0x01);
    }

    #[test]
    fn test_tax_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x00, 0xAA, 0x00]);
        assert!(cpu.status.contains(StatusFlags::ZERO));
    }

    #[test]
    fn test_tax_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x80, 0xAA, 0x00]);
        assert!(cpu.status.contains(StatusFlags::NEGATIVE));
    }
}
