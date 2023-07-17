use crate::lexer::lexer_open;
use crate::token::{Token, TokenType};
use crate::ast::*;

pub fn parser_open(filename: &String, debug: bool, trace: bool) {
  let token_stream = lexer_open(filename, debug);
  ast::create_program_ast(filename, 0, 0);
}