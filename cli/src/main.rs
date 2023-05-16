use std::io::{self, Write};
use std::process::Command;

fn main(){
    loop {
        print!("sham> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let parts = input.trim().split_whitespace();
        let args: Vec<&str> = parts.collect();
        let query = args[0];

        match query {
            "quit" => return,
            "R" => lib::read_file(&args),
            "W" => lib::write_to_file(&args),
            "C" => lib::create_file(&args),
            "find" => lib::find_all_instances(&args),
            "size" => lib::size(&args),
            "rm" => lib::remove_file(&args),
            "oW" => lib::over_write_content(&args),
            "man" => lib::man_page(),
            "findAcr" => lib::finder_across_all(&args),
            query => {
                let mut child = Command::new(query)
                    .args(args)
                    .spawn()
                    .unwrap();

                child.wait().unwrap();
            }
        }
    }
}

