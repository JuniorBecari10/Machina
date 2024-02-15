use std::{fs::File, io::{Error, Write}};

use crate::ast::{AstNode, AstNodeData};

pub fn compile(ast: &[AstNode], path: &str) -> Result<(), Error> {
  let mut file = File::create(path)?;
  let mut output: Vec<u8> = vec![];

  for node in ast {
    let n = &node.data;
    output.push(n.discriminant());

    match n {
        AstNodeData::Pushc(val) => output.extend_from_slice(&val.encode()),
        AstNodeData::Setc(val, var) => {
          output.extend_from_slice(&val.encode());
          encode_string(&mut output, var);
        },

        AstNodeData::Add
        | AstNodeData::Sub
        | AstNodeData::Mul
        | AstNodeData::Div

        | AstNodeData::Pop

        | AstNodeData::Inputn
        | AstNodeData::Inputb
        | AstNodeData::Inputs

        | AstNodeData::Print
        | AstNodeData::Println

        | AstNodeData::Cmpg
        | AstNodeData::Cmpge

        | AstNodeData::Cmpl
        | AstNodeData::Cmple

        | AstNodeData::Cmpe
        | AstNodeData::Cmpne => {}, // discriminant already pushed

        AstNodeData::Pushv(var)
        | AstNodeData::Popv(var) => encode_string(&mut output, var),

        AstNodeData::Label(label)
        | AstNodeData::Jmp(label)
        | AstNodeData::Jt(label)
        | AstNodeData::Jf(label) => encode_string(&mut output, label),
    }
  }

  file.write_all(&output)?;
  Ok(())
}

pub fn encode_string(output: &mut Vec<u8>, s: &str) {
  output.extend_from_slice(&(s.len() as u32).to_ne_bytes());
  output.extend_from_slice(s.as_bytes());
}
