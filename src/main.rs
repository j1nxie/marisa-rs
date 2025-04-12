use std::{env, time::Duration};

use sdl3::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};

mod cpu;
mod display;
mod keypad;

const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (display::WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (display::HEIGHT as u32) * SCALE;

const INSTRUCTIONS_PER_SECOND: u32 = 1000;
const TIMER_FREQUENCY: u32 = 60;
const CYCLES_PER_FRAME: u32 = INSTRUCTIONS_PER_SECOND / TIMER_FREQUENCY;

fn handle_key_event(keycode: Keycode, pressed: bool, cpu: &mut cpu::Cpu) {
    let key = match keycode {
        Keycode::_1 => Some(0x1),
        Keycode::_2 => Some(0x2),
        Keycode::_3 => Some(0x3),
        Keycode::_4 => Some(0xC),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),
        Keycode::Z => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None,
    };

    if let Some(key) = key {
        if pressed {
            cpu.keypad.key_down(key);
        } else {
            cpu.keypad.key_up(key);
        }
    }
}

fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <rom_path>", args[0]);
        std::process::exit(1);
    }

    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("marisa-rs v2025.4.0-alpha", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas();

    let mut cpu = cpu::Cpu::new();
    cpu.reset();

    if let Some(rom_path) = std::env::args().nth(1) {
        let rom_data = std::fs::read(rom_path)?;
        cpu.load(&rom_data);
    }

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        let frame_start = std::time::Instant::now();

        while let Some(event) = event_pump.poll_event() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    println!("Key up: {:?}", key); // Debug print
                    handle_key_event(key, false, &mut cpu);
                }
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    println!("Key down: {:?}", key); // Debug print
                    handle_key_event(key, true, &mut cpu);
                }
                _ => {}
            }
        }

        for _ in 0..CYCLES_PER_FRAME {
            cpu.execute();
        }
        cpu.decrement_timers();

        if cpu.display.draw_flag {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();

            canvas.set_draw_color(Color::RGB(255, 255, 255));
            for y in 0..display::HEIGHT {
                for x in 0..display::WIDTH {
                    if cpu.display.memory[y][x] == 1 {
                        let rect = Rect::new(
                            (x as u32 * SCALE) as i32,
                            (y as u32 * SCALE) as i32,
                            SCALE,
                            SCALE,
                        );
                        canvas.fill_rect(rect)?;
                    }
                }
            }

            canvas.present();
            cpu.display.draw_flag = false;
        }

        let frame_time = frame_start.elapsed();
        if frame_time < Duration::from_millis(16) {
            std::thread::sleep(Duration::from_millis(16) - frame_time);
        }
    }

    Ok(())
}
