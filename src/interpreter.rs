use std::{collections::HashMap, io::{self, Write}};

use crate::{ast::*, util::print_error_reduced};

type LabelMap = HashMap<String, usize>;
type VariableMap = HashMap<String, Value>;

macro_rules! try_pop {
  ($operation_stack: expr, $inst: literal, $count: expr) => {
    match $operation_stack.pop() {
      Some(v) => v,
      None => {
        print_error_reduced(&format!("In 'add' instruction: Attempt to pop the operation stack while being empty"), $count); // TODO | diverge from first and second operand
        return Err(());
      }
    }
  };
}

pub fn interpret(ast: &[ReducedAstNode]) -> Result<(), ()> {
  let labels = search_labels(ast);

  let mut operation_stack: Vec<Value> = vec![];
  let mut variables: VariableMap = HashMap::new();
  
  let mut count: usize = 0;
  while count < ast.len() {
    match ast[count].0.clone() {
      AstNodeData::Label(_) => {},
      
      AstNodeData::Pushc(value) => operation_stack.push(value),
      
      AstNodeData::Pushv(var) => match variables.get(&var) {
        Some(value) => operation_stack.push(value.clone()),
        None => print_error_reduced(&format!("In 'pushv' instruction: Variable '{}' doesn't exist", var), count),
      },
      
      AstNodeData::Setc(value, var) => { variables.insert(var, value); }, // TODO | check if the variable wasn't present
      AstNodeData::Popv(var) => {
        variables.insert(var, match operation_stack.pop() {
          Some(v) => v,
          None => {
            print_error_reduced("In 'popv' instruction: Attempt to pop the operation stack while being empty", count);
            return Err(());
          }
        });
      },
      
      AstNodeData::Pop => { operation_stack.pop(); },
      
      AstNodeData::Add => {
        let a = try_pop!(operation_stack, "add", count);
        let b = try_pop!(operation_stack, "add", count);
        
        if let Value::Num(a) = a {
          if let Value::Num(b) = b {
            operation_stack.push(Value::Num(a + b));

            count += 1;
            continue;
          }
        }
        
        else if let Value::Str(a) = a.clone() {
          if let Value::Str(b) = b {
            operation_stack.push(Value::Str(format!("{}{}", a, b)));

            count += 1;
            continue;
          }
        }
        
        print_error_reduced(&format!("In 'add' instruction: Cannot add {} and {}", a.as_str_debug(), b.as_str_debug()), count);
        return Err(());
      },
      AstNodeData::Sub => {
        let a = try_pop!(operation_stack, "sub", count);
        let b = try_pop!(operation_stack, "sub", count);
        
        if let Value::Num(a) = a {
          if let Value::Num(b) = b {
            operation_stack.push(Value::Num(a - b));

            count += 1;
            continue;
          }
        }
        
        print_error_reduced(&format!("In 'sub' instruction: Cannot subtract {} and {}", a.as_str_debug(), b.as_str_debug()), count);
        return Err(());
      },
      AstNodeData::Mul => {
        let a = try_pop!(operation_stack, "mul", count);
        let b = try_pop!(operation_stack, "mul", count);
        
        if let Value::Num(a) = a {
          if let Value::Num(b) = b {
            operation_stack.push(Value::Num(a * b));
          }
        }
        
        else {
          print_error_reduced(&format!("In 'sub' instruction: Cannot multiply {} and {}", a.as_str_debug(), b.as_str_debug()), count);
          return Err(());
        }
      },
      AstNodeData::Div => {
        let a = try_pop!(operation_stack, "div", count);
        let b = try_pop!(operation_stack, "div", count);
        
        if let Value::Num(a) = a {
          if let Value::Num(b) = b {
            if b == 0.0 {
              print_error_reduced("In 'div' instruction: Cannot divide by zero", count);
              return Err(());
            }
            
            operation_stack.push(Value::Num(a / b));
          }
        }
        
        else {
          print_error_reduced(&format!("In 'sub' instruction: Cannot divide {} and {}", a.as_str_debug(), b.as_str_debug()), count);
          return Err(());
        }
      },
      
      AstNodeData::Inputn => {
        let mut s = String::new();
        input(&mut s);
        
        match s.parse::<f64>() {
          Ok(n) => operation_stack.push(Value::Num(n)),
          Err(_) => {
            print_error_reduced(&format!("In 'inputn' instruction: Cannot parse '{}' as a number", s), count);
            return Err(());
          }
        }
      },
      AstNodeData::Inputb => {
        let mut s = String::new();
        input(&mut s);

        if s == "true" {
          operation_stack.push(Value::Bool(true));
        }
        
        else if s == "false" {
          operation_stack.push(Value::Bool(false));
        }
        
        else {
          print_error_reduced(&format!("In 'inputb' instruction: Cannot parse '{}' as a boolean (Type only accepts either 'true' of 'false')", s), count);
          return Err(());
        }
      }
      AstNodeData::Inputs => {
        let mut s = String::new();
        input(&mut s);

        operation_stack.push(Value::Str(s));
      }
      
      AstNodeData::Print => print!("{}", try_pop!(operation_stack, "print", count).as_str()),
      AstNodeData::Println => println!("{}", try_pop!(operation_stack, "print", count).as_str()),
      
      AstNodeData::Cmpg => {
        let a = try_pop!(operation_stack, "cmpg", count);
        let b = try_pop!(operation_stack, "cmpg", count);
        
        if let Value::Num(a) = a {
          if let Value::Num(b) = b {
            operation_stack.push(Value::Bool(a > b));
          }
        }
        
        else {
          print_error_reduced(&format!("In 'cmpg' instruction: Cannot compare {} and {} as greater", a.as_str_debug(), b.as_str_debug()), count);
          return Err(());
        }
      }
      AstNodeData::Cmpge => {
        let a = try_pop!(operation_stack, "cmpge", count);
        let b = try_pop!(operation_stack, "cmpge", count);
        
        if let Value::Num(a) = a {
          if let Value::Num(b) = b {
            operation_stack.push(Value::Bool(a >= b));
          }
        }
        
        else {
          print_error_reduced(&format!("In 'cmpg' instruction: Cannot compare {} and {} as greater or equal", a.as_str_debug(), b.as_str_debug()), count);
          return Err(());
        }
      }
      
      AstNodeData::Cmpl => {
        let a = try_pop!(operation_stack, "cmpl", count);
        let b = try_pop!(operation_stack, "cmpl", count);
        
        if let Value::Num(a) = a {
          if let Value::Num(b) = b {
            operation_stack.push(Value::Bool(a < b));
          }
        }
        
        else {
          print_error_reduced(&format!("In 'cmpg' instruction: Cannot compare {} and {} as less", a.as_str_debug(), b.as_str_debug()), count);
          return Err(());
        }
      }
      AstNodeData::Cmple => {
        let a = try_pop!(operation_stack, "cmple", count);
        let b = try_pop!(operation_stack, "cmple", count);
        
        if let Value::Num(a) = a {
          if let Value::Num(b) = b {
            operation_stack.push(Value::Bool(a <= b));
          }
        }
        
        else {
          print_error_reduced(&format!("In 'cmpg' instruction: Cannot compare {} and {} as less or equal", a.as_str_debug(), b.as_str_debug()), count);
          return Err(());
        }
      },
      
      AstNodeData::Cmpe => {
        let a = try_pop!(operation_stack, "cmpe", count);
        let b = try_pop!(operation_stack, "cmpe", count);
        
        if let Value::Num(a) = a {
          if let Value::Num(b) = b {
            operation_stack.push(Value::Bool(a == b));
          }
        }

        else if let Value::Str(a) = a.clone() {
          if let Value::Str(b) = b {
            operation_stack.push(Value::Bool(a == b));

            count += 1;
            continue;
          }
        }
        
        else {
          print_error_reduced(&format!("In 'cmpe' instruction: Cannot compare {} and {} as equal (they must be of the same type)", a.as_str_debug(), b.as_str_debug()), count);
          return Err(());
        }
      },
      AstNodeData::Cmpne => {
        let a = try_pop!(operation_stack, "cmpne", count);
        let b = try_pop!(operation_stack, "cmpne", count);
        
        if let Value::Num(a) = a {
          if let Value::Num(b) = b {
            operation_stack.push(Value::Bool(a != b));
          }
        }

        else if let Value::Str(a) = a.clone() {
          if let Value::Str(b) = b {
            operation_stack.push(Value::Bool(a != b));

            count += 1;
            continue;
          }
        }
        
        else {
          print_error_reduced(&format!("In 'cmpe' instruction: Cannot compare {} and {} as not equal (they must be of the same type)", a.as_str_debug(), b.as_str_debug()), count);
          return Err(());
        }
      },
      
      AstNodeData::Jmp(label) => {
        let index = match labels.get(&label) {
          Some(i) => *i,
          None => {
            print_error_reduced(&format!("In 'jmp' instruction: Label {} doesn't exist", label), count);
            return Err(());
          }
        };

        count = index;
      },
      AstNodeData::Jt(label) => {
        let index = match labels.get(&label) {
          Some(i) => *i,
          None => {
            print_error_reduced(&format!("In 'jt' instruction: Label {} doesn't exist", label), count);
            return Err(());
          }
        };

        let v = try_pop!(operation_stack, "jt", count);

        if let Value::Bool(b) = v {
          if b {
            count = index;
          }
        }

        else {
          print_error_reduced(&format!("In 'jt' instruction: Value {} is not a boolean", v.as_str_debug()), count);
            return Err(());
        }
      }
      AstNodeData::Jf(label) => {
        let index = match labels.get(&label) {
          Some(i) => *i,
          None => {
            print_error_reduced(&format!("In 'jf' instruction: Label {} doesn't exist", label), count);
            return Err(());
          }
        };

        let v = try_pop!(operation_stack, "jt", count);

        if let Value::Bool(b) = v {
          if !b {
            count = index;
          }
        }

        else {
          print_error_reduced(&format!("In 'jf' instruction: Value {} is not a boolean", v.as_str_debug()), count);
          return Err(());
        }
      }
    }
    
    count += 1;
  }
  
  Ok(())
}

fn search_labels(ast: &[ReducedAstNode]) -> LabelMap {
  let mut map = HashMap::new();
  
  for (i, n) in ast.iter().enumerate() {
    if let AstNodeData::Label(name) = n.0.clone() {
      map.insert(name, i);
    }
  }
  
  map
}

fn input(out: &mut String) {
  io::stdout().flush().unwrap();
  io::stdin().read_line(out).unwrap();
  *out = out.trim().into();
}
