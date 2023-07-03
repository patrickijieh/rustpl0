use std::{fs::File, io::{prelude::*, SeekFrom, BufReader}};
use crate::token::{Token, TokenType};
use crate::reserved_types::get_reserved_word;

const END_OF_FILE: char = '\u{0003}';
const MAX_IDENTIFIER_LENGTH: usize = 255;
const MAX_NUMBER_LENGTH: usize = 5;
struct Lexer {
  input_file_name: String,
  file_reader: BufReader<File>,
  done: bool,
  line: i32,
  column: i32,
  last_column: i32
}

impl Lexer {
  fn initialize(filename: &String, reader: BufReader<File>) -> Self {
    let new_lexer = Lexer { input_file_name: filename.to_string(), file_reader: reader, done: false, line: 0, column: 0, last_column: 0 };
    new_lexer
  }

  fn error(&self, msg: &str) {
    panic!("Error: {}\nAt line: {}, column: {}", msg, self.line, self.column);
  }

  fn lexer_run(&mut self) {
    while !self.done {
      let t = self.lexer_next();
      //print!("token: {}, type: {}", t.text, t.ttyp2str());
      if t.typ == TokenType::Numbersym {
        //print!(", value: {}", t.value);
      }
      //println!("");
    }
  }

  fn lexer_next(&mut self) -> Token {
    let mut t: Token = Token::with_file(self.input_file_name.clone());

    self.consume_ignored();

    t.line = self.line;
    t.column = self.column;

    let c = self.getchar();

    if c == END_OF_FILE  {
      //print!("found EOF\n");
      t.typ = TokenType::Eofsym;
      t.text = END_OF_FILE.to_string();
      self.done = true;
      return t;
    }

    if is_alpha(c) {
      return self.lexer_ident(c, t);
    } else if is_numeric(c) {
      return self.lexer_num(c, t);
    }

    t.text = c.to_string();

    //print!("line {} column {} char {}\n", t.line, t.column, c);

    t 
  }

  fn getchar(&mut self) -> char {
    let mut buffer = [0; 1];

    let res = self.file_reader.read(&mut buffer);

    match res {
      Ok(n) => {
        if n == 0 {
          return END_OF_FILE;
        }
      },
      Err(err) => {
        panic!("Error: {}: Could not read from file `{}`", err.kind(), self.input_file_name);
      },
    }

    self.last_column = self.column;

    if buffer[0] == b'\n' {
      self.line += 1;
      self.column = 0;
    } else {
      self.column += 1;
    }

    buffer[0] as char
  }

  fn ungetchar(&mut self, c: char) {

    self.column = self.last_column;

    if c == '\n' {
      self.line -= 1;
    }

    if c != END_OF_FILE {
      self.file_reader.seek(SeekFrom::Current(-1))
      .expect("Error: Could not seek back one character!");
    }
  }

  fn consume_ignored(&mut self) {
    let mut c = self.getchar();
    while is_space(c) || is_comment(c) {
      if is_space(c) {
        c = self.getchar();
      } else if is_comment(c) {
        self.consume_comment();
        c = self.getchar();
      }
    }
    self.ungetchar(c);
  }

  fn consume_comment(&mut self) {
    let mut c = self.getchar();
    while (c != '\n') && (c != END_OF_FILE) {
      c = self.getchar();
    }

    if c == END_OF_FILE {
      self.done = true;
      self.error("Unexpected end of file while reading comment!");
    }

    self.line += 1;
  }

  fn lexer_ident(&mut self, c: char, mut tok: Token) -> Token {
    let mut ident = String::new();
    let mut i: u8 = 0;

    ident.push(c);

    let mut c = self.getchar();

    while is_alpha(c) || is_numeric(c) {
      if i >= MAX_IDENTIFIER_LENGTH as u8 {
        self.error(format!("Identifier starting with {} is too long!", ident).as_str());
      }

      ident.push(c);
      i += 1;
      c = self.getchar();
    }

    self.ungetchar(c);
    tok.text = ident;
    tok.typ = get_reserved_word(&tok.text);
    tok
  }

  fn lexer_num(&mut self, c: char, mut tok: Token) -> Token {
    let mut num = String::new();
    let mut i: u8 = 0;

    num.push(c);
    let mut c = self.getchar();

    while is_numeric(c) {
      if i >= MAX_NUMBER_LENGTH as u8 {
        self.error(format!("Number starting with {} is too long!", num).as_str());
      }
      num.push(c);
      i += 1;
      c = self.getchar();
    }

    self.ungetchar(c);
    let mut val: i32 = 0;
    for digit in num.chars() {
      val = val * 10 + digit.to_digit(10).unwrap() as i32;
    }

    tok.text = num;
    tok.value = val;
    tok.typ = TokenType::Numbersym;
    tok
  }
  
}

pub fn lexer_open(filename: &String) {
  //println!("lexer_open()");
  let reader = create_reader(filename);
  let mut lexer = Lexer::initialize(filename, reader);
  lexer.lexer_run();
}

fn create_reader(filename: &String) -> BufReader<File> {
  let file_path: String = filename.to_owned();

  let file = File::open(file_path.clone());

  let f;
  match file {
    Ok(_) => {f = file.unwrap();},
    Err(_) => {
      panic!("Error: Could not open file `{}`", file_path);
    },
  }

  // sizeof(char) = 4
  let reader = BufReader::with_capacity(4, f);
  
  reader
}

fn is_space(c: char) -> bool {
  (c == ' ') || (c == '\t')
}

fn is_comment(c: char) -> bool {
  c == '#'
}

fn is_alpha(c: char) -> bool {
  c.is_alphabetic()
}

fn is_numeric(c: char) -> bool {
  c.is_numeric()
}