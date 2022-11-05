// Import
use std::{io, process};

// Main function
fn main() {
    // Get program from input
    let program = boiler_plate::init_1d(
        "barely",
        "0.1.0",
        "A esolang that its goal is to be the smallest interpreter binary as possible",
    );

    // Remove invalid characters
    let program = boiler_plate::remove_invalid_chars("]^bghijkmnoptxf~lqs", &program);

    // Run program
    run_program(program);
}

// Run program
fn run_program(program: String) {
    let mut memory = [0_u16; 30000]; // Officially unlimited and uninitialized
    let mut memory_pointer = 0_usize;
    let mut instruction_pointer = program.chars().position(|c| c == '~').unwrap();
    let mut jump_offset = 0_usize;
    let mut accumulator = 126_u16;
    let mut started = false;

    // Main loop
    loop {
        // Read instruction from program
        let instruction = match program.chars().nth(instruction_pointer) {
            Some(instruction) => instruction,
            _ => {
                println!("Error: Program pointer out of bounds");
                process::exit(1);
            }
        };

        // Execute instruction
        match started {
            true => match instruction {
                // Exit program
                ']' => process::exit(0),

                // If accumulator is 0, execute b instruction
                '[' => {
                    if accumulator == 0 {
                        instruction_pointer += jump_offset;
                    }
                }

                // Add jump to ip
                'b' => instruction_pointer += jump_offset,

                // Accumulator is memory
                'g' => memory[memory_pointer] = accumulator,

                'h' => accumulator += 71,

                // Increment memory pointer
                'i' => memory_pointer += 1,

                // Increment accumulator
                'j' => accumulator += 1,

                // Decrement jump
                'k' => jump_offset -= 1,

                // Memory is accumulator
                'm' => accumulator = memory[memory_pointer],

                // Decrement memory pointer
                'n' => memory_pointer -= 1,

                // Decrement accumulator
                'o' => accumulator -= 1,

                // Add 10 to jump
                'p' => jump_offset += 10,

                // Store user input in accumulator
                't' => {
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                    accumulator = input.chars().next().unwrap() as u16;
                }

                // Print accumulator
                'x' => print!("{}", accumulator as u8 as char),

                // No operation
                'f' | 'l' | 'q' | 's' => {}

                // Invalid instruction
                _ => {
                    println!("Error: Invalid instruction");
                    process::exit(1);
                }
            },
            false => {
                match instruction {
                    // Start program
                    '~' => started = true,

                    // Found instruction before start
                    _ => {
                        println!(
                            "Error: Program has not started or found instruction before start"
                        );
                        process::exit(1);
                    }
                }
            }
        }

        // Increment program pointer
        instruction_pointer -= 1;
    }
}
