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
    pub fn fetch_instruction(&self, opcode: u8) -> Instruction {
        match opcode {
            // Add with Carry
            0x69 => Instruction::new(2, CPU::adc, CPU::immediate),
            0x65 => Instruction::new(3, CPU::adc, CPU::zero_page),
            0x75 => Instruction::new(4, CPU::adc, CPU::zero_page_x),
            0x6D => Instruction::new(4, CPU::adc, CPU::absolute),
            0x7D => Instruction::new(4, CPU::adc, CPU::absolute_x),
            0x79 => Instruction::new(4, CPU::adc, CPU::absolute_y),
            0x61 => Instruction::new(6, CPU::adc, CPU::indirect_x),
            0x71 => Instruction::new(5, CPU::adc, CPU::indirect_y),

            // Logical AND
            0x29 => Instruction::new(2, CPU::and, CPU::immediate),
            0x25 => Instruction::new(3, CPU::and, CPU::zero_page),
            0x35 => Instruction::new(4, CPU::and, CPU::zero_page_x),
            0x2D => Instruction::new(4, CPU::and, CPU::absolute),
            0x3D => Instruction::new(4, CPU::and, CPU::absolute_x),
            0x39 => Instruction::new(4, CPU::and, CPU::absolute_y),
            0x21 => Instruction::new(6, CPU::and, CPU::indirect_x),
            0x31 => Instruction::new(5, CPU::and, CPU::indirect_y),

            // Arithmetic Shift Left
            0x0A => Instruction::new(2, CPU::asl, CPU::implied),
            0x06 => Instruction::new(5, CPU::asl, CPU::zero_page),
            0x16 => Instruction::new(6, CPU::asl, CPU::zero_page_x),
            0x0E => Instruction::new(6, CPU::asl, CPU::absolute),
            0x1E => Instruction::new(7, CPU::asl, CPU::absolute_x),

            // Branch if Carry Clear
            0x90 => Instruction::new(2, CPU::bcc, CPU::immediate),

            // Branch if Carry Set
            0xB0 => Instruction::new(2, CPU::bcs, CPU::immediate),

            // Branch if Equal
            0xF0 => Instruction::new(2, CPU::beq, CPU::immediate),

            // Bit Test
            0x24 => Instruction::new(3, CPU::bit, CPU::zero_page),
            0x2C => Instruction::new(4, CPU::bit, CPU::absolute),

            // Branch if Minus
            0x30 => Instruction::new(2, CPU::bmi, CPU::immediate),

            // Branch if Not Equal
            0xD0 => Instruction::new(2, CPU::bne, CPU::immediate),

            // Branch if Positive
            0x10 => Instruction::new(2, CPU::bpl, CPU::immediate),

            // Break
            0x00 => Instruction::new(2, CPU::brk, CPU::implied),

            // Branch if Overflow Clear
            0x50 => Instruction::new(2, CPU::bvc, CPU::immediate),

            // Branch if Overflow Set
            0x70 => Instruction::new(2, CPU::bvs, CPU::immediate),

            // Clear Carry Flag
            0x18 => Instruction::new(2, CPU::clc, CPU::implied),

            // Clear Decimal Flag
            0xD8 => Instruction::new(2, CPU::cld, CPU::implied),

            // Clear Interrupt Disable
            0x58 => Instruction::new(2, CPU::cli, CPU::implied),

            // Clear Overflow Flag
            0xB8 => Instruction::new(2, CPU::clv, CPU::implied),

            // Compare Accumulator
            0xC9 => Instruction::new(2, CPU::cmp, CPU::immediate),
            0xC5 => Instruction::new(2, CPU::cmp, CPU::zero_page),
            0xD5 => Instruction::new(2, CPU::cmp, CPU::zero_page_x),
            0xCD => Instruction::new(3, CPU::cmp, CPU::absolute),
            0xDD => Instruction::new(3, CPU::cmp, CPU::absolute_x),
            0xD9 => Instruction::new(3, CPU::cmp, CPU::absolute_y),
            0xC1 => Instruction::new(2, CPU::cmp, CPU::indirect_x),
            0xD1 => Instruction::new(2, CPU::cmp, CPU::indirect_y),

            // Compare X
            0xE0 => Instruction::new(2, CPU::cpx, CPU::immediate),
            0xE4 => Instruction::new(3, CPU::cpx, CPU::zero_page),
            0xEC => Instruction::new(4, CPU::cpx, CPU::absolute),

            // Compare Y
            0xC0 => Instruction::new(2, CPU::cpy, CPU::immediate),
            0xC4 => Instruction::new(3, CPU::cpy, CPU::zero_page),
            0xCC => Instruction::new(4, CPU::cpy, CPU::absolute),

            // Decrement Memory
            0xC6 => Instruction::new(5, CPU::dec, CPU::zero_page),
            0xD6 => Instruction::new(6, CPU::dec, CPU::zero_page_x),
            0xCE => Instruction::new(3, CPU::dec, CPU::absolute),
            0xDE => Instruction::new(4, CPU::dec, CPU::absolute_x),

            // Decrement X
            0xCA => Instruction::new(2, CPU::dex, CPU::implied),

            // Decrement Y
            0x88 => Instruction::new(2, CPU::dey, CPU::implied),

            // Exclusive OR
            0x49 => Instruction::new(2, CPU::eor, CPU::immediate),
            0x45 => Instruction::new(3, CPU::eor, CPU::zero_page),
            0x55 => Instruction::new(4, CPU::eor, CPU::zero_page_x),
            0x4D => Instruction::new(4, CPU::eor, CPU::absolute),
            0x5D => Instruction::new(4, CPU::eor, CPU::absolute_x),
            0x59 => Instruction::new(4, CPU::eor, CPU::absolute_y),
            0x41 => Instruction::new(6, CPU::eor, CPU::indirect_x),
            0x51 => Instruction::new(5, CPU::eor, CPU::indirect_y),

            // Increment Memory
            0xE6 => Instruction::new(5, CPU::inc, CPU::zero_page),
            0xF6 => Instruction::new(6, CPU::inc, CPU::zero_page_x),
            0xEE => Instruction::new(6, CPU::inc, CPU::absolute),
            0xFE => Instruction::new(7, CPU::inc, CPU::absolute_x),

            // Increment X
            0xE8 => Instruction::new(2, CPU::inx, CPU::implied),

            // Increment Y
            0xC8 => Instruction::new(2, CPU::iny, CPU::implied),

            // Jump
            0x4C => Instruction::new(3, CPU::jmp, CPU::absolute),
            0x6C => Instruction::new(5, CPU::jmp, CPU::indirect),

            // Jump to Subroutine
            0x20 => Instruction::new(6, CPU::jsr, CPU::absolute),

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

            // Logical Shift Right
            0x4A => Instruction::new(2, CPU::lsr, CPU::implied),
            0x46 => Instruction::new(5, CPU::lsr, CPU::zero_page),
            0x56 => Instruction::new(6, CPU::lsr, CPU::zero_page_x),
            0x4E => Instruction::new(6, CPU::lsr, CPU::absolute),
            0x5E => Instruction::new(7, CPU::lsr, CPU::absolute_x),

            // No Operation
            0xEA => Instruction::new(2, CPU::nop, CPU::implied),

            // Logical Inclusive OR
            0x09 => Instruction::new(2, CPU::ora, CPU::immediate),
            0x05 => Instruction::new(3, CPU::ora, CPU::zero_page),
            0x15 => Instruction::new(4, CPU::ora, CPU::zero_page_x),
            0x0D => Instruction::new(4, CPU::ora, CPU::absolute),
            0x1D => Instruction::new(4, CPU::ora, CPU::absolute_x),
            0x19 => Instruction::new(4, CPU::ora, CPU::absolute_y),
            0x01 => Instruction::new(6, CPU::ora, CPU::indirect_x),
            0x11 => Instruction::new(5, CPU::ora, CPU::indirect_y),

            // Push Accumulator
            0x48 => Instruction::new(3, CPU::pha, CPU::implied),

            // Push Processor Status
            0x08 => Instruction::new(3, CPU::php, CPU::implied),

            // Pull Accumulator
            0x68 => Instruction::new(4, CPU::pla, CPU::implied),

            // Pull Processor Status
            0x28 => Instruction::new(4, CPU::plp, CPU::implied),

            // Rotate Left
            0x2A => Instruction::new(2, CPU::rol, CPU::implied),
            0x26 => Instruction::new(5, CPU::rol, CPU::zero_page),
            0x36 => Instruction::new(6, CPU::rol, CPU::zero_page_x),
            0x2E => Instruction::new(6, CPU::rol, CPU::absolute),
            0x3E => Instruction::new(7, CPU::rol, CPU::absolute_x),

            // Rotate Right
            0x6A => Instruction::new(2, CPU::ror, CPU::implied),
            0x66 => Instruction::new(5, CPU::ror, CPU::zero_page),
            0x76 => Instruction::new(6, CPU::ror, CPU::zero_page_x),
            0x6E => Instruction::new(6, CPU::ror, CPU::absolute),
            0x7E => Instruction::new(7, CPU::ror, CPU::absolute_x),

            // Return from Subroutine
            0x60 => Instruction::new(6, CPU::rts, CPU::implied),

            // Subtract with Carry
            0xE9 => Instruction::new(2, CPU::sbc, CPU::immediate),
            0xE5 => Instruction::new(3, CPU::sbc, CPU::zero_page),
            0xF5 => Instruction::new(4, CPU::sbc, CPU::zero_page_x),
            0xED => Instruction::new(4, CPU::sbc, CPU::absolute),
            0xFD => Instruction::new(4, CPU::sbc, CPU::absolute_x),
            0xF9 => Instruction::new(4, CPU::sbc, CPU::absolute_y),
            0xE1 => Instruction::new(6, CPU::sbc, CPU::indirect_x),
            0xF1 => Instruction::new(5, CPU::sbc, CPU::indirect_y),

            // Set Carry Flag
            0x38 => Instruction::new(2, CPU::sec, CPU::implied),

            // Set Decimal Flag
            0xF8 => Instruction::new(2, CPU::sed, CPU::implied),

            // Set Interrupt Disable
            0x78 => Instruction::new(2, CPU::sei, CPU::implied),

            // Store Accumulator
            0x85 => Instruction::new(3, CPU::sta, CPU::zero_page),
            0x95 => Instruction::new(4, CPU::sta, CPU::zero_page_x),
            0x8D => Instruction::new(4, CPU::sta, CPU::absolute),
            0x9D => Instruction::new(5, CPU::sta, CPU::absolute_x),
            0x99 => Instruction::new(5, CPU::sta, CPU::absolute_y),
            0x81 => Instruction::new(6, CPU::sta, CPU::indirect_x),
            0x91 => Instruction::new(6, CPU::sta, CPU::indirect_y),

            // Store X
            0x86 => Instruction::new(3, CPU::stx, CPU::zero_page),
            0x96 => Instruction::new(4, CPU::stx, CPU::zero_page_y),
            0x8E => Instruction::new(4, CPU::stx, CPU::absolute),

            // Store Y
            0x84 => Instruction::new(3, CPU::sty, CPU::zero_page),
            0x94 => Instruction::new(4, CPU::sty, CPU::zero_page_x),
            0x8C => Instruction::new(4, CPU::sty, CPU::absolute),

            // Transfer accumulator to X
            0xAA => Instruction::new(2, CPU::tax, CPU::implied),

            // Transfer accumulator to Y
            0x8A => Instruction::new(2, CPU::tay, CPU::implied),

            // Transfer X to accumulator
            0xA8 => Instruction::new(2, CPU::txa, CPU::implied),

            // Transfer Y to accumulator
            0x98 => Instruction::new(2, CPU::tya, CPU::implied),

            _ => panic!("Invalid opcode: {:02X}", opcode),
        }
    }

    fn adc(&mut self) {
        let a = self.registers.a as u16;
        let m = self.mode() as u16;
        let c = self.status.contains(StatusFlags::CARRY) as u16;

        let result = a + m + c;
        let value = result as u8;
        let overflow = !((a ^ m) & (a ^ result)) & 0x80 != 0;

        self.registers.a = value;

        self.set_carry_flag(result > 0xFF);
        self.set_overflow_flag(overflow);
        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn and(&mut self) {
        let value = self.registers.a & self.mode();

        self.registers.a = value;

        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn asl(&mut self) {
        let result = (self.mode() as u16) << 1;
        let value = result as u8;

        self.set_carry_flag(result > 0xFF);
        self.update_zero_flag(value);
        self.update_negative_flag(value);

        if self.implied {
            self.registers.a = value;
        } else {
            self.write(self.addressed, value);
        }
    }

    fn bcc(&mut self) {
        if !self.status.contains(StatusFlags::CARRY) {
            self.branch();
        }
    }

    fn bcs(&mut self) {
        if self.status.contains(StatusFlags::CARRY) {
            self.branch();
        }
    }

    fn beq(&mut self) {
        if self.status.contains(StatusFlags::ZERO) {
            self.branch();
        }
    }

    fn bit(&mut self) {
        let value = self.mode();
        let result = self.registers.a & value;

        self.update_zero_flag(result);
        self.update_negative_flag(value);
        self.set_overflow_flag(value & 0x40 != 0);
    }

    fn bmi(&mut self) {
        if self.status.contains(StatusFlags::NEGATIVE) {
            self.branch();
        }
    }

    fn bne(&mut self) {
        if !self.status.contains(StatusFlags::ZERO) {
            self.branch();
        }
    }

    fn bpl(&mut self) {
        if !self.status.contains(StatusFlags::NEGATIVE) {
            self.branch();
        }
    }

    fn bvc(&mut self) {
        if !self.status.contains(StatusFlags::OVERFLOW) {
            self.branch();
        }
    }

    fn bvs(&mut self) {
        if self.status.contains(StatusFlags::OVERFLOW) {
            self.branch();
        }
    }

    fn brk(&mut self) {
        todo!();
    }

    fn clc(&mut self) {
        self.status.remove(StatusFlags::CARRY);
    }

    fn cld(&mut self) {
        self.status.remove(StatusFlags::DECIMAL);
    }

    fn cli(&mut self) {
        self.status.remove(StatusFlags::INTERRUPT_DISABLE);
    }

    fn clv(&mut self) {
        self.status.remove(StatusFlags::OVERFLOW);
    }

    fn cmp(&mut self) {
        let a = self.registers.a;
        let m = self.mode();

        let value = a.wrapping_sub(m);

        self.set_carry_flag(a >= m);
        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn cpx(&mut self) {
        let x = self.registers.x;
        let m = self.mode();

        let value = x.wrapping_sub(m);

        self.set_carry_flag(x >= m);
        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn cpy(&mut self) {
        let y = self.registers.y;
        let m = self.mode();

        let value = y.wrapping_sub(m);

        self.set_carry_flag(y >= m);
        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn dec(&mut self) {
        let value = self.mode().wrapping_sub(1);

        self.write(self.addressed, value);

        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn dex(&mut self) {
        let value = self.registers.x.wrapping_sub(1);

        self.registers.x = value;

        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn dey(&mut self) {
        let value = self.registers.y.wrapping_sub(1);

        self.registers.y = value;

        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn eor(&mut self) {
        let value = self.registers.a ^ self.mode();

        self.registers.a = value;

        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn inc(&mut self) {
        let value = self.mode().wrapping_add(1);

        self.write(self.addressed, value);

        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn inx(&mut self) {
        let value = self.registers.x.wrapping_add(1);

        self.registers.x = value;

        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn iny(&mut self) {
        let value = self.registers.y.wrapping_add(1);

        self.registers.y = value;

        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn jmp(&mut self) {
        self.registers.pc = self.addressed;
    }

    fn jsr(&mut self) {
        self.push_word(self.registers.pc - 1);
        self.registers.pc = self.addressed;
    }

    fn lda(&mut self) {
        let value = self.mode();

        self.registers.a = value;

        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn ldx(&mut self) {
        let value = self.mode();

        self.registers.x = value;

        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn ldy(&mut self) {
        let value = self.mode();

        self.registers.y = value;

        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn lsr(&mut self) {
        let m = self.mode();

        self.set_carry_flag(m & 0x01 != 0);

        let result = (m as u16) >> 1;
        let value = result as u8;

        self.update_zero_flag(value);
        self.update_negative_flag(value);

        if self.implied {
            self.registers.a = value;
        } else {
            self.write(self.addressed, value);
        }
    }

    fn nop(&mut self) {
        // No operation
    }

    fn ora(&mut self) {
        let value = self.registers.a | self.mode();

        self.registers.a = value;

        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn pha(&mut self) {
        self.push(self.registers.a);
    }

    fn php(&mut self) {
        self.push(self.status.bits());

        self.status.insert(StatusFlags::BREAK);
        self.status.insert(StatusFlags::UNUSED);
    }

    fn pla(&mut self) {
        let value = self.pull();

        self.registers.a = value;

        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn plp(&mut self) {
        self.status.bits = self.pull();
    }

    fn rol(&mut self) {
        let value = self.mode();

        let carry = value & 0x80 != 0;
        let result = value.rotate_left(1) | self.status.contains(StatusFlags::CARRY) as u8;

        if self.implied {
            self.registers.a = result;
        } else {
            self.write(self.addressed, result);
        }

        self.set_carry_flag(carry);
        self.update_zero_flag(result);
        self.update_negative_flag(result);
    }

    fn ror(&mut self) {
        let value = self.mode();

        let carry = value & 0x01 != 0;
        let result = value.rotate_right(1) | self.status.contains(StatusFlags::CARRY) as u8;

        if self.implied {
            self.registers.a = result;
        } else {
            self.write(self.addressed, result);
        }

        self.set_carry_flag(carry);
        self.update_zero_flag(result);
        self.update_negative_flag(result);
    }

    fn rts(&mut self) {
        self.registers.pc = self.pull_word() + 1;
    }

    fn sbc(&mut self) {
        let a = self.registers.a as u16;
        let m = self.mode() as u16 ^ 0xFF;
        let c = self.status.contains(StatusFlags::CARRY) as u16;

        let result = a + m + c;
        let value = result as u8;
        let overflow = !((a ^ m) & (a ^ result)) & 0x80 != 0;

        self.registers.a = value;

        self.set_carry_flag(result > 0xFF);
        self.set_overflow_flag(overflow);
        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn sec(&mut self) {
        self.status.insert(StatusFlags::CARRY);
    }

    fn sed(&mut self) {
        self.status.insert(StatusFlags::DECIMAL);
    }

    fn sei(&mut self) {
        self.status.insert(StatusFlags::INTERRUPT_DISABLE);
    }

    fn sta(&mut self) {
        self.write(self.addressed, self.registers.a);
    }

    fn stx(&mut self) {
        self.write(self.addressed, self.registers.x);
    }

    fn sty(&mut self) {
        self.write(self.addressed, self.registers.y);
    }

    fn tax(&mut self) {
        let value = self.registers.a;

        self.registers.x = value;

        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn tay(&mut self) {
        let value = self.registers.a;

        self.registers.y = value;

        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn txa(&mut self) {
        let value = self.registers.x;

        self.registers.a = value;

        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    fn tya(&mut self) {
        let value = self.registers.y;

        self.registers.a = value;

        self.update_zero_flag(value);
        self.update_negative_flag(value);
    }

    #[inline]
    fn update_zero_flag(&mut self, value: u8) {
        self.status.set(StatusFlags::ZERO, value == 0);
    }

    #[inline]
    fn update_negative_flag(&mut self, value: u8) {
        self.status.set(StatusFlags::NEGATIVE, value & 0x80 != 0);
    }

    #[inline]
    fn set_carry_flag(&mut self, value: bool) {
        self.status.set(StatusFlags::CARRY, value);
    }

    #[inline]
    fn set_overflow_flag(&mut self, value: bool) {
        self.status.set(StatusFlags::OVERFLOW, value);
    }
}
