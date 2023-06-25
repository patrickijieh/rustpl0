pub type Word = i32;

pub type Address = i32;

#[derive(Debug)]
pub struct Instruction {
  pub op: i32,
  pub m: i32,
}