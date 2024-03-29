use crate::compiler::encode_string;

#[derive(Debug)]
pub struct AstNode {
  pub data: AstNodeData,
  pub code: String,
  pub line: usize
}

impl AstNode {
  pub fn new(data: AstNodeData, code: String, line: usize) -> Self {
    Self {
      data,
      code,
      line
    }
  }
}

#[derive(Debug)]
pub struct ReducedAstNode(pub AstNodeData);

#[derive(Debug, Clone)]
#[repr(u8)]
pub enum AstNodeData {
  Label(String),

  Pushc(Value),
  Pushv(String),
  
  Setc(String, Value),
  Popv(String),

  Pop,

  Add,
  Sub,
  Mul,
  Div,

  Inc,
  Dec,

  Inputn,
  Inputb,
  Inputs,

  Print,
  Println,

  Cmpg,
  Cmpge,

  Cmpl,
  Cmple,

  Cmpe,
  Cmpne,

  Jmp,
  Jt,
  Jf,

  Save,
  Ret,
}

impl AstNodeData {
  pub fn discriminant(&self) -> u8 {
    // Safety: got from <https://doc.rust-lang.org/std/mem/fn.discriminant.html>
    unsafe { *<*const _>::from(self).cast::<u8>() }
  }
}

#[derive(Debug, Clone)]
#[repr(u8)]
pub enum Value {
  Num(f64),
  Str(String),
  Bool(bool),
  Label(String),
  // TODO! Ref(String)
}

impl Value {
  pub fn as_str(&self) -> String {
    match self {
      Value::Num(n) => format!("{}", n),
      Value::Str(s) => s.clone(),
      Value::Bool(b) => format!("{}", b),
      Value::Label(l) => l.clone(),
    }
  }

  pub fn as_str_debug(&self) -> String {
    match self {
      Value::Num(n) => format!("num {}", n),
      Value::Str(s) => format!("str \"{}\"", s),
      Value::Bool(b) => format!("bool {}", b),
      Value::Label(l) => format!("label {}", l),
    }
  }

  pub fn encode(&self) -> Vec<u8> {
    let mut output = vec![];
    output.push(self.discriminant());

    match self {
      Value::Num(n) => output.extend_from_slice(&n.to_le_bytes()),
      Value::Str(s) => encode_string(&mut output, s),
      Value::Bool(b) => output.push(*b as u8),
      Value::Label(l) => encode_string(&mut output, l),
    }

    output
  }

  pub fn discriminant(&self) -> u8 {
    // Safety: got from <https://doc.rust-lang.org/std/mem/fn.discriminant.html>
    unsafe { *<*const _>::from(self).cast::<u8>() }
  }
}
