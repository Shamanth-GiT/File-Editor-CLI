mod lib;

use std::env; // used to extract the arguments from cli
use std::fs; // used for file system functionalities
use std::io::Write; // used for writing to the file

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 5{
        panic!("Not enough arguments. Usage: <operation: R, W, C> <query> <filename> <message>");
    }

    let (io, query, filename, msg) = lib::extract_args(&args);
    println!("({}, {}, {}, {})", io, query, filename, msg);

    let contents;
    if io == "R"{
        contents = fs::read_to_string(filename)
            .expect("something went wrong with opening the file");
        println!("{}", contents);
    }
    if io == "C"{
        fs::File::create(filename)
            .expect("something went wrong with creating the file ");
        println!("File has been created with the name: {}", filename);
    }
    if io == "W" {
        let mut file = fs::File::create(filename)
            .expect("something went wrong with opening the file for writing");
        file.write(msg.as_bytes()).expect("writing failed");
        println!("--{}-- has been written to file with name {}", msg, filename);
    }
}
