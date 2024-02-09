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
        AstNodeData::Setc(val, var) => todo!(),

        AstNodeData::Add
        | AstNodeData::Sub
        | AstNodeData::Mul
        | AstNodeData::Div

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
        | AstNodeData::Cmpne

        | AstNodeData::Save
        | AstNodeData::Ret => {}, // discriminant already pushed

        AstNodeData::Pushv(var) => todo!(),
        AstNodeData::Pop(var) => todo!(),

        AstNodeData::Label(label) => todo!(),
        AstNodeData::Jmp(label) => todo!(),
        AstNodeData::Jt(label) => todo!(),
        AstNodeData::Jf(label) => todo!(),
    }
  }

  file.write_all(&output)?;
  Ok(())
}
