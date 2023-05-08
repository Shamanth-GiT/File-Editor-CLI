use std::fs; // used for file system functionalities
use std::io;
use std::io::Write;
use std::path::Path;

pub fn create_file(args: &[String]) -> (){
    let query = &args[1];
    let filename = &args[2];

    fs::File::create(filename)
            .expect("something went wrong with creating the file ");

    println!("File has been created with the name: {}", filename);   
}

pub fn read_file(args: &[String]) -> String{
    let filename = &args[2];
    let contents = fs::read_to_string(filename)
        .expect("something went wrong with opening the file");
    contents
}

pub fn write_to_file(args: &[String]) -> (){
    let filename: &String = &args[2];
    let mut rs:bool=true;
    rs = Path::new(filename).exists();

    println!("Please include the message you would like to write to {}", filename);
    let mut msg = String::new();
    io::stdin().read_line(&mut msg).unwrap();

    if rs == true{
        let mut file = fs::OpenOptions::new()
        .append(true)
        .open(filename)
        .expect("cannot open file");
        
        file.write(msg.as_bytes()).expect("failed to write to file");
    }
    else {
        let mut file = fs::File::create(filename)
            .expect("something went wrong with opening the file for writing");

        file.write(msg.as_bytes()).expect("writing failed");
    }
    println!("message appended to file");
}