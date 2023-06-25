use std::{fs::File, io::Read};
use super::machine_types::{Word, Address, Instruction};
use super::stack::Stack;
use super::instruction;

const MAX_CODE_LENGTH: usize = 512;
struct Machine {
  pc: Address,
  halt: bool,
  no_output: bool,
  code: Vec<Instruction>,
  debug: bool,
}

impl Machine {
  fn initialize() -> Self {
    let new_machine = Machine { pc: 0, halt: false, no_output: false, code: Vec::new(), debug: true };
    new_machine
  }

  fn print_machine(&self, stack: &Stack) {
    println!("Machine:");
    println!("PC: {}, BP: {}, SP: {}", self.pc, stack.ar_base(), stack.size());
    println!("Stack:");
    stack.print_stack();
  }

  fn execute(&mut self, stack: &mut Stack) {
    self.pc += 1;
  }

  fn read_program(&mut self, file_contents: &String) -> i32 {
    let mut program: Vec<Instruction> = Vec::new();

    let mut count: i32 = 0;
    for line in file_contents.lines() {
      count += 1;
      let instr = instruction::read_instruction(line);
      program.push(instr);
    }

    if count >= MAX_CODE_LENGTH as i32 {
      panic!("Error: Too many instructions! (Code length: {}, Max: {})", count, MAX_CODE_LENGTH);
    }

    self.code = program;
    count
  }
}

pub fn start_machine(file_name: &String) {
  let mut stack: Stack = Stack::initialize();
  let mut machine: Machine = Machine::initialize();

  println!("Reading file `{}`...", file_name);
  let file_contents: String = open_file(file_name);

  let program_length = machine.read_program(&file_contents);
  println!("File read successfully! Program length: {}", program_length);

  if !machine.no_output {
    println!("\nTracing...");

  }
  machine.print_machine(&stack);
  machine.execute(&mut stack);
  machine.print_machine(&stack);
}

fn open_file(file_name: &String) -> String {
  let file_path: String = file_name.to_owned();

  let error_msg: String = format!("Error: Could not read file `{}`", file_path);

  let mut file = File::open(file_path)
    .expect(error_msg.as_str());

  let mut contents = String::new();

  file.read_to_string(&mut contents)
    .expect(error_msg.as_str());

  println!("File contents:\n\n{}", contents);

  contents
}