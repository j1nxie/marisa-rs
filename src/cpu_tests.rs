use super::*;

fn build_cpu() -> Cpu {
    let mut cpu = Cpu::new();
    cpu.pc = 0xF00;
    cpu.v = [0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7];
    cpu
}

#[test]
fn test_initial_state() {
    let mut cpu = Cpu::new();
    cpu.reset();
    assert_eq!(cpu.pc, 0x200);
    assert_eq!(cpu.sp, 0);
    assert_eq!(cpu.stack, [0; 16]);
    assert_eq!(cpu.memory[0..5], [0xF0, 0x90, 0x90, 0x90, 0xF0]);
    assert_eq!(
        cpu.memory[FONT_SET.len() - 5..FONT_SET.len()],
        [0xF0, 0x80, 0xF0, 0x80, 0x80]
    );
}

#[test]
fn test_load_data() {
    let mut cpu = Cpu::new();
    cpu.load(&vec![1, 2, 3]);
    assert_eq!(cpu.memory[0x200], 1);
    assert_eq!(cpu.memory[0x201], 2);
    assert_eq!(cpu.memory[0x202], 3);
}

#[test]
fn test_op_00e0() {
    let mut cpu = build_cpu();
    cpu.display.memory = [[128; WIDTH]; HEIGHT];
    cpu.process_opcode(0x00E0);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            assert_eq!(cpu.display.memory[y][x], 0);
        }
    }

    assert_eq!(cpu.pc, (0xF00 + 2));
}

#[test]
fn test_op_00ee() {
    let mut cpu = Cpu::new();
    cpu.sp = 7;
    cpu.stack[6] = 0x7777;
    cpu.process_opcode(0x00EE);

    assert_eq!(cpu.sp, 6);
    assert_eq!(cpu.pc, 0x7777);
}

#[test]
fn test_op_1nnn() {
    let mut cpu = Cpu::new();
    cpu.process_opcode(0x1777);

    assert_eq!(cpu.pc, 0x0777);
}

#[test]
fn test_op_2nnn() {
    let mut cpu = build_cpu();
    cpu.process_opcode(0x2777);

    assert_eq!(cpu.pc, 0x0777);
    assert_eq!(cpu.sp, 1);
    assert_eq!(cpu.stack[0], (0xF00 + 2));
}

#[test]
fn test_op_3xkk() {
    let mut cpu = build_cpu();
    cpu.process_opcode(0x3201);

    assert_eq!(cpu.pc, (0xF00 + 4));

    cpu.process_opcode(0x3200);

    assert_eq!(cpu.pc, (0xF04 + 2));
}

#[test]
fn test_op_4xkk() {
    let mut cpu = build_cpu();
    cpu.process_opcode(0x4200);

    assert_eq!(cpu.pc, (0xF00 + 4));

    cpu.process_opcode(0x4201);

    assert_eq!(cpu.pc, (0xF04 + 2));
}

#[test]
fn test_op_5xy0() {
    let mut cpu = build_cpu();
    cpu.process_opcode(0x5540);

    assert_eq!(cpu.pc, (0xF00 + 4));

    cpu.process_opcode(0x5500);

    assert_eq!(cpu.pc, (0xF04 + 2));
}

#[test]
fn test_op_6xkk() {
    let mut cpu = build_cpu();
    cpu.process_opcode(0x6577);

    assert_eq!(cpu.v[5], 0x77);
    assert_eq!(cpu.pc, (0xF00 + 2));
}

#[test]
fn test_op_7xkk() {
    let mut cpu = build_cpu();
    cpu.process_opcode(0x75f0);

    assert_eq!(cpu.v[5], 0xf2);
    assert_eq!(cpu.pc, (0xF00 + 2));
}

#[test]
fn test_op_8xy0() {
    let mut cpu = build_cpu();
    cpu.process_opcode(0x8050);

    assert_eq!(cpu.v[0], 0x02);
    assert_eq!(cpu.pc, (0xF00 + 2));
}

#[test]
fn test_op_8xy1() {
    let mut cpu = build_cpu();
    todo!();
}

