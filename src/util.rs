use std::{fs::File, io::{self, Read}};

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
  
  eprintln!("  [X] Error in line {line}: {msg}");
  eprintln!(" {line} | {code}\n");
}

pub fn print_error_reduced(msg: &str) {
  eprintln!("Error: {msg}");
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

pub fn read_to_vec(file: &str) -> io::Result<Vec<u8>> {
  let mut file = File::open(file)?;

  let mut buffer = vec![];
  file.read_to_end(&mut buffer)?;

  Ok(buffer)
}

pub fn custom_split(input: &str) -> Vec<String> {
  let mut result = Vec::new();
  let mut current_word = String::new();
  let mut in_quotes = false;
  
  for c in input.chars() {
    if c.is_whitespace() && !in_quotes {
      if !current_word.is_empty() {
        result.push(current_word.clone());
        current_word.clear();
      }
    }
    
    else if c == '"' {
      in_quotes = !in_quotes;
      
      if !current_word.is_empty() && !in_quotes {
        result.push(format!("\"{}\"", current_word));
        current_word.clear();
      }
    }
    
    else {
      current_word.push(c);
    }
  }
  
  if !current_word.is_empty() {
    if in_quotes {
      result.push(format!("\"{}\"", current_word));
    }
    
    else {
      result.push(current_word);
    }
  }
  
  result
}
