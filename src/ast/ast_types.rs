use std::mem::ManuallyDrop;
use super::file_location::FileLocation;
pub enum ASTType {
  Program,
  ConstDecl, VarDecl, ProcDecl,
  Assign, Call, Begin,
  If, While, Read, Write, Skip,
  OddCond, BinCond, OpExpr, BinExpr,
  Ident, Number, EMPTY,
}

pub enum RelationalOp {
  Eqop, Neqop,
  Ltop, Leqop, Gtop, Geqop,
}

pub enum ArithmeticOp {
  Addop, Subop, Mulop, Divop,
}

pub struct ProgramTree {
  consts: Vec<AST>,
  vars: Vec<AST>,
  procs: Vec<AST>,
  stmt: Vec<AST>
}

pub struct ConstDeclTree {
  name: String,
  value: i32
}

pub struct VarDeclTree {
  name: String,
}

pub struct ProcDeclTree {
  name: String,
  block: Vec<AST>,
}

pub struct AssignTree {
  ident: String,
  expr: Vec<AST>,
}

pub struct CallTree {
  ident: String,
}

pub struct BeginTree {
  stmt: Vec<AST>,
}

pub struct IfTree {
  cond: Vec<AST>,
  then_stmt: Vec<AST>,
  else_stmt: Vec<AST>,
}

pub struct WhileTree {
  cond: Vec<AST>,
  stmt: Vec<AST>,
}

pub struct ReadTree {
  ident: Vec<AST>,
}

pub struct WriteTree {
  expr: Vec<AST>,
}

pub struct SkipTree {

}

pub struct OddCondTree {
  expr: Vec<AST>,
}

pub struct BinCondTree {
  left: Vec<AST>,
  rel_op: RelationalOp,
  right: Vec<AST>,
}

pub struct OpExprTree {
  arith_op: ArithmeticOp,
  expr: Vec<AST>,
}

pub struct BinExprTree {
  left: Vec<AST>,
  arith_op: ArithmeticOp,
  right: Vec<AST>,
}

pub struct IdentTree {
  name: String,
}

pub struct NumberTree {
  value: i32,
}

pub struct EmptyTree {

}
pub union AST_Union {
  pub program: ManuallyDrop<ProgramTree>,
  pub const_decl: ManuallyDrop<ConstDeclTree>,
  pub var_decl: ManuallyDrop<VarDeclTree>,
  pub proc_decl: ManuallyDrop<ProcDeclTree>,
  pub assign_stmt: ManuallyDrop<AssignTree>,
  pub call_stmt: ManuallyDrop<CallTree>,
  pub begin_stmt: ManuallyDrop<BeginTree>,
  pub if_stmt: ManuallyDrop<IfTree>,
  pub while_stmt: ManuallyDrop<WhileTree>,
  pub read_stmt: ManuallyDrop<ReadTree>,
  pub write_stmt: ManuallyDrop<WriteTree>,
  pub skip_stmt: ManuallyDrop<SkipTree>,
  pub dd_cond: ManuallyDrop<OddCondTree>,
  pub bin_cond: ManuallyDrop<BinCondTree>,
  pub op_expr: ManuallyDrop<OpExprTree>,
  pub bin_expr: ManuallyDrop<BinExprTree>,
  pub ident: ManuallyDrop<IdentTree>,
  pub number: ManuallyDrop<NumberTree>,
  pub empty: ManuallyDrop<EmptyTree>,
}

pub struct AST {
  pub fileloc: FileLocation,
  pub type_tag: ASTType,
  pub next: Vec<AST>,
  pub data: AST_Union,
}

pub trait Init {
  fn init() -> ManuallyDrop<Self>;
}

impl Init for EmptyTree {
  fn init() -> ManuallyDrop<Self> {
    ManuallyDrop::new(EmptyTree {})
  }
}

impl Init for ProgramTree {
  fn init() -> ManuallyDrop<Self> {
    ManuallyDrop::new(ProgramTree {
      consts: Vec::new(),
      vars: Vec::new(),
      procs: Vec::new(),
      stmt: Vec::new()
    })
  }
}