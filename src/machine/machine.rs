use std::{fs::File, io::Read};
use super::machine_types::{Word, Address};
use super::stack::Stack;

struct Machine {
  pc: i32,
  halt: bool,
  no_output: bool,
  code: Vec<Word>,
}

impl Machine {
  fn initialize() -> Self {
    let new_machine = Machine { pc: 0, halt: false, no_output: false, code: Vec::new() };
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
    stack.push(1);
  }
}

pub fn start_machine(file_name: &String) {
  let mut stack: Stack = Stack::initialize();
  let mut machine: Machine = Machine::initialize();

  println!("Reading file `{}`...", file_name);
  let file_contents: String = open_file(file_name);
  println!("File read successfully! Length: {}", file_contents.len());

  println!("\nTracing...");
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