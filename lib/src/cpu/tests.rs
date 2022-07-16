use super::*;

#[test]
fn test_addressing_mode_immediate() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0x20, // LDA #$20
    ]);
    assert_eq!(cpu.registers.a, 0x20);
}

#[test]
fn test_addressing_mode_absolute() {
    let mut cpu = CPU::new();
    cpu.write_word(0x4000, 0xFF);
    cpu.load_and_run(vec![
        0xAD, 0x00, 0x40, // LDA $4000
    ]);
    assert_eq!(cpu.registers.a, 0xFF);
}

#[test]
fn test_addressing_mode_absolute_x() {
    let mut cpu = CPU::new();
    cpu.write_word(0x4010, 0xFF);
    cpu.load_and_run(vec![
        0xA2, 0x10, // LDX #$10
        0xBD, 0x00, 0x40, // LDA $4000,X
    ]);
    assert_eq!(cpu.registers.a, 0xFF);
}

#[test]
fn test_addressing_mode_absolute_y() {
    let mut cpu = CPU::new();
    cpu.write_word(0x4010, 0xFF);
    cpu.load_and_run(vec![
        0xA0, 0x10, // LDY #$10
        0xB9, 0x00, 0x40, // LDA $4000,Y
    ]);
    assert_eq!(cpu.registers.a, 0xFF);
}

#[test]
fn test_addressing_mode_zero_page() {
    let mut cpu = CPU::new();
    cpu.write(0x00FF, 0x20);
    cpu.load_and_run(vec![
        0xA5, 0xFF, // LDA $80
    ]);
    assert_eq!(cpu.registers.a, 0x20);
}

#[test]
fn test_addressing_mode_zero_page_x() {
    let mut cpu = CPU::new();
    cpu.write(0x000F, 0x20);
    cpu.load_and_run(vec![
        0xA2, 0x10, // LDX #$10
        0xB5, 0xFF, // LDA $80,X
    ]);
    assert_eq!(cpu.registers.a, 0x20);
}

#[test]
fn test_addressing_mode_zero_page_y() {
    let mut cpu = CPU::new();
    cpu.write(0x000F, 0x20);
    cpu.load_and_run(vec![
        0xA0, 0x10, // LDY #$10
        0xB6, 0xFF, // LDX $80,Y
    ]);
    assert_eq!(cpu.registers.x, 0x20);
}

#[test]
fn test_addressing_mode_indirect() {
    let mut cpu = CPU::new();
    cpu.write_word(0x1000, 0xff00);
    cpu.load_and_run(vec![
        0x6C, 0x00, 0x10, // JMP ($1000)
    ]);
    assert_eq!(cpu.registers.pc, 0xff00);
}

#[test]
fn test_addressing_mode_indirect_x() {
    let mut cpu = CPU::new();
    cpu.write_word(0x0026, 0x40ff);
    cpu.write(0x40ff, 0x42);
    cpu.load_and_run(vec![
        0xA2, 0x06, // LDX #$06
        0xA1, 0x20, // LDA ($20,X)
    ]);
    assert_eq!(cpu.registers.a, 0x42);
}

#[test]
fn test_addressing_mode_indirect_y() {
    let mut cpu = CPU::new();
    cpu.write_word(0x0020, 0x4080);
    cpu.write(0x4086, 0x42);
    cpu.load_and_run(vec![
        0xA0, 0x06, // LDY #$06
        0xB1, 0x20, // LDA ($20),Y
    ]);
    assert_eq!(cpu.registers.a, 0x42);
}

#[test]
fn test_adc() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0xF0, // LDA #$F0
        0x69, 0x0F, // ADC #$0F
    ]);
    assert_eq!(cpu.registers.a, 0xFF);
}

#[test]
fn test_adc_with_carry() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0x38, // SEC
        0xA9, 0x40, // LDA #$40
        0x69, 0x0F, // ADC #$0F
    ]);
    assert_eq!(cpu.registers.a, 0x50);
}

#[test]
fn test_adc_with_overflow() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0x40, // LDA #$40
        0x69, 0x40, // ADC #$40
    ]);
    assert!(cpu.status.contains(StatusFlags::OVERFLOW));
}

#[test]
fn test_and() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9,        // LDA
        0b1111_0000, // #%00001111
        0x29,        // AND
        0b0011_1111, // #%11110011
    ]);
    assert_eq!(cpu.registers.a, 0b0011_0000);
}

#[test]
fn test_asl() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0x04, // LDA #$04
        0x0A, // ASL
    ]);
    assert_eq!(cpu.registers.a, 0x08);
}

#[test]
fn test_bcc() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0x90, 0x20, // BCC $20
    ]);
    assert_eq!(cpu.registers.pc, 0x8022);
}

#[test]
fn test_bcs() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0x38, // SEC
        0xB0, 0x20, // BCC $20
    ]);
    assert_eq!(cpu.registers.pc, 0x8023);
}

