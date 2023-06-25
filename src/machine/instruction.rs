use super::machine_types::{Word, Address, Instruction};

const NUM_OPCODES: usize = 31;

enum OPCODES {
  NOP, LIT, RTN, CAL, POP,
  PSI, LOD, STO, INC, JMP,
  JPC, CHO, CHI, HLT, NDB,
  NEG, ADD, SUB, MUL, DIV,
  MOD, EQL, NEQ, LSS, LEQ,
  GTR, GEQ, PSP, PBP, PPC,
  JMI,
}

fn legal_op_code(op: i32) -> bool {
  0 <= op && op < NUM_OPCODES as i32
}

pub fn read_instruction(instruction_line: &str) -> Instruction {
  let mut new_instruction: Instruction = Instruction { op: 0, m: 0 };
  let instruction_line: Vec<&str> = instruction_line.split_whitespace().collect();

  new_instruction.op = instruction_line[0].parse::<i32>().unwrap();
  new_instruction.m = instruction_line[1].parse::<i32>().unwrap();

  if !legal_op_code(new_instruction.op) {
    panic!("Error: Illegal op code! (Op code: {})", new_instruction.op);
  }

  new_instruction
}