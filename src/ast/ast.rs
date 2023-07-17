use super::ast_types::*;
use super::file_location::FileLocation;

pub fn create_program_ast(file_name: &String, ln: i32, col: i32) -> AST {
  let owned_path = file_name.to_owned();
  let mut progast = new_ast(&owned_path, ln, col);
  progast.type_tag = ASTType::Program;
  progast.data = AST_Union { program: ProgramTree::init() };

  progast
}

fn new_ast(file_name: &String, ln: i32, col: i32) -> AST {
  AST {
    fileloc: FileLocation {
      filename: file_name.to_string(),
      line: ln,
      column: col
    },
    type_tag: ASTType::EMPTY,
    next: Vec::new(),
    data: AST_Union { empty: EmptyTree::init() }
  }
}