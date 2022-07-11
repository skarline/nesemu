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
    assert_eq!(cpu.registers.pc, 0xff01);
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
        0xA9, 0xFF, // LDA #$FF
        0x69, 0x01, // ADC #$01
    ]);
    assert!(cpu.status.contains(StatusFlags::OVERFLOW));
}

#[test]
fn test_jmp() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0x4C, 0x00, 0x40, // JMP $4000
    ]);
    assert_eq!(cpu.registers.pc, 0x4001);
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
fn test_lda_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0x00, // LDA #$00
    ]);
    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_lda_negative_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0x80, // LDA #$80
    ]);
    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
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
fn test_tax_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0x00, // LDA #$00
        0xAA,
    ]);
    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_tax_negative_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0x80, // LDA #$80
        0xAA,
    ]);
    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}
