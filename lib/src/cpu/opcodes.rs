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
    pub fn fetch_instruction(&self) -> Instruction {
        let opcode = self.read(self.registers.pc);

        match opcode {
            // Break
            0x00 => Instruction::new(2, CPU::brk, CPU::implied),

            // Add with Carry
            0x69 => Instruction::new(2, CPU::adc, CPU::immediate),
            0x65 => Instruction::new(3, CPU::adc, CPU::zero_page),
            0x75 => Instruction::new(4, CPU::adc, CPU::zero_page_x),
            0x6D => Instruction::new(4, CPU::adc, CPU::absolute),
            0x7D => Instruction::new(4, CPU::adc, CPU::absolute_x),
            0x79 => Instruction::new(4, CPU::adc, CPU::absolute_y),
            0x61 => Instruction::new(6, CPU::adc, CPU::indirect_x),
            0x71 => Instruction::new(5, CPU::adc, CPU::indirect_y),

            // Jump
            0x4C => Instruction::new(3, CPU::jmp, CPU::absolute),
            0x6C => Instruction::new(5, CPU::jmp, CPU::indirect),

            // Load accumulator
            0xA9 => Instruction::new(2, CPU::lda, CPU::immediate),
            0xA5 => Instruction::new(3, CPU::lda, CPU::zero_page),
            0xB5 => Instruction::new(4, CPU::lda, CPU::zero_page_x),
            0xAD => Instruction::new(4, CPU::lda, CPU::absolute),
            0xBD => Instruction::new(4, CPU::lda, CPU::absolute_x),
            0xB9 => Instruction::new(4, CPU::lda, CPU::absolute_y),
            0xA1 => Instruction::new(6, CPU::lda, CPU::indirect_x),
            0xB1 => Instruction::new(5, CPU::lda, CPU::indirect_y),

            // Load X
            0xA2 => Instruction::new(2, CPU::ldx, CPU::immediate),
            0xA6 => Instruction::new(3, CPU::ldx, CPU::zero_page),
            0xB6 => Instruction::new(4, CPU::ldx, CPU::zero_page_y),
            0xAE => Instruction::new(4, CPU::ldx, CPU::absolute),
            0xBE => Instruction::new(4, CPU::ldx, CPU::absolute_y),

            // Load Y
            0xA0 => Instruction::new(2, CPU::ldy, CPU::immediate),
            0xA4 => Instruction::new(3, CPU::ldy, CPU::zero_page),
            0xB4 => Instruction::new(4, CPU::ldy, CPU::zero_page_x),
            0xAC => Instruction::new(4, CPU::ldy, CPU::absolute),
            0xBC => Instruction::new(4, CPU::ldy, CPU::absolute_x),

            // Transfer accumulator to X
            0xAA => Instruction::new(2, CPU::tax, CPU::implied),

            _ => panic!("Invalid opcode: {:02X}", opcode),
        }
    }

    fn brk(&mut self) {
        self.status.insert(StatusFlags::INTERRUPT_DISABLE);
    }

    fn adc(&mut self) {
        let (result, carry) = u8::overflowing_add(self.registers.a, self.fetch_operand());

        self.registers.a = result;

        self.update_carry_flag(carry);
    }

    fn jmp(&mut self) {
        self.registers.pc = self.operand_address;
    }

    fn lda(&mut self) {
        let value = self.fetch_operand();

        self.registers.a = value;

        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn ldx(&mut self) {
        let value = self.fetch_operand();

        self.registers.x = value;

        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn ldy(&mut self) {
        let value = self.fetch_operand();

        self.registers.y = value;

        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn tax(&mut self) {
        let value = self.registers.a;

        self.registers.x = value;

        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    #[inline]
    fn update_carry_flag(&mut self, value: bool) {
        self.status.set(StatusFlags::CARRY, value);
    }

    #[inline]
    fn update_zero_flag(&mut self, value: u8) {
        self.status.set(StatusFlags::ZERO, value == 0);
    }

    #[inline]
    fn update_negative_flag(&mut self, value: u8) {
        self.status.set(StatusFlags::NEGATIVE, value & 0x80 != 0);
    }
}