#[test]
fn test_beq() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0x03, // LDA #$03
        0xE9, 0x01, // SBC #$01
        0xF0, 0xFC, // BEQ $FA
    ]);
    assert_eq!(cpu.registers.a, 0x01);
}

#[test]
fn test_bit_zero() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0x80, // LDA #$80
        0x24, 0x10, // BIT $10
    ]);
    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_bit_negative() {
    let mut cpu = CPU::new();
    cpu.write(0x10, 0xFF);
    cpu.load_and_run(vec![
        0xA9, 0xFF, // LDA #$FF
        0x24, 0x10, // BIT $10
    ]);
    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_bmi() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0xF0, // LDA #$F0
        0x30, 0x10, // BMI $10
    ]);
    assert_eq!(cpu.registers.pc, 0x8014);
}

#[test]
fn test_bne() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0x03, // LDA #$03
        0xE9, 0x01, // SBC #$01
        0xD0, 0xFC, // BNE $FA
    ]);
    assert_eq!(cpu.registers.pc, 0x8006);
}

#[test]
fn test_bpl() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0x10, // LDA #$10
        0x10, 0x10, // BPL $10
    ]);
    assert_eq!(cpu.registers.pc, 0x8014);
}

#[test]
fn test_bvc() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0x50, 0x20, // BVC $20
    ]);
    assert_eq!(cpu.registers.pc, 0x8022);
}

#[test]
fn test_bvs() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0x40, // LDA #$40
        0xE9, 0x80, // SBC #$80
        0x70, 0x10, // BVS $10
    ]);
    assert_eq!(cpu.registers.pc, 0x8016);
}

#[test]
fn test_asl_with_carry() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0x80, // LDA #$80
        0x0A, // ASL
    ]);
    assert_eq!(cpu.registers.a, 0x00);
    assert!(cpu.status.contains(StatusFlags::CARRY));
}

#[test]
fn test_clc() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0x38, // SEC
        0x18, // CLC
    ]);
    assert!(!cpu.status.contains(StatusFlags::CARRY));
}

#[test]
fn test_cld() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xD8, // CLD
    ]);
    assert!(!cpu.status.contains(StatusFlags::DECIMAL));
}

#[test]
fn test_cli() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0x58, // CLI
    ]);
    assert!(!cpu.status.contains(StatusFlags::INTERRUPT_DISABLE));
}

#[test]
fn test_clv() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xB8, // CLV
    ]);
    assert!(!cpu.status.contains(StatusFlags::OVERFLOW));
}

#[test]
fn test_cmp() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0x80, // LDA #$80
        0xC9, 0x80, // CMP #$80
    ]);
    assert_eq!(cpu.registers.a, 0x80);
}

#[test]
fn test_cmp_negative() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0x80, // LDA #$80
        0xC9, 0x81, // CMP #$81
    ]);
    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cpx() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0x80, // LDA #$80
        0xE0, 0x80, // CPX #$80
    ]);
    assert_eq!(cpu.registers.a, 0x80);
}

#[test]
fn test_cpy() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0x80, // LDA #$80
        0xC0, 0x80, // CPY #$80
    ]);
    assert_eq!(cpu.registers.a, 0x80);
}

#[test]
fn test_dec() {
    let mut cpu = CPU::new();
    cpu.write(0x20, 0x80);
    cpu.load_and_run(vec![
        0xC6, 0x20, // DEC $20
    ]);
    assert_eq!(cpu.read(0x20), 0x7F);
}

#[test]
fn test_dex() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA2, 0x80, // LDX #$80
        0xCA, // DEX
    ]);
    assert_eq!(cpu.registers.x, 0x7F);
}

#[test]
fn test_dey() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA0, 0x80, // LDY #$80
        0x88, // DEY
    ]);
    assert_eq!(cpu.registers.y, 0x7F);
}

#[test]
fn test_eor() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9,        // LDA
        0b1100_1100, // #%11001100
        0x49,        // EOR
        0b0000_1111, // #%11110000
    ]);
    assert_eq!(cpu.registers.a, 0b1100_0011);
}

#[test]
fn test_inc() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0x01, // LDA #$01
        0x85, 0x80, // STA $80
        0xE6, 0x80, // INC $80
    ]);
    assert!(cpu.read(0x80) == 0x02);
}

#[test]
fn test_inx() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA2, 0x01, // LDX #$01
        0xE8, // INX
    ]);
    assert_eq!(cpu.registers.x, 0x02);
}

#[test]
fn test_iny() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA0, 0x01, // LDY #$01
        0xC8, // INY
    ]);
    assert!(cpu.registers.y == 0x02);
}

#[test]
fn test_jmp() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0x4C, 0x00, 0x40, // JMP $4000
    ]);
    assert_eq!(cpu.registers.pc, 0x4000);
}

