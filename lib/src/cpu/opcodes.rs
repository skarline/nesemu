use super::*;

pub struct Instruction {
    pub cycles: u8,
    pub operate: fn(&mut CPU),
    pub mode: fn(&mut CPU),
}

impl Instruction {
    pub fn new(cycles: u8, operate: fn(&mut CPU), mode: fn(&mut CPU)) -> Self {
        Instruction {
            cycles,
            operate,
            mode,
        }
    }
}

impl CPU {
    pub fn fetch_instruction(&self) -> Option<Instruction> {
        let opcode = self.read(self.registers.pc);

        match opcode {
            0x00 => Some(Instruction::new(2, CPU::brk, CPU::implied)),
            0xA9 => Some(Instruction::new(2, CPU::lda, CPU::immediate)),
            0xAA => Some(Instruction::new(2, CPU::tax, CPU::implied)),
            _ => None,
        }
    }

    // Force break
    fn brk(&mut self) {
        self.status.insert(StatusFlags::INTERRUPT_DISABLE);
    }

    // Load Accumulator
    fn lda(&mut self) {
        let value = self.fetched as u8;

        self.registers.a = value;
        self.update_zero_and_negative_flags(value);
    }

    // Transfer Accumulator to X
    fn tax(&mut self) {
        let value = self.registers.a;

        self.registers.x = value;
        self.update_zero_and_negative_flags(value);
    }

    fn update_zero_and_negative_flags(&mut self, value: u8) {
        if value == 0 {
            self.status.insert(StatusFlags::ZERO);
        } else {
            self.status.remove(StatusFlags::ZERO);
        }

        if value & 0b1000_0000 != 0 {
            self.status.insert(StatusFlags::NEGATIVE);
        } else {
            self.status.remove(StatusFlags::NEGATIVE);
        }
    }
}
