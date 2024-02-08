#[derive(Debug)]
pub enum AstNode {
  Label(String),

  Pushc(Value),
  Pushv(String),
  Setc(Value),
  Pop(String),
}

#[derive(Debug)]
pub enum Value {
  Num(f64),
  Str(String),
  Bool(bool)
}