#[test]
fn test_lda() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0x01, // LDA #$01
    ]);
    assert_eq!(cpu.registers.a, 0x01);
}

#[test]
fn test_ldx() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA2, 0x01, // LDX #$01
    ]);
    assert_eq!(cpu.registers.x, 0x01);
}

#[test]
fn test_ldy() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA0, 0x01, // LDY #$01
    ]);
    assert_eq!(cpu.registers.y, 0x01);
}

#[test]
fn test_lsr() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0x04, // LDA #$04
        0x4A, // LSR
    ]);
    assert_eq!(cpu.registers.a, 0x02);
}

#[test]
fn test_lsr_with_carry() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0x11, // LDA #$80
        0x4A, // LSR
    ]);
    assert!(cpu.status.contains(StatusFlags::CARRY));
}

#[test]
fn test_ora() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9,        // LDA
        0b1100_1100, // #%11001100
        0x09,        // ORA
        0b0000_1111, // #%11110000
    ]);
    assert_eq!(cpu.registers.a, 0b1100_1111);
}

#[test]
fn test_pha() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0x80, // LDA #$80
        0x48, // PHA
    ]);
    assert_eq!(cpu.read(0x0100), 0x80);
}

#[test]
fn test_php() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0x08, // PHP
    ]);
    assert_eq!(cpu.read(0x0100), 0b0010_0100);
}

#[test]
fn test_pla() {
    let mut cpu = CPU::new();
    cpu.write(0x0100, 0x80);
    cpu.load_and_run(vec![
        0xA9, 0x80, // LDA #$80
        0x48, // PHA
        0xA9, 0x00, // LDA #$00
        0x68, // PLA
    ]);
    assert_eq!(cpu.registers.a, 0x80);
}

#[test]
fn test_plp() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0x38, // SEC
        0x08, // PHP
        0x18, // CLI
        0x28, // PLP
    ]);
    assert!(cpu.status.contains(StatusFlags::CARRY));
}

#[test]
fn test_rol() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9,        // LDA
        0b1100_1100, // #%11001100
        0x2A,        // ROL
    ]);
    assert_eq!(cpu.registers.a, 0b1001_1001);
}

#[test]
fn test_ror() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9,        // LDA
        0b1100_1100, // #%11001100
        0x6A,        // ROR
    ]);
    assert_eq!(cpu.registers.a, 0b0110_0110);
}

#[test]
fn test_sbc() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0xF0, // LDA #$F0
        0xE9, 0x08, // SBC #$08
    ]);
    assert_eq!(cpu.registers.a, 0xE7);
}

#[test]
fn test_sbc_with_carry() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0x38, // SEC
        0xA9, 0x40, // LDA #$40
        0xE9, 0x08, // SBC #$08
    ]);
    assert_eq!(cpu.registers.a, 0x38);
}

#[test]
fn test_sbc_with_overflow() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0x40, // LDA #$40
        0xE9, 0x80, // SBC #$80
    ]);
    assert!(cpu.status.contains(StatusFlags::OVERFLOW));
}

#[test]
fn test_sec() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0x38, // SEC
    ]);
    assert!(cpu.status.contains(StatusFlags::CARRY));
}

#[test]
fn test_sed() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xF8, // SED
    ]);
    assert!(cpu.status.contains(StatusFlags::DECIMAL));
}

#[test]
fn test_sei() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0x58, // CLI
        0x78, // SEI
    ]);
    assert!(cpu.status.contains(StatusFlags::INTERRUPT_DISABLE));
}

#[test]
fn test_sta() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0x01, // LDA #$01
        0x8D, 0x00, 0x40, // STA $4000
    ]);
    assert_eq!(cpu.read(0x4000), 0x01);
}

#[test]
fn test_stx() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA2, 0x01, // LDX #$01
        0x8E, 0x00, 0x40, // STX $4000
    ]);
    assert_eq!(cpu.read(0x4000), 0x01);
}

#[test]
fn test_sty() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA0, 0x01, // LDY #$01
        0x8C, 0x00, 0x40, // STY $4000
    ]);
    assert_eq!(cpu.read(0x4000), 0x01);
}

#[test]
fn test_tax() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0x01, // LDA #$01
        0xAA, // TAX
    ]);
    assert_eq!(cpu.registers.x, 0x01);
}

#[test]
fn test_tay() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0x01, // LDA #$01
        0x8A, // TAY
    ]);
    assert_eq!(cpu.registers.y, 0x01);
}

#[test]
fn test_txa() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0x42, // LDA #$42
        0xAA, // TAX
        0xA9, 0x01, // LDA #$01
        0xA8, // TXA
    ]);
    assert_eq!(cpu.registers.a, 0x42);
}

#[test]
fn test_tya() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0x42, // LDA #$42
        0x8A, // TAY
        0xA9, 0x01, // LDA #$01
        0x98, // TYA
    ]);
    assert_eq!(cpu.registers.a, 0x42);
}
