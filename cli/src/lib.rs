use std::fs; // used for file system functionalities
use std::io;
use std::io::Write;
use std::path::Path;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::env;

pub fn parse_args() -> String{
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input
}

pub fn create_file(args: &[&str]) -> (){
    let filename = &args[1];

    let file = fs::File::create(filename);
    match file{
        #[allow(unused_variables)]
        Ok(file) => {},
        #[allow(unused_variables)]
        Err(err) => {println!("something went wrong in creating file {}", filename);}
    };
    println!("File has been created with the name: {}", filename);   
}

pub fn read_file(args: &[&str]) -> (){
    let filename = &args[1];
    let contents = fs::read_to_string(filename);
    match contents {
        Ok(contents)=> print!("{}", contents),
        #[allow(unused_variables)]
        Err(err)=> println!("trouble opening file {}", filename)
    };
}

pub fn write_to_file(args: &[&str]) -> (){
    let filename: &&str = &args[1];
    let rs:bool = Path::new(filename).exists();

    println!("Please include the message you would like to write to {}", filename);
    let mut msg = String::new();
    io::stdin().read_line(&mut msg).unwrap();

    if rs == true{
        let file = fs::OpenOptions::new()
            .append(true)
            .open(filename);
        match file{
            Ok(mut file) => {file.write(msg.as_bytes()).unwrap();},
            #[allow(unused_variables)]
            Err(err) => {println!("unable to write to file {}", filename);}
        };
        
    }
    else {
        let file = fs::File::create(filename);

        match file{
            Ok(mut file) => {file.write(msg.as_bytes()).unwrap();},
            #[allow(unused_variables)]
            Err(err) => {println!("unable to write to file {}", filename);}
        };
    }
    println!("message appended to file");
}

pub fn over_write_content(args: &[&str]) -> (){
    let filename = &args[1];
    let rs:bool = Path::new(filename).exists();

    println!("Please include the message you would like to write to {}", filename);
    let mut msg = String::new();
    io::stdin().read_line(&mut msg).unwrap();

    println!("Please also include the index at which you would like to start writing");
    let num:u64;
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    num = input.trim().parse().unwrap();
    
    if rs == true{
        let file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(filename);
        
        match file{
            Ok(mut file) => {
                file.seek(SeekFrom::Start(num)).unwrap();
                file.write(msg.as_bytes()).expect("failed to write to file");
                println!("message has been written to file {}", filename);
            },
            #[allow(unused_variables)]
            Err(err) => {println!("unable to write to file {}", filename);}
        };
    }
}

pub fn find_all_instances(args: &[&str]) -> (){
    let filename = &args[1];
    let word = &args[2];
    let contents = fs::read_to_string(filename);
    match contents{
        Ok(contents) => {
            let v: Vec<_> = contents.match_indices(word).collect();
            println!("{:?}, {} instances", v, v.len());
        },
        #[allow(unused_variables)]
        Err(err) => {println!("there was some issue with opening the file {}", filename);}
    };
}

pub fn size(args: &[&str]) -> (){
    let filename = &args[1];
    let file = fs::File::open(filename);

    match file{
        Ok(file) => {
            let size = file.metadata().unwrap().len();
            println!("{} is {} bytes", filename, size);
        },
        #[allow(unused_variables)]
        Err(err) => {println!("unable to open the file {}", filename);}
    };
}

pub fn remove_file(args: &[&str]) -> (){
    let filename = &args[1];
    let file = fs::remove_file(filename);
    match file{
        #[allow(unused_variables)]
        Ok(file) => {println!("{} was removed.", filename);},
        #[allow(unused_variables)]
        Err(err) => {println!("file {} could not be removed", filename);}
    };
}

/*
    (1) Read the input file.
    (2) Read the files with file names included in the input file.
    (3) Find word in files.
    (4) Print the word with indexes of files it is present in.
 */
pub fn finder_across_all(args: &[&str]) -> (){
    let filename = &args[1];
    let word = &args[2];
    let contents = fs::read_to_string(filename);
    match contents{
        Ok(contents) => {
            let mut i = 0;
            print!("{}: ", word);
            for file in contents.split_whitespace(){
                let in_contents = fs::read_to_string(file);
                match in_contents{
                    Ok(in_contents) => {
                        let v: Vec<_> = in_contents.match_indices(word).collect();
                        if v.len() > 0 {
                            print!("{} ", i);
                        }
                        i+=1;
                    },
                    #[allow(unused_variables)]
                    Err(err) => {
                        println!("there was some issue with opening an inner file with index {}", i);
                        break;
                    }
                };
            }
        },
        #[allow(unused_variables)]
        Err(err) => {println!("there was some issue with opening the file {}", filename);}
    };
}

pub fn pwd() -> (){
    println!("{:?}", env::current_dir().unwrap());
}

pub fn man_page() -> (){
    println!("R <filename>: reads the contents of a file");
    println!("W <filename>: writes to a file (will create the file if it doesn't exist or append to a file if it does exist)");
    println!("C <filename>: creates a file");
    println!("find <filename> <query>: finds all instances of query word in file");
    println!("size <filename>: finds the size of a file");
    println!("rm <filename>: removes/deletes a file");
    println!("oW <filename>: overwrites contents of a file with user defined location");
    println!("pwd: print working directory");
    println!("man: overwrites contents of a file with user defined location");
}