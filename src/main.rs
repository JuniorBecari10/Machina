use std::{env, fs, process::exit};

mod ast;
mod parser;
mod resolver;
mod util;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        eprintln!("Usage: machina assemble/run <file>");
        exit(1);
    }

    let contents = match fs::read_to_string(&args[2]).ok() {
        Some(c) => c,
        None => {
            eprintln!("Couldn't read file '{}'", &args[2]);
            exit(1);
        }
    };

    if contents.is_empty() {
        eprintln!("File is empty; no bytecode was generated.");
        exit(0);
    }
    
    match args[1].as_str() {
        "assemble" => {
            let parser_res = parser::parse(&contents);

            let ast = match parser_res {
                Ok(a) => a,
                Err(_) => exit(1)
            };

            let resolver_res = resolver::resolve(&ast);

            if resolver_res.is_err() {
                exit(1);
            }

            dbg!(&ast);
        }
        "run" => todo!(),
        
        s => eprintln!("Invalid option: '{s}'. Available options: 'assemble', 'run'.")
    }
}
