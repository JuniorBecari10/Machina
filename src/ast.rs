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
  Bool(bool)
}
