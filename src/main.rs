use std::{env, fs, process::exit};

mod ast;

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
    
    match args[1].as_str() {
        "assemble" => {
            let ast = parse(&contents);
            println!("{}", args[2]);
            dbg!(ast);
            
        }
        "run" => todo!(),
        
        s => eprintln!("Invalid option: '{s}'. Available options: 'assemble', 'run'.")
    }
}

fn parse(input: &str) -> Vec<ast::AstNode> {
    let lines: Vec<&str> = input.split('\n').collect();
    let mut nodes: Vec<ast::AstNode> = vec![];
    
    for (i, line) in lines.into_iter().enumerate() {
        let tokens: Vec<&str> = line.split(' ').collect();
        
        let first = tokens.first().cloned();
        let args = &tokens[1..];
        
        if let Some(inst) = first {
            match inst {
                "pushc" => {
                    if args.len() != 1 {
                        print_error(&format!("'pushc' instruction requires 1 argument, got {}", args.len()), line, i);
                        break;
                    }

                    let value = match ast::parse_value(args[0]) {
                        Some(v) => v,
                        None => {
                            print_error(&format!("Couldn't parse value '{}'", args[0]), line, i);
                            break
                        }
                    };

                    nodes.push(ast::AstNode::Pushc(value));
                }
                
                _ => print_error(&format!("Invalid instruction: '{inst}'"), line, i)
            }    
        }
    }
    
    nodes
}

fn print_error(msg: &str, code: &str, mut line: usize) {
    line += 1;

    eprintln!("Error in line {line}: {msg}");
    eprintln!("{line} | {code}");
}
