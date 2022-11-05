// Import
use boiler_plate;
use std::{io, process};

// Main function
fn main() {
    // Get program from input
    let program = boiler_plate::init_1d(
        "brainfuck",
        "0.1.0",
        "A simple esolang using only 8 instructions",
    );

    // Remove invalid characters
    let program = boiler_plate::remove_invalid_chars("><+-.,[]#", &program);

    // Run program
    run_program(program);
}

// Run program
fn run_program(program: String) {
    // Initialize variables
    let mut memory = [0_u8; 30000];
    let mut program_pointer = 0_usize;
    let mut memory_pointer = 0_usize;
    let mut loop_counter = 0_usize;

    // Main program loop
    loop {
        // Read instruction from program
        let instruction = match program.chars().nth(program_pointer) {
            Some(instruction) => instruction,
            None => break,
        };

        // Execute instruction
        match instruction {
            // Move memory pointer to the right
            '>' => {
                // Wrap around if pointer is at the end of the memory
                if memory_pointer < memory.len() - 1 {
                    memory_pointer += 1;
                } else {
                    memory_pointer = 0;
                }
            }

            // Move memory pointer to the left
            '<' => {
                // Warp around memory if pointer is at the start of the memory
                if memory_pointer > 0 {
                    memory_pointer -= 1;
                } else {
                    memory_pointer = memory.len() - 1;
                }
            }

            // Increment memory cell
            '+' => {
                // prevent overflow
                if memory[memory_pointer] < 255 {
                    memory[memory_pointer] += 1;
                } else {
                    eprintln!(
                        "Error: Overflow detected at memory address {} and on program pointer {}",
                        memory_pointer, program_pointer
                    );
                    process::exit(1);
                }
            }

            // Decrement memory cell
            '-' => {
                // Prevent underflow
                if memory[memory_pointer] > 0 {
                    memory[memory_pointer] -= 1;
                } else {
                    eprintln!(
                        "Error: Underflow detected at memory address {} and on program pointer {}",
                        memory_pointer, program_pointer
                    );
                    process::exit(1);
                }
            }

            // Output memory cell
            '.' => {
                // 10 is the code for a new line
                if memory[memory_pointer] == 10 {
                    println!();
                } else {
                    print!("{}", memory[memory_pointer] as char);
                }
            }

            // Get user input
            ',' => {
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => {}
                    Err(error) => println!("Error: {}", error),
                }

                // 10 is the code for a new line
                if input.trim() == "" {
                    memory[memory_pointer] = 10;
                } else {
                    memory[memory_pointer] = input.chars().nth(0).unwrap() as u8;
                }
            }

            // Start loop
            '[' => {
                // If memory cell is 0, skip to the end of the loop
                if memory[memory_pointer] == 0 {
                    loop_counter += 1;
                    loop {
                        program_pointer += 1;
                        match program.chars().nth(program_pointer).unwrap() {
                            '[' => loop_counter += 1,
                            ']' => loop_counter -= 1,
                            _ => {}
                        }
                        if loop_counter == 0 {
                            break;
                        }
                    }
                }
            }

            // End loop
            ']' => {
                // If memory cell is not 0, go back to the start of the loop
                if memory[memory_pointer] != 0 {
                    loop_counter += 1;
                    loop {
                        program_pointer -= 1;
                        match program.chars().nth(program_pointer).unwrap() {
                            '[' => loop_counter -= 1,
                            ']' => loop_counter += 1,
                            _ => {}
                        }
                        if loop_counter == 0 {
                            break;
                        }
                    }
                }
            }

            // Debug memory
            '#' => {
                // Print memory address and value
                println!(
                    "Memory addresses {:}-{:}",
                    memory_pointer,
                    memory_pointer + 16
                );
                for i in memory_pointer..memory_pointer + 16 {
                    print!("{:} ", memory[i]);
                }
                println!();
            }
            _ => panic!("Invalid instruction"),
        }

        // Increment program pointer
        program_pointer += 1;
    }
}
