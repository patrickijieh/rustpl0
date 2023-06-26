use super::machine_types::{Address, Instruction};

const NUM_OPCODES: usize = 31;

// NOP, LIT, RTN, CAL, POP,
//   PSI, LOD, STO, INC, JMP,
//   JPC, CHO, CHI, HLT, NDB,
//   NEG, ADD, SUB, MUL, DIV,
//   MOD, EQL, NEQ, LSS, LEQ,
//   GTR, GEQ, PSP, PBP, PPC,
//   JMI,

const OPCODES: [&str; NUM_OPCODES] = [
  "NOP", "LIT", "RTN", "CAL", "POP",
  "PSI", "LOD", "STO", "INC", "JMP",
  "JPC", "CHO", "CHI", "HLT", "NDB",
  "NEG", "ADD", "SUB", "MUL", "DIV",
  "MOD", "EQL", "NEQ", "LSS", "LEQ",
  "GTR", "GEQ", "PSP", "PBP", "PPC",
  "JMI",
];

fn legal_op_code(op: i32) -> bool {
  0 <= op && op < NUM_OPCODES as i32
}

pub fn read_instruction(instruction_line: &str) -> Instruction {
  let mut new_instruction: Instruction = Instruction { op: 0, m: 0 };
  let instruction_line: Vec<&str> = instruction_line.split_whitespace().collect();

  if instruction_line.len() != 2 {
    new_instruction.op = -1;
    return new_instruction;
  }
  new_instruction.op = instruction_line[0].parse::<i32>().unwrap();
  new_instruction.m = instruction_line[1].parse::<i32>().unwrap();

  if !legal_op_code(new_instruction.op) {
    panic!("Error: Illegal op code! (Op code: {})", new_instruction.op);
  }

  new_instruction
}


fn mnemonic(op: i32) -> String {
  if !legal_op_code(op) {
    panic!("Error: Illegal op code! (Op code: {})", op);
  }

  OPCODES[op as usize].to_string()
}

pub fn print_instruction_heading() {
  println!("{:>5} {:>5} {:>5}", "ADDR", "OP", "M");
}

pub fn print_formatted_instruction(address: Address, instruction: &Instruction) {
  println!("{:>5} {:>5} {:>5}", address, mnemonic(instruction.op), instruction.m);
}

pub fn print_machine_instruction(instruction: Instruction) {
  println!("{:>5} {:>5}", instruction.op, instruction.m);
}