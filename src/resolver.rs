use crate::{ast::{AstNode, AstNodeData}, util::print_error};

pub fn resolve(ast: &[AstNode]) -> Result<(), ()> {
  let labels = match search_labels(ast) {
    Ok(v) => v,
    Err(_) => return Err(())
  };

  let mut had_error = false;

  for node in ast {
    match node.data.clone() {
      AstNodeData::Jmp(l)
      | AstNodeData::Jt(l)
      | AstNodeData::Jf(l) => {
        if !labels.contains(&l.as_str()) {
          print_error(&format!("Label '{}' doesn't exist", l), &node.code, node.line);
          
          had_error = true;
          continue;
        }
      }

      _ => {}
    }
  }

  if had_error { Err(()) } else { Ok(()) }
}

fn search_labels(ast: &[AstNode]) -> Result<Vec<&str>, ()> {
  let mut labels: Vec<&str> = vec![];
  let mut had_error = false;

  for node in ast {
    if let AstNodeData::Label(name) = &node.data {
      if labels.contains(&name.as_str()) {
        print_error(&format!("Cannot redeclare label '{}'", name), &node.code, node.line);
        had_error = true;
      }

      labels.push(name);
    }
  }

  if had_error { Err(()) } else { Ok(labels) }
}
