use crate::{ast::{AstNode, AstNodeData, ReducedAstNode, Value}, util::{is_identifier, is_label, parse_value, print_error, print_error_reduced}};

macro_rules! parse_string {
    ($bytes: expr, $count: expr, $inst: literal) => {
        match parse_string($bytes, $count) {
            Some(n) => n,
            None => {
                print_error_reduced(&format!("While parsing '{}' instruction: Bytecode size isn't long enough to properly parse a string", $inst));
                return Err(());
            }
        }
    }
}

macro_rules! parse_value {
    ($bytes: expr, $count: expr, $inst: literal) => {
        match parse_value_reduced($bytes, $count) {
            Some(n) => n,
            None => {
                print_error_reduced(&format!("While parsing '{}' instruction: Bytecode size isn't long enough to properly parse a value", $inst));
                return Err(());
            }
        }
    }
}

pub fn parse(input: &str) -> Result<Vec<AstNode>, ()> {
    let mut had_error = false;
    let mut nodes: Vec<AstNode> = vec![];
    
    for (i, line) in input.lines().into_iter().enumerate() {
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
                    if args.len() != 2 {
                        print_error(&format!("'setc' instruction requires 2 arguments, got {}", args.len()), line, i);
                        
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

                    if !is_identifier(args[1]) {
                        print_error(&format!("Identifier '{}' is not valid (valid identifiers only contain letters, numbers and underscores; the first character must not be a number)", args[0]), line, i);

                        had_error = true;
                        break;
                    }

                    nodes.push(AstNode::new(AstNodeData::Setc(value, args[1].into()), line.into(), i));
                }

                "popv" => {
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

                    nodes.push(AstNode::new(AstNodeData::Popv(args[0].into()), line.into(), i));
                }

                "pop" => nodes.push(AstNode::new(AstNodeData::Pop, line.into(), i)),

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

                "save" => nodes.push(AstNode::new(AstNodeData::Save, line.into(), i)),
                "ret" => nodes.push(AstNode::new(AstNodeData::Ret, line.into(), i)),

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

pub fn parse_reduced(input: &str) -> Result<Vec<ReducedAstNode>, ()> {
    let mut nodes = vec![];
    let bytes: Vec<u8> = input.bytes().collect();

    let mut count: usize = 0;
    while count < bytes.len() {
        let inst = bytes[count];
        count += 1;

        match inst {
            0 => nodes.push(ReducedAstNode(AstNodeData::Label(parse_string!(&bytes, &mut count, "label")))),
            1 => nodes.push(ReducedAstNode(AstNodeData::Pushc(parse_value!(&bytes, &mut count, "pushc")))),
            2 => nodes.push(ReducedAstNode(AstNodeData::Pushv(parse_string!(&bytes, &mut count, "label")))),

            3 => { // Setc
                let value = parse_value!(&bytes, &mut count, "setc");
                let name = parse_string!(&bytes, &mut count, "setc");

                nodes.push(ReducedAstNode(AstNodeData::Setc(value, name)));
            }

            4 => nodes.push(ReducedAstNode(AstNodeData::Popv(parse_string!(&bytes, &mut count, "popv")))),

            5 => nodes.push(ReducedAstNode(AstNodeData::Pop)),

            6 => nodes.push(ReducedAstNode(AstNodeData::Add)),
            7 => nodes.push(ReducedAstNode(AstNodeData::Sub)),
            8 => nodes.push(ReducedAstNode(AstNodeData::Mul)),
            9 => nodes.push(ReducedAstNode(AstNodeData::Div)),

            10 => nodes.push(ReducedAstNode(AstNodeData::Inputn)),
            11 => nodes.push(ReducedAstNode(AstNodeData::Inputb)),
            12 => nodes.push(ReducedAstNode(AstNodeData::Inputs)),

            13 => nodes.push(ReducedAstNode(AstNodeData::Print)),
            14 => nodes.push(ReducedAstNode(AstNodeData::Println)),

            15 => nodes.push(ReducedAstNode(AstNodeData::Cmpg)),
            16 => nodes.push(ReducedAstNode(AstNodeData::Cmpge)),

            17 => nodes.push(ReducedAstNode(AstNodeData::Cmpl)),
            18 => nodes.push(ReducedAstNode(AstNodeData::Cmple)),

            19 => nodes.push(ReducedAstNode(AstNodeData::Cmpe)),
            20 => nodes.push(ReducedAstNode(AstNodeData::Cmpne)),

            21 => nodes.push(ReducedAstNode(AstNodeData::Save)),
            22 => nodes.push(ReducedAstNode(AstNodeData::Ret)),

            23 => nodes.push(ReducedAstNode(AstNodeData::Jmp(parse_string!(&bytes, &mut count, "jmp")))),
            24 => nodes.push(ReducedAstNode(AstNodeData::Jt(parse_string!(&bytes, &mut count, "jt")))),
            25 => nodes.push(ReducedAstNode(AstNodeData::Jf(parse_string!(&bytes, &mut count, "jf")))),

            _ => {
                print_error_reduced(&format!("Invalid instruction code: {}", inst));
                return Err(());
            }
        }
    }
    
    Ok(nodes)
}

fn parse_string(slice: &[u8], count: &mut usize) -> Option<String> {
    let mut c = *count;

    if slice.len() - c >= 4 {
        let len_bytes: [u8; 4] = slice[c..(c + 4)].try_into().unwrap();
        let len = u32::from_ne_bytes(len_bytes) as usize;

        c += 4;
        *count += 4;

        if slice.len() - c >= len {
            let data = String::from_utf8_lossy(&slice[c..(c + len)]).into();

            *count += len;
            Some(data)
        }
        else {
            None
        }
    }
    else {
        None
    }
}

fn parse_value_reduced(slice: &[u8], count: &mut usize) -> Option<Value> {
    let mut c = *count;
    let kind = slice[c];

    c += 1;
    *count += 1;

    match kind {
        0 => { // Num
            if slice.len() - c >= 8 {
                let bytes: [u8; 8] = slice[c..(c + 8)].try_into().unwrap();
                let num = f64::from_ne_bytes(bytes);

                *count += 8;
                Some(Value::Num(num))
            }
            else {
                None
            }
        }

        1 => { // Str
            let s = parse_string(slice, count)?;
            Some(Value::Str(s))
        }

        2 => { // Bool
            if slice.len() >= 2 {
                let value = slice[c] != 0;

                *count += 1;
                Some(Value::Bool(value))
            }
            else {
                None
            }
        }

        _ => None
    }
}
