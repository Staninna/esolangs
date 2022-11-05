// Import
use std::{env, fs, io, process};

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
            match flag.as_str() {
                "-f" => {
                    program = match fs::read_to_string(input) {
                        Ok(program) => program,
                        Err(_) => {
                            eprintln!("Error: File not found or could not be read");
                            process::exit(1);
                        }
                    };
                }
                "-s" => program = input.to_string(),
                "-h" => {
                    println!("Usage: {} [flag] [input]", language_name);
                    println!("Flags:");
                    println!("    -f: File input");
                    println!("    -s: String input");
                    println!("    -h: Help");
                    process::exit(0);
                }
                _ => {
                    eprintln!("Error: Invalid flag");
                    process::exit(1);
                }
            }
        }

        // Get user input
        _ => {
            // Get program from user
            let mut program = String::new();
            println!("Input program input `Exit` to exit: ");

            // Main input loop
            loop {
                // Get input
                let mut new_line = String::new();
                io::stdin()
                    .read_line(&mut new_line)
                    .expect("Failed to read line");

                // Check if input is exit
                if new_line.to_lowercase().trim() == "exit" {
                    break;
                }

                // Add new line to program
                program.push_str(&new_line);
            }
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
