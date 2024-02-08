use crate::{ast::*, util::*};

pub fn parse(input: &str) -> Result<Vec<AstNode>, ()> {
    let lines: Vec<&str> = input.split_terminator('\n').map(|l| l.trim()).collect();
    let mut had_error = false;
    let mut nodes: Vec<AstNode> = vec![];
    
    for (i, line) in lines.into_iter().enumerate() {
        if line.is_empty() {
          continue;
        }

        let tokens: Vec<&str> = line.split(' ').collect();
        
        let first = tokens.first().cloned();
        let args = &tokens[1..];
        
        if let Some(inst) = first {
            match inst {
                s if s.starts_with('#') => {
                    if !is_label(s) {
                        print_error(&format!("Label identifier '{}' is not valid", args[0]), line, i);

                        had_error = true;
                        break;
                    }

                    nodes.push(AstNode::new(AstNodeData::Label(s.into()), line.into(), i));
                }

                "pushc" => {
                    if args.len() != 1 {
                        print_error(&format!("'pushc' instruction requires 1 argument, got {}", args.len()), line, i);
                        
                        had_error = true;
                        break;
                    }

                    let value = match parse_value(args[0]) {
                        Some(v) => v,
                        None => {
                            print_error(&format!("Couldn't parse value '{}'", args[0]), line, i);
                            
                            had_error = true;
                            break
                        }
                    };

                    nodes.push(AstNode::new(AstNodeData::Pushc(value), line.into(), i));
                }

                "pushv" => {
                    if args.len() != 1 {
                        print_error(&format!("'pushv' instruction requires 1 argument, got {}", args.len()), line, i);
                        
                        had_error = true;
                        break;
                    }

                    if !is_identifier(args[0]) {
                        print_error(&format!("Identifier '{}' is not valid (valid identifiers only contain letters, numbers and underscores; the first character must not be a number)", args[0]), line, i);

                        had_error = true;
                        break;
                    }

                    nodes.push(AstNode::new(AstNodeData::Pushv(args[0].into()), line.into(), i));
                }

                "setc" => {
                    if args.len() != 1 {
                        print_error(&format!("'setc' instruction requires 1 argument, got {}", args.len()), line, i);
                        
                        had_error = true;
                        break;
                    }

                    let value = match parse_value(args[0]) {
                        Some(v) => v,
                        None => {
                            print_error(&format!("Couldn't parse value '{}'", args[0]), line, i);
                            
                            had_error = true;
                            break
                        }
                    };

                    nodes.push(AstNode::new(AstNodeData::Setc(value), line.into(), i));
                }

                "pop" => {
                    if args.len() != 1 {
                        print_error(&format!("'pop' instruction requires 1 argument, got {}", args.len()), line, i);
                        
                        had_error = true;
                        break;
                    }

                    if !is_identifier(args[0]) {
                        print_error(&format!("Identifier '{}' is not valid", args[0]), line, i);

                        had_error = true;
                        break;
                    }

                    nodes.push(AstNode::new(AstNodeData::Pop(args[0].into()), line.into(), i));
                }

                "add" => nodes.push(AstNode::new(AstNodeData::Add, line.into(), i)),
                "sub" => nodes.push(AstNode::new(AstNodeData::Sub, line.into(), i)),
                "mul" => nodes.push(AstNode::new(AstNodeData::Mul, line.into(), i)),
                "div" => nodes.push(AstNode::new(AstNodeData::Div, line.into(), i)),

                "inputn" => nodes.push(AstNode::new(AstNodeData::Inputn, line.into(), i)),
                "inputb" => nodes.push(AstNode::new(AstNodeData::Inputb, line.into(), i)),
                "inputs" => nodes.push(AstNode::new(AstNodeData::Inputs, line.into(), i)),

                "print" => nodes.push(AstNode::new(AstNodeData::Print, line.into(), i)),
                "println" => nodes.push(AstNode::new(AstNodeData::Println, line.into(), i)),

                "cmpg" => nodes.push(AstNode::new(AstNodeData::Cmpg, line.into(), i)),
                "cmpge" => nodes.push(AstNode::new(AstNodeData::Cmpge, line.into(), i)),

                "cmpl" => nodes.push(AstNode::new(AstNodeData::Cmpl, line.into(), i)),
                "cmple" => nodes.push(AstNode::new(AstNodeData::Cmple, line.into(), i)),

                "cmpe" => nodes.push(AstNode::new(AstNodeData::Cmpe, line.into(), i)),
                "cmpne" => nodes.push(AstNode::new(AstNodeData::Cmpne, line.into(), i)),

                "jmp" => {
                    if args.len() != 1 {
                        print_error(&format!("'jmp' instruction requires 1 argument, got {}", args.len()), line, i);
                        
                        had_error = true;
                        break;
                    }

                    if !is_label(args[0]) {
                        print_error(&format!("Label identifier '{}' is not valid", args[0]), line, i);

                        had_error = true;
                        break;
                    }

                    nodes.push(AstNode::new(AstNodeData::Jmp(args[0].into()), line.into(), i));
                }

                "jt" => {
                    if args.len() != 1 {
                        print_error(&format!("'jt' instruction requires 1 argument, got {}", args.len()), line, i);
                        
                        had_error = true;
                        break;
                    }

                    if !is_label(args[0]) {
                        print_error(&format!("Label identifier '{}' is not valid", args[0]), line, i);

                        had_error = true;
                        break;
                    }

                    nodes.push(AstNode::new(AstNodeData::Jt(args[0].into()), line.into(), i));
                }

                "jf" => {
                    if args.len() != 1 {
                        print_error(&format!("'jf' instruction requires 1 argument, got {}", args.len()), line, i);
                        
                        had_error = true;
                        break;
                    }

                    if !is_label(args[0]) {
                        print_error(&format!("Label identifier '{}' is not valid", args[0]), line, i);

                        had_error = true;
                        break;
                    }

                    nodes.push(AstNode::new(AstNodeData::Jf(args[0].into()), line.into(), i));
                }
                    
                
                _ => {
                    print_error(&format!("Invalid instruction: '{inst}'"), line, i);
                    had_error = true;
                }
            }    
        }
    }
    
    if had_error { Err(()) } else { Ok(nodes) }
}
