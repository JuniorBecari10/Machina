#[derive(Debug)]
pub enum AstNode {
  Pushc(Value)
}

#[derive(Debug)]
pub enum Value {
  Num(f64),
  Str(String),
  Bool(bool)
}

pub fn parse_value(s: &str) -> Option<Value> {
  if s == "true" || s == "false" {
    Some(Value::Bool(s == "true"))
  }

  else if s.starts_with('\"') && s.ends_with('\"') {
    Some(Value::Str(s[1..s.len() - 1].into()))
  }

  else if let Ok(n) = s.parse::<f64>() {
    Some(Value::Num(n))
  }

  else {
    None
  }
}