#[test]
fn test_op_9xy0() {
    let mut cpu = build_cpu();
    cpu.process_opcode(0x90E0);

    assert_eq!(cpu.pc, (0xF00 + 4));

    cpu.process_opcode(0x9010);

    assert_eq!(cpu.pc, (0xF04 + 2));
}

#[test]
fn test_op_annn() {
    let mut cpu = build_cpu();
    cpu.process_opcode(0xA123);

    assert_eq!(cpu.i, 0x123);
}

#[test]
fn test_op_bnnn() {
    let mut cpu = build_cpu();
    cpu.process_opcode(0xB123);

    assert_eq!(cpu.pc, 0x123);
}

#[test]
fn test_op_cxkk() {
    let mut cpu = build_cpu();
    cpu.process_opcode(0xc000);

    assert_eq!(cpu.v[0], 0);

    cpu.process_opcode(0xc00f);

    assert_eq!(cpu.v[0] & 0xf0, 0);
}

#[test]
fn test_op_dxyn() {
    todo!();
}

#[test]
fn test_op_ex9e() {
    let mut cpu = build_cpu();
    cpu.keypad.keys[9] = true;
    cpu.v[5] = 9;
    cpu.process_opcode(0xe59e);

    assert_eq!(cpu.pc, (0xF00 + 4));

    cpu.process_opcode(0xe19e);

    assert_eq!(cpu.pc, (0xF04 + 2));
}

#[test]
fn test_op_exa1() {
    let mut cpu = build_cpu();
    cpu.keypad.keys[9] = true;
    cpu.v[5] = 9;
    cpu.process_opcode(0xe5a1);

    assert_eq!(cpu.pc, (0xF00 + 2));

    cpu.process_opcode(0xe1a1);

    assert_eq!(cpu.pc, (0xF02 + 4));
}

#[test]
fn test_op_fx07() {
    let mut cpu = build_cpu();
    cpu.dt = 20;
    cpu.process_opcode(0xf507);

    assert_eq!(cpu.v[5], 20);
    assert_eq!(cpu.pc, (0xF00 + 2));
}

#[test]
fn test_op_fx0a() {
    let mut cpu = build_cpu();
    cpu.keypad.keys[9] = true;
    cpu.process_opcode(0xf00a);

    assert_eq!(cpu.v[0], 9);
    assert_eq!(cpu.pc, (0xF00 + 2));
}

#[test]
fn test_op_fx15() {
    let mut cpu = build_cpu();
    cpu.process_opcode(0xf115);

    assert_eq!(cpu.dt, 0);
}

#[test]
fn test_op_fx18() {
    let mut cpu = build_cpu();
    cpu.process_opcode(0xf118);

    assert_eq!(cpu.st, 0);
}

#[test]
fn test_op_fx1e() {
    let mut cpu = build_cpu();
    cpu.process_opcode(0xf51e);

    assert_eq!(cpu.i, 2);
}

#[test]
fn test_op_fx29() {
    let mut cpu = build_cpu();
    cpu.process_opcode(0xf529);

    todo!();
}

#[test]
fn test_op_fx33() {
    let mut cpu = build_cpu();
    cpu.process_opcode(0xf533);

    assert_eq!(cpu.memory[cpu.i as usize], 0);
    assert_eq!(cpu.memory[(cpu.i + 1) as usize], 0);
    assert_eq!(cpu.memory[(cpu.i + 2) as usize], 2);
}

#[test]
fn test_op_fx55() {
    let mut cpu = build_cpu();
    cpu.process_opcode(0xf555);

    assert_eq!(cpu.memory[cpu.i as usize], cpu.v[0]);
    assert_eq!(cpu.memory[(cpu.i + 1) as usize], cpu.v[1]);
    assert_eq!(cpu.memory[(cpu.i + 2) as usize], cpu.v[2]);
    assert_eq!(cpu.memory[(cpu.i + 3) as usize], cpu.v[3]);
    assert_eq!(cpu.memory[(cpu.i + 4) as usize], cpu.v[4]);
    assert_eq!(cpu.memory[(cpu.i + 5) as usize], cpu.v[5]);
}

#[test]
fn test_op_fx65() {
    let mut cpu = build_cpu();
    cpu.memory[cpu.i as usize] = 7;
    cpu.process_opcode(0xf065);

    assert_eq!(cpu.v[0], 7);
}
