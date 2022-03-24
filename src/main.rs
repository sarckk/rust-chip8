extern crate minifb;
extern crate rodio;

use std::process;
use std::env;
use std::io;
use std::io::Write;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashSet;
use std::time::{Duration, Instant};

use minifb::{Key, Window, WindowOptions};
use rodio::{Sink, OutputStream, source::SineWave};

use librchip::vm::VM;
use librchip::display::{DISPLAY_WIDTH, DISPLAY_HEIGHT};

const CPU_CYCLE_RATE: u128 = 600;

const PX_SCALING: usize = 10;  // pixel scaling factor
const ON_PIXEL: u32 = 0x00FFFFFF; // white pixel

#[inline]
fn print_debug_help() {
    println!("USAGE: ");
    println!("  r        - run until next checkpoint");
    println!("  b <addr> - add breakpoint at address <addr>");
    println!("  ni       - execute next instruction");
    println!("  p        - print current state of CHIP-8");
    println!("  help     - print list of commands available");
}


pub fn map_keypress_bits(keys: Vec<Key>) ->  u16 {
    let mut keybits: u16 = 0;

    for key in keys.iter() {
        let exponent = match key {
            Key::Key1 => Some(0x1),
            Key::Key2 => Some(0x2),
            Key::Key3 => Some(0x3),
            Key::Key4 => Some(0xC),
            Key::Q =>    Some(0x4),
            Key::W =>    Some(0x5),
            Key::E =>    Some(0x6),
            Key::R =>    Some(0xD),
            Key::A =>    Some(0x7),
            Key::S =>    Some(0x8),
            Key::D =>    Some(0x9),
            Key::F =>    Some(0xE),
            Key::Z =>    Some(0xA),
            Key::X =>    Some(0x0),
            Key::C =>    Some(0xB),
            Key::V =>    Some(0xF),
            _ => None
        };

        
        if let Some(exp) = exponent {
            keybits = 1 << exp;
            break;
        } 
    }

    keybits 
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        // does not contain path to .ch8 program  
        eprintln!("Error: Missing path to CHIP-8 program to emulate");
        eprintln!("USAGE: cargo run <file_path>");
        process::exit(1);
    }

    let mut debug = false;

    if args.len() == 3 && args[2] == "-d" {
        // enter debug mode
        debug = true;

        println!("Starting program in debug mode...");
        print_debug_help();
    }

    let file_path = &args[1];
    let f = File::open(file_path).expect("Error: File not found");
    let mut reader = BufReader::new(f);
    let mut buf = Vec::new();

    // TODO: add error
    reader.read_to_end(&mut buf).unwrap(); 

    let mut chip = VM::new();
    chip.load_program(&buf);

    let win_width = DISPLAY_WIDTH * PX_SCALING;
    let win_height = DISPLAY_HEIGHT * PX_SCALING;
    let mut buffer: Vec<u32> = vec![0; win_width * win_height];

    let mut window = Window::new (
        "CHIP-8",
        win_width,
        win_height,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // 60fps
    window.limit_update_rate(Some(Duration::from_millis(1000/60)));

    let cur_t = Instant::now();
    let mut finished_cycles = 0;

    let mut redraw = false;

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(SineWave::new(356.0)); // create a beep
    sink.pause();

    // debugging stuff
    let mut breakpoints: HashSet<u16> = HashSet::new();

    // start fetching
    while window.is_open() && !window.is_key_down(Key::Escape) {

        let expected_cycles: u128 = cur_t.elapsed().as_millis() / (1000 / CPU_CYCLE_RATE); 

        for _ in 0..(expected_cycles - finished_cycles) {
            // check if current pc is in breakpoints to pause at
            if breakpoints.contains(&chip.pc) {
                println!("Hit a breakpoint at {:#x}", chip.pc);
                debug = true;
            }

            if debug {
                let stdin = io::stdin(); 
                let input = &mut String::new();

                loop {
                    print!(">> ");
                    let _ = io::stdout().flush();
                    input.clear();
                    let _ = stdin.read_line(input); // blocks
                    let commands: Vec<&str> = input.split_whitespace().collect();

                    if commands.len() == 0 {
                        print_debug_help();
                        continue;
                    }

                    match commands[0] {
                        "r" =>  {
                            // run until we hit a breakpoint, or run until end
                            debug = false;
                            break;
                        }
                        "b" => {
                            let breakpoint: u16;

                            if commands.len() == 1 {
                                eprintln!("No breakpoint specified.");
                                eprintln!("Usage: b <address>");
                            }

                            match u16::from_str_radix(commands[1], 16) {
                                Ok(addr) => {
                                    breakpoint = addr;
                                }
                                Err(_) => {
                                    eprintln!("Badly formatted hex address.");
                                    eprintln!("Enter a valid hex address without leading '0x'");
                                    continue;
                                }
                            }
                            // set a breakpoint at <breakpoint> 
                            println!("Setting break point at {:#x}", breakpoint);
                            breakpoints.insert(breakpoint);
                        }
                        "p" => {
                            // print state of VM
                            println!("{:#?}", chip);
                        }
                        "ni" => {
                            // next instruction
                            println!("{:#x}\topcode={:#x}", chip.pc, chip.memory.get_instr(chip.pc));
                            break;
                        }
                        "help" => {
                            print_debug_help();
                        }
                        _ => { }
                    }
                }
            }

            chip.keys.set_keys(
                map_keypress_bits( window.get_keys() )
            );

            chip.emulate_cycle();
            finished_cycles += 1;

            if chip.redraw {
                redraw = true;
            }
        }

        let beep: bool = chip.decrement_timers();
        if beep {
            sink.play();
        } else {
            sink.pause();
        }

        if redraw {
            for (i, px) in chip.get_display().into_iter().enumerate() {
                // px is u8, either 0x1 or 0x0
                let row = i / DISPLAY_WIDTH;
                let col = i % DISPLAY_WIDTH;

                for row_offset in 0..PX_SCALING  {
                    let buf_idx = row*win_width*PX_SCALING + col*PX_SCALING + row_offset*win_width;
                    buffer[buf_idx..buf_idx+PX_SCALING].fill(if *px == 0 {0} else {ON_PIXEL});
                }
            }

            window
                .update_with_buffer(&buffer, win_width, win_height)
                .unwrap();
            
            redraw = false;
        }
    }
}

