use crate::token::Token;
pub struct FileLocation {
  pub filename: String,
  pub line: i32,
  pub column: i32
}

pub fn token_to_fileloc(token: &Token) -> FileLocation {
  FileLocation {
    filename: token.filename.clone(),
    line: token.line,
    column: token.column
  }
}