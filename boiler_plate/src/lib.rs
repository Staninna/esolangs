// Import
use std::{env, fs, process};

// Load program from input
pub fn init_1d(language_name: &str) -> String {
    // Get arguments
    let args: Vec<String> = env::args().collect();

    // Get program from input
    let mut program = String::new();
    match &args[1..] {
        // Load program in memory from input
        [flag, input] => {
            // File input
            if flag == "-f" {
                program = match fs::read_to_string(input) {
                    Ok(program) => program,
                    Err(_) => {
                        eprintln!("Error: File not found or could not be read");
                        process::exit(1);
                    }
                };
            }
            // String input
            else if flag == "-s" {
                program = input.to_string();
            }
        }

        // Show help if no input
        _ => {
            println!("Usage: {} [-f file] [-s string] [-h]", language_name);
            process::exit(0);
        }
    }

    program
}

pub fn remove_invalid_chars(valid_chars: &str, program: &str) -> String {
    let mut new_program = String::new();
    for c in program.chars() {
        if valid_chars.contains(c) {
            new_program.push(c);
        }
    }
    new_program
}
