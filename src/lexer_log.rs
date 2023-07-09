use std::{fs::File, io::prelude::*};
use crate::token::{Token, TokenType};

pub fn create_log(filename: String, token_stream: &Vec<Token>) {
    println!("Creating lexer.log file...");
    let mut log;
    let f = File::create("lexer.log");

    match f {
        Ok(file) => {
            log = file;
            write_header(&mut log, filename, token_stream);
        },
        Err(_) => {
            println!("Error: could not create log file!");
        }
    }
}

pub fn write_header(logfile: &mut File, filename: String, token_stream: &Vec<Token>) {
  let file_header = format!("Tokens from file {}:\n\n", filename);
  let tokens_header = format!("{:12}\t{:8}\t{:8}\t{:12}\n", "Type", "Line", "Column", "Text/Value");

  let _ = logfile.write_all(file_header.as_bytes());
  let _ = logfile.write_all(tokens_header.as_bytes());

  for i in token_stream {
    if i.typ == TokenType::Numbersym {
      let token_string = format!("{:<12}\t{:<8}\t{:<8}\t{:<12}\n", i.ttyp2str(), i.line, i.column, i.value);
      let _ = logfile.write_all(token_string.as_bytes());
      let _ = logfile.write_all(b"\n");
      continue;
    } else if i.typ == TokenType::Eofsym {
      let token_string = format!("{:<12}\t{:<8}\t{:<8}\t{:<12}\n", i.ttyp2str(), i.line, i.column, "EOF");
      let _ = logfile.write_all(token_string.as_bytes());
      let _ = logfile.write_all(b"\n");
      continue;
    }
    let token_string = format!("{:<12}\t{:<8}\t{:<8}\t{:<12}\n", i.ttyp2str(), i.line, i.column, i.text);
    let _ = logfile.write_all(token_string.as_bytes());
    let _ = logfile.write_all(b"\n");
  }
}