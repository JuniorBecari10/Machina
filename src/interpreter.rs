use std::{collections::HashMap, io::{self, Write}};

use crate::{ast::*, util::print_error_reduced};

type LabelMap = HashMap<String, usize>;
type VariableMap = HashMap<String, Value>;
type ScopeStack = Vec<VariableMap>;

macro_rules! try_pop {
  ($operation_stack: expr, $inst: literal, $count: expr) => {
    match $operation_stack.pop() {
      Some(v) => v,
      None => {
        print_error_reduced(&format!("In '{}' instruction: Attempt to pop the operation stack while being empty", $inst), $count); // TODO | diverge from first and second operand
        return Err(());
      }
    }
  };
}

pub fn interpret(ast: &[ReducedAstNode]) -> Result<(), ()> {
  let labels = search_labels(ast);

  let mut operation_stack: Vec<Value> = vec![];
  let mut variables: VariableMap = HashMap::new();
  let mut scopes = vec![];
  
  let mut count: usize = 0;
  
  while count < ast.len() {
    match ast[count].0.clone() {
      AstNodeData::Label(_) => {},
      
      AstNodeData::Pushc(value) => operation_stack.push(value),
      
      AstNodeData::Pushv(var) => match get_var(&variables, &scopes, &var) {
        Some(value) => operation_stack.push(value.clone()),
        None => print_error_reduced(&format!("In 'pushv' instruction: Variable '{}' doesn't exist", var), count),
      },
      
      AstNodeData::Setc(var, value) => { variables.insert(var, value); }, // TODO | check if the variable wasn't present
      AstNodeData::Popv(var) => {
        variables.insert(var, match operation_stack.pop() {
          Some(v) => v,
          None => {
            print_error_reduced("In 'popv' instruction: Attempt to pop the operation stack while being empty", count);
            return Err(());
          }
        });
      },
      
      AstNodeData::Pop => { try_pop!(operation_stack, "pop", count); },
      
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
          print_error_reduced(&format!("In 'div' instruction: Cannot divide {} and {}", a.as_str_debug(), b.as_str_debug()), count);
          return Err(());
        }
      },

      AstNodeData::Inc => {
        let x = try_pop!(operation_stack, "inc", count);

        if let Value::Num(n) = x {
          operation_stack.push(Value::Num(n + 1.0));
        }

        else {
          print_error_reduced(&format!("In 'inc' instruction: Cannot increment {}", x.as_str_debug()), count);
          return Err(());
        }
      }

      AstNodeData::Dec => {
        let x = try_pop!(operation_stack, "dec", count);

        if let Value::Num(n) = x {
          operation_stack.push(Value::Num(n - 1.0));
        }

        else {
          print_error_reduced(&format!("In 'dec' instruction: Cannot decrement {}", x.as_str_debug()), count);
          return Err(());
        }
      }
      
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
      AstNodeData::Println => println!("{}", try_pop!(operation_stack, "println", count).as_str()),
      
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
      
      AstNodeData::Jmp => {
        let label = try_pop!(operation_stack, "jmp", count);

        if let Value::Label(l) = label {
          let index = match labels.get(&l) {
            Some(i) => *i,
            None => {
              print_error_reduced(&format!("In 'jmp' instruction: Label {} doesn't exist", l), count);
              return Err(());
            }
          };

          count = index;
        }
        else {
          print_error_reduced(&format!("In 'jmp' instruction: Cannot jump to {}; must be a label", label.as_str_debug()), count);
          return Err(());
        }
      },
      AstNodeData::Jt => {
        let label = try_pop!(operation_stack, "jt", count);

        if let Value::Label(ref l) = label {
          let index = match labels.get(l) {
            Some(i) => *i,
            None => {
              print_error_reduced(&format!("In 'jt' instruction: Label {} doesn't exist", label.as_str_debug()), count);
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
            print_error_reduced(&format!("In 'jf' instruction: Value {} is not a boolean", v.as_str_debug()), count);
            return Err(());
          }
        }
        else {
          print_error_reduced(&format!("In 'jt' instruction: Cannot jump to {}; must be a label", label.as_str_debug()), count);
          return Err(());
        }
      }
      AstNodeData::Jf => {
        let label = try_pop!(operation_stack, "jf", count);

        if let Value::Label(ref l) = label {
          let index = match labels.get(l) {
            Some(i) => *i,
            None => {
              print_error_reduced(&format!("In 'jf' instruction: Label {} doesn't exist", label.as_str_debug()), count);
              return Err(());
            }
          };

          let v = try_pop!(operation_stack, "jf", count);

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
        else {
          print_error_reduced(&format!("In 'jf' instruction: Cannot jump to {}; must be a label", label.as_str_debug()), count);
          return Err(());
        }
      }

      AstNodeData::Save => {
        scopes.push(variables.clone());
        variables = HashMap::new();
      }

      AstNodeData::Ret => {
        variables = match scopes.pop() {
          Some(s) => s,
          None => {
            print_error_reduced("In 'ret' instruction: Attempt to pop the scope stack while being empty", count);
            return Err(());
          }
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

fn get_var(scope: &VariableMap, stack: &ScopeStack, name: &str) -> Option<Value> {
  match scope.get(name) {
    Some(v) => Some(v.clone()),
    None => {
      for scope in stack.iter().rev() {
        match scope.get(name) {
          Some(v) => return Some(v.clone()),
          None => continue
        }
      }

      None
    }
  }
}

fn input(out: &mut String) {
  io::stdout().flush().unwrap();
  io::stdin().read_line(out).unwrap();
  *out = out.trim().into();
}
