// Main function
fn main() {
    // Read program
    let raw_program = read_program();

    // Remove comments
    let program = remove_comments(raw_program);

    // Run program
    run_program(program);
}

// read program from stdin
fn read_program() -> String {
    let mut program_reader = Vec::new();
    println!("Enter your program. Type 'end' to finish.");
    loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim() == "end" {
                    break;
                }
                program_reader.push(input);
            }
            Err(error) => println!("Error: {}", error),
        }
    }
    program_reader.join("")
}

// remove invalid characters
fn remove_comments(raw_program: String) -> String {
    let mut program = String::new();
    for char in raw_program.chars() {
        match char.to_string().as_str() {
            ">" | "<" | "+" | "-" | "." | "," | "[" | "]" | "#" => {
                program.push(char);
            }
            _ => {}
        }
    }

    program
}

// run program
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
                if memory_pointer < memory.len() - 1 {
                    memory_pointer += 1;
                } else {
                    memory_pointer = 0;
                }
            }

            // Move memory pointer to the left
            '<' => {
                if memory_pointer > 0 {
                    memory_pointer -= 1;
                } else {
                    memory_pointer = memory.len() - 1;
                }
            }

            // Increment memory cell
            '+' => memory[memory_pointer] += 1,

            // Decrement memory cell
            '-' => memory[memory_pointer] -= 1,

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
                match std::io::stdin().read_line(&mut input) {
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
