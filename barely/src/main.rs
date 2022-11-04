// Import
use boiler_plate;
use std::process;

// Main function
fn main() {
    // Get program from input
    let program = boiler_plate::init_1d("barely");

    // Remove invalid characters
    let program = boiler_plate::remove_invalid_chars("]^bghijkmnoptxf~lqs", &program); // TODO add valid characters

    // Check if program has start and end
    if program.contains("]") && program.contains("~") {
        // Run program
        run_program(program);
    } else {
        // Print error
        println!("Error: Program does not contain start or end");
    }
}

// Run program
fn run_program(program: String) {
    let mut memory = [0_u8; 30000]; // Officially unlimited and uninitialized
    let mut memory_pointer = 0_usize;
    let mut program_pointer = program.chars().position(|c| c == '~').unwrap() - 1;
    let mut jump_offset = 0_usize;
    let mut accumulator = 126_u8;
    let mut started = false;

    // Main loop
    loop {
        // Read instruction from program
        let instruction = match program.chars().nth(program_pointer) {
            Some(instruction) => instruction,
            _ => {
                println!("Error: Program pointer out of bounds");
                process::exit(1);
            }
        };

        // Execute instruction
        match (instruction, started) {
            (']', true) => todo!(),
            ('^', true) => todo!(),
            ('b', true) => todo!(),
            ('g', true) => todo!(),
            ('h', true) => todo!(),
            ('i', true) => todo!(),
            ('j', true) => todo!(),
            ('k', true) => todo!(),
            ('m', true) => todo!(),
            ('n', true) => todo!(),
            ('o', true) => todo!(),
            ('p', true) => todo!(),
            ('t', true) => todo!(),
            ('x', true) => todo!(),

            // Start program
            ('~', false) => started = true,

            // No operation
            ('f', true) | ('l', true) | ('q', true) | ('s', true) => {}

            // Found instruction before start
            (_, false) => {
                println!("Error: Program has not started");
                process::exit(1);
            }

            // Invalid instruction
            (_, true) => {
                println!("Error: Invalid instruction");
                process::exit(1);
            }
        }

        // Increment program pointer
        program_pointer -= 1;
    }
}
