use std::fs; // used for file system functionalities
use std::io;
use std::io::Write;
use std::path::Path;

pub fn create_file(args: &[String]) -> (){
    let filename = &args[2];

    fs::File::create(filename)
            .expect("something went wrong with creating the file ");

    println!("File has been created with the name: {}", filename);   
}

pub fn read_file(args: &[String]) -> (){
    let filename = &args[2];
    let contents = fs::read_to_string(filename)
        .expect("something went wrong with opening the file");
    println!("{}", contents);
}

pub fn write_to_file(args: &[String]) -> (){
    let filename: &String = &args[2];
    let rs:bool;
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

pub fn find_all_instances(args: &[String]) -> (){
    let filename = &args[2];
    let word = &args[3];
    let contents = fs::read_to_string(filename)
        .expect("something went wrong with opening the file");
    let v: Vec<_> = contents.match_indices(word).collect();
    println!("{:?}", v);
}

pub fn size(args: &[String]) -> (){
    let filename = &args[2];
    let file = fs::File::open(filename)
        .expect("something went wrong when trying to open the file");
    let size = file.metadata().unwrap().len();

    println!("{} is {} bytes", filename, size);
}

pub fn remove_file(args: &[String]) -> (){
    let filename = &args[2];
    fs::remove_file(filename).expect("something went wrong with deleting the file");
    println!("{} was removed.", filename);
}