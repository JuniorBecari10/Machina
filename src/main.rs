use std::{env, fs, process::exit};

mod ast;
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
    
    match args[1].as_str() {
        "assemble" => {
            let ast = parse(&contents).unwrap();
            println!("{}", args[2]);
            dbg!(ast);
            
        }
        "run" => todo!(),
        
        s => eprintln!("Invalid option: '{s}'. Available options: 'assemble', 'run'.")
    }
}

fn parse(input: &str) -> Result<Vec<ast::AstNode>, ()> {
    let lines: Vec<&str> = input.split_terminator('\n').map(|l| l.trim()).collect();
    let mut had_error = false;
    let mut nodes: Vec<ast::AstNode> = vec![];
    
    for (i, line) in lines.into_iter().enumerate() {
        let tokens: Vec<&str> = line.split(' ').collect();
        
        let first = tokens.first().cloned();
        let args = &tokens[1..];
        
        if let Some(inst) = first {
            match inst {
                s if s.starts_with('#') => nodes.push(ast::AstNode::Label(s.into())),

                "pushc" => {
                    if args.len() != 1 {
                        print_error(&format!("'pushc' instruction requires 1 argument, got {}", args.len()), line, i);
                        
                        had_error = true;
                        break;
                    }

                    let value = match util::parse_value(args[0]) {
                        Some(v) => v,
                        None => {
                            print_error(&format!("Couldn't parse value '{}'", args[0]), line, i);
                            
                            had_error = true;
                            break
                        }
                    };

                    nodes.push(ast::AstNode::Pushc(value));
                }

                "pushv" => {
                    if args.len() != 1 {
                        print_error(&format!("'pushv' instruction requires 1 argument, got {}", args.len()), line, i);
                        
                        had_error = true;
                        break;
                    }

                    if !util::is_identifier(args[0]) {
                        print_error(&format!("Identifier '{}' is not valid (valid identifiers only contain letters, numbers and underscores; the first character must not be a number)", args[0]), line, i);

                        had_error = true;
                        break;
                    }

                    nodes.push(ast::AstNode::Pushv(args[0].into()));
                }

                "setc" => {
                    if args.len() != 1 {
                        print_error(&format!("'setc' instruction requires 1 argument, got {}", args.len()), line, i);
                        
                        had_error = true;
                        break;
                    }

                    let value = match util::parse_value(args[0]) {
                        Some(v) => v,
                        None => {
                            print_error(&format!("Couldn't parse value '{}'", args[0]), line, i);
                            
                            had_error = true;
                            break
                        }
                    };

                    nodes.push(ast::AstNode::Setc(value));
                }

                "pop" => {
                    if args.len() != 1 {
                        print_error(&format!("'pushc' instruction requires 1 argument, got {}", args.len()), line, i);
                        
                        had_error = true;
                        break;
                    }

                    if !util::is_identifier(args[0]) {
                        print_error(&format!("Identifier '{}' is not valid (valid identifiers only contain letters, numbers and underscores; the first character must not be a number)", args[0]), line, i);

                        had_error = true;
                        break;
                    }

                    nodes.push(ast::AstNode::Pop(args[0].into()));
                }
                
                _ => {
                    print_error(&format!("Invalid instruction: '{inst}'"), line, i);
                    had_error = true;
                }
            }    
        }
    }
    
    if !had_error { Ok(nodes) } else { Err(()) }
}

fn print_error(msg: &str, code: &str, mut line: usize) {
    line += 1;

    eprintln!("Error in line {line}: {msg}");
    eprintln!("{line} | {code}");
}
