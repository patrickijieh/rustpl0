use std::{fs::File, io::Read};
struct Lexer {
  input_file_name: String,
  done: bool,
  line: i32,
  column: i32,
}

impl Lexer {
  pub fn initialize(filename: &String) -> Self {
    let new_lexer = Lexer { input_file_name: filename.to_string(), done: false, line: 0, column: 0 };
    new_lexer
  }
}

pub fn lexer_open(filename: &String) {
  println!("lexer_open({})", filename);
  let mut lexer = Lexer::initialize(filename);
  let file_contents = read_file(filename);
}

fn read_file(filename: &String) -> String {
  let file_path: String = filename.to_owned();

  let file = File::open(file_path.clone());

  match file {
    Ok(_) => {},
    Err(_) => {
      panic!("Error: Could not open file `{}`", file_path);
    },
  }

  let mut contents = String::new();

  match file {
    Ok(mut f) => {
      let _ = f.read_to_string(&mut contents);
    },
    Err(_) => {
      panic!("Error: Could not read file `{}`", file_path);
    },
  }
  
  contents
}