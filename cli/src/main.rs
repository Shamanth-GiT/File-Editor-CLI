mod lib;

use std::env; // used to extract the arguments from cli

fn main() {
    // collects the arguments as a vector of strings.
    let args: Vec<String> = env::args().collect();

    let query = &args[1];

    if query == "R"{
        let contents = lib::read_file(&args);
        println!("{}", contents);
    }
    if query == "C"{
        lib::create_file(&args);
    }
    if query == "W" {
        lib::write_to_file(&args);
    }
}
