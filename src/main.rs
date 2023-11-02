use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "hello" => { println!("Hello, rust-grep!"); },
            "file" => if args.len() > 2 {
                match args[2].as_str() {
                    "r" => { println!("read file")},
                    "c" => { println!("create file")}
                    _ => { println!("Unknown Command")}
                };
            }else{
                println!("<SECOND ARG> - r to read a file");
                println!("             - c to create a file");
            },
            _ => { println!("Unknown Command...") }
        }
    }else{
        println!("<FIRST ARG>  - hello for a greeting");
        println!("             - file for file manipulation");
    }
    
}
