// Import
use boiler_plate;
use rand;
use std::process;
use std::{io, thread, time::Duration};

// Main function
fn main() {
    // Get program from input
    let program = boiler_plate::init_1d(
        "spyrodecimal",
        "0.1.0",
        "A esolang that was designed to have only numbers",
    );

    // Remove invalid characters
    let program = boiler_plate::remove_invalid_chars("0123456789qxsrabcdefg", &program);

    // Run program
    run_program(program);
}

// Run program
fn run_program(program: String) {
    // Initialize variables
    let mut program_pointer = 0_usize;
    let mut memory = 0_u8;
    let mut a = 0_u8;
    let mut b = 0_u8;
    let mut c = 0_u8;
    let mut d = 0_u8;
    let mut e = 0_u8;
    let mut f = 0_u8;

    // Main program loop
    loop {
        // Read instruction from program
        let instruction = match program.chars().nth(program_pointer) {
            Some(instruction) => instruction,
            None => break,
        };

        // Execute instruction
        match instruction {
            // Sleep 1/10 second
            '0' => {
                thread::sleep(Duration::from_millis(100));
            }

            // Outputs the ASCII byte in memory
            '1' => {
                // print!("{}", memory as u8 as char);
                println!("{}:{}", memory as char, memory);
            }

            // Increase memory by 1
            '2' => {
                // Prevent overflow
                if memory < 255 {
                    memory += 1;
                } else {
                    eprintln!(
                        "Error: Overflow detected at program instruction {}",
                        program_pointer
                    );
                    process::exit(1);
                }
            }

            // Decrease memory by 1
            '3' => {
                // Prevent underflow
                if memory > 0 {
                    memory -= 1;
                } else {
                    eprintln!(
                        "Error: Underflow detected at program instruction {}",
                        program_pointer
                    );
                    process::exit(1);
                }
            }

            // Get input and store it in memory
            '4' => {
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => {}
                    Err(error) => println!("Error: {}", error),
                }

                memory = input.chars().nth(0).unwrap() as u8;
            }

            // Print new line
            '5' => println!(""),

            // generate random number and store it in memory
            '6' => {
                memory = rand::random::<u8>();
            }

            // Move program pointer back by memory
            '7' => {
                // Prevent underflow
                if program_pointer > memory as usize {
                    program_pointer -= memory as usize;
                } else {
                    eprintln!(
                        "Error: Underflow detected at program instruction {}",
                        program_pointer
                    );
                    process::exit(1);
                }
            }

            // Reset memory
            '8' => memory = 0,

            // Move program pointer forward by memory
            '9' => {
                // Prevent overflow
                if program_pointer + (memory as usize) < program.len() {
                    program_pointer += memory as usize;
                } else {
                    eprintln!(
                        "Error: Overflow detected at program instruction {}",
                        program_pointer
                    );
                    process::exit(1);
                }
            }

            // Quit interpreter
            'q' => process::exit(0),

            // Quit program
            'x' => process::exit(0),

            // Store memory in var in next instruction
            's' => {
                // Ignore when memory is 8
                if memory != 8 {
                    // get variable
                    let var = match program.chars().nth(program_pointer + 1) {
                        Some(var) => match var {
                            'a' => &mut a,
                            'b' => &mut b,
                            'c' => &mut c,
                            'd' => &mut d,
                            'e' => &mut e,
                            'f' => &mut f,
                            _ => {
                                eprintln!("Error: Invalid variable");
                                process::exit(1);
                            }
                        },
                        None => {
                            eprintln!(
                                "Error: No variable specified at program instruction {}",
                                program_pointer
                            );
                            process::exit(1);
                        }
                    };

                    // Store memory in variable
                    *var = memory;
                    program_pointer += 1;
                }
            }

            // Load var from next instruction into memory
            'r' => {
                let var = match program.chars().nth(program_pointer + 1) {
                    Some(var) => match var {
                        'a' => &mut a,
                        'b' => &mut b,
                        'c' => &mut c,
                        'd' => &mut d,
                        'e' => &mut e,
                        'f' => &mut f,
                        _ => {
                            eprintln!("Error: Invalid variable");
                            process::exit(1);
                        }
                    },
                    None => {
                        eprintln!(
                            "Error: No variable specified at program instruction {}",
                            program_pointer
                        );
                        process::exit(1);
                    }
                };

                // Store var in memory
                memory = *var;
                program_pointer += 1;
            }

            // Invalid instruction
            _ => {
                eprintln!("Error: Invalid character");
                process::exit(1);
            }
        }

        // Increment program pointer
        program_pointer += 1;
    }
}
