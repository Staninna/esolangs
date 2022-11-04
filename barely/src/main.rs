// Import
use boiler_plate;

// Main function
fn main() {
    // Get program from input
    let program = boiler_plate::init_1d("barely");

    // Remove invalid characters
    let program = boiler_plate::remove_invalid_chars("", &program); // TODO add valid characters

    // Run program
    run_program(program);
}

// Run program
fn run_program(program: String) {
    // TODO write logic
    println!("{}", program)
}
