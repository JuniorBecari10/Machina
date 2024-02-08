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

#[derive(Debug, Clone)]
#[repr(u8)]
pub enum AstNodeData {
  Label(String),

  Pushc(Value),
  Pushv(String),
  
  Setc(Value),
  Pop(String),

  Add,
  Sub,
  Mul,
  Div,

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

  Jmp(String),
  Jt(String),
  Jf(String),
}

#[derive(Debug, Clone)]
#[repr(u8)]
pub enum Value {
  Num(f64),
  Str(String),
  Bool(bool)
}
