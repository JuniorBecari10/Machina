use crate::{ast::{AstNode, AstNodeData}, util::*};

pub fn resolve(ast: &[AstNode]) -> Result<(), ()> {
  let labels: Vec<&str> = search_labels(ast);
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

fn search_labels(ast: &[AstNode]) -> Vec<&str> {
  let mut labels: Vec<&str> = vec![];

  for node in ast {
    if let AstNodeData::Label(name) = &node.data {
      labels.push(name);
    }
  }

  labels
}
