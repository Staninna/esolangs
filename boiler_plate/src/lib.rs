// Import
use std::{env, fs, io, process};

// Load program from input
pub fn init_1d(language_name: &str, version: &str, description: &str) -> String {
    // Get arguments
    let args: Vec<String> = env::args().collect();

    // Get program from input
    let mut program = String::new();
    match &args[1..] {
        // Load program in memory from input
        [flag, input] => {
            match flag.as_str() {
                // Input from file
                "-f" | "--file" => {
                    program = match fs::read_to_string(input) {
                        Ok(program) => program,
                        Err(_) => {
                            eprintln!("Error: File not found or could not be read");
                            process::exit(1);
                        }
                    };
                }

                // Input from sting
                "-s" | "--string" => program = input.to_string(),

                // Invalid flag
                _ => {
                    eprintln!("Error: Invalid flag");
                    process::exit(1);
                }
            }
        }

        [flag] => match flag.as_str() {
            // Print version
            "-v" | "--version" => {
                println!("{} {}", language_name, version);
                println!("{}", description);
                process::exit(0);
            }

            // Show help
            "-h" | "--help" => {
                println!("{} {}", language_name, version);
                println!("{}", description);
                println!("Usage: {} [flag] [input]", language_name);
                println!("Flags:");
                println!("  -f --file [file]  Load program from file");
                println!("  -s --string [string]  Load program from string");
                println!("  -v --version  Print version");
                println!("  -h --help  Display this help message");
                process::exit(0);
            }

            // Invalid flag
            _ => {
                match flag.as_str() {
                    // Input from file
                    "-f" | "--file" => {
                        eprintln!("Error: No file specified");
                    }

                    // Input from sting
                    "-s" | "--string" => eprintln!("Error: No string specified"),

                    // Invalid flag
                    _ => eprintln!("Error: Invalid flag"),
                }
                process::exit(1);
            }
        },

        // Get program from user
        _ => {
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
