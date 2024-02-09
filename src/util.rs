use crate::ast::*;

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

pub fn is_identifier(s: &str) -> bool {
  for (i, c) in s.char_indices() {
    if i == 0 {
      if !(c.is_alphabetic() || c == '_') { return false };
    }

    else if !(c.is_alphanumeric() || c == '_') { return false };
  }

  true
}

pub fn is_label(s: &str) -> bool {
  s.starts_with('#') && is_identifier(&s[1..])
}

pub fn print_error(msg: &str, code: &str, mut line: usize) {
  line += 1;

  eprintln!("Error in line {line}: {msg}");
  eprintln!(" {line} | {code}\n");
}

pub fn change_file_extension(filename: &str, extension: &str) -> String {
  if filename.contains('.') {
    let split: Vec<&str> = filename.split('.').map(|s| s.trim()).collect();
    let not_last = split[..split.len() - 1].join(".");

    format!("{}.{}", not_last, extension)
  }
  else {
    format!("{}.{}", filename, extension)
  }
}
