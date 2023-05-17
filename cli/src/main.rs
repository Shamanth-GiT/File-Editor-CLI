use std::process::Command;

fn main(){
    loop {
        let input = lib::parse_args();

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
            "pwd" => lib::pwd(),
            query => {
                let child = Command::new(query)
                    .args(args)
                    .spawn();
                match child{
                    #[allow(unused_must_use)]
                    Ok(mut child) => {child.wait();},
                    #[allow(unused_variables)]
                    Err(err) => {println!("not a valid command, try man");}
                };
            }
        }
    }
}

