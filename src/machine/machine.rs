use std::{fs::File, io::{Read, stdin, stdout, Write}};
use super::machine_types::{Word, Address, Instruction};
use super::stack::Stack;
use super::instruction;

const MAX_CODE_LENGTH: usize = 512;
struct Machine {
  pc: Address,
  halt: bool,
  no_out: bool,
  code: Vec<Instruction>,
  debug: bool,
}

impl Machine {
  fn initialize() -> Self {
    let new_machine = Machine { pc: 0, halt: false, no_out: true, code: Vec::new(), debug: false };
    new_machine
  }

  fn enable_debug(&mut self) {
    self.debug = true;
  }

  fn enable_trace(&mut self) {
    self.no_out = false;
  }

  fn run_machine(&mut self, stack: &mut Stack, program_length: i32) {
    println!("Running...");
    if !self.debug && self.no_out {
      println!("Hint: to enable debug mode, run the program with the '-debug' flag.");
      println!("To enable tracing, run the program with the '-trace' flag.");
    }
    println!("\n");

    if !self.no_out {
      if self.debug {
        println!("Program length: {}", program_length);
      }
      self.print_program(program_length);
      println!("\nTracing...");
      self.print_machine(stack);
    }

    while !self.halt {
      self.okay_to_run();
      let instr = self.code[self.pc as usize].copy();
      self.trace_and_execute(stack, &instr);
    }
  }

  fn okay_to_run(&self)  {
    if !self.pc >= 0 || !(self.pc < self.code.len().try_into().unwrap()) {
      panic!("Error: Program Counter out of bounds! (PC: {}, Program Length: {})", self.pc, self.code.len());
    }
  }

  fn read_program(&mut self, file_contents: &String) -> i32 {
    let mut program: Vec<Instruction> = Vec::new();

    let mut count: i32 = 0;
    for line in file_contents.lines() {
      count += 1;
      let instr = instruction::read_instruction(line);
      if instr.op == -1 {
        count -= 1;
        continue;
      }
      program.push(instr);
    }

    if count >= MAX_CODE_LENGTH as i32 {
      panic!("Error: Too many instructions! (Code length: {}, Max: {})", count, MAX_CODE_LENGTH);
    }

    self.code = program;
    count
  }

  fn print_program(&self, program_length: i32) {
    instruction::print_instruction_heading();
    for i in 0..program_length {
      instruction::print_formatted_instruction(i, &self.code[i as usize]);
    }
  }

  fn print_machine(&self, stack: &Stack) {
    println!("Machine:");
    println!("PC: {}, BP: {}, SP: {}", self.pc, stack.ar_base(), stack.size());
    println!("Stack:");
    stack.print_stack();
  }

  fn trace_and_execute(&mut self, stack: &mut Stack, instr: &Instruction) {
    if !self.no_out {
      print!("--> addr: ");
      instruction::print_formatted_instruction(self.pc, instr);
    }

    self.execute(stack, instr);

    if !self.no_out {
      self.print_machine(stack);
    }
  }

  fn execute(&mut self, stack: &mut Stack, instr: &Instruction) {
    self.pc += 1;
    self.halt = false;

    match instr.op {
      0 => { // NOP
        if self.debug {
          println!("NOP");
          println!("{:->40}", "");
        }
      },
      1 => { // LIT
        if self.debug {
          println!("LIT {}", instr.m);
          println!("{:->40}", "");
        }
        stack.push(instr.m);
      },
      2 => { // RTN
        if self.debug {
          println!("RTN");
          println!("{:->40}", "");
        }
        stack.return_stack(self.pc);
      },
      3 => { // CAL
        if self.debug {
          println!("CAL {}", instr.m);
          println!("{:->40}", "");
        }
        stack.call(self.pc);
      },
      4 => { // POP
        if self.debug {
          println!("POP {}", instr.m);
          println!("{:->40}", "");
        }
        stack.pop();
      },
      5 => { // PSI
        if self.debug {
          println!("PSI {}", instr.m);
          println!("{:->40}", "");
        }
        let address: Address = stack.pop();
        stack.push(stack.fetch(address));
      },
      6 => { // LOD
        if self.debug {
          println!("LOD {}", instr.m);
          println!("{:->40}", "");
        }
        let address: Address = stack.pop() + instr.m;
        stack.push(stack.fetch(address));
      },
      7 => { // STO
        if self.debug {
          println!("STO {}", instr.m);
          println!("{:->40}", "");
        }
        let word: Word = stack.pop();
        let destination: Address = stack.pop() + instr.m;
        stack.assign(destination, word);
      },
      8 => { // INC
        if self.debug {
          println!("INC {}", instr.m);
          println!("{:->40}", "");
        }
        stack.allocate(instr.m);
      },
      9 => { // JMP
        if self.debug {
          println!("JMP {}", instr.m);
          println!("{:->40}", "");
        }
        self.pc += instr.m - 1;
      },
      10 => { // JPC
        if self.debug {
          println!("JPC {}", instr.m);
          println!("{:->40}", "");
        }
        if stack.pop() != 0 {
          self.pc += instr.m - 1;
        }
      },
      11 => { // CHO
        if self.debug {
          println!("CHO");
          println!("{:->40}", "");
        }
        let output: Word = stack.pop();
        println!("OUTPUT: {}", output);
      },
      12 => { // CHI
        if self.debug {
          println!("CHI");
          println!("{:->40}", "");
        }
        let input: Word = read_console_input();
        stack.push(input);
      },
      13 => { // HLT
        if self.debug {
          println!("HLT");
          println!("{:->40}", "");
        }
        self.halt = true;
      },
      14 => { // NDB
        if self.debug {
          println!("NDB");
          println!("{:->40}", "");
        }
        println!("\nno_out");
        self.no_out = true;
      },
      15 => { // NEG
        if self.debug {
          println!("NEG");
          println!("{:->40}", "");
        }
        let neg_value: Word = - stack.pop();
        stack.push(neg_value);
      },
      16 => { // ADD
        if self.debug {
          println!("ADD");
          println!("{:->40}", "");
        }
        let top_value: Word = stack.pop();
        let bottom_value: Word = stack.pop();
        stack.push(bottom_value + top_value);
      },
      17 => { // SUB
        if self.debug {
          println!("SUB");
          println!("{:->40}", "");
        }
        let top_value: Word = stack.pop();
        let bottom_value: Word = stack.pop();
        stack.push(bottom_value - top_value);
      },
      18 => { // MUL
        if self.debug {
          println!("MUL");
          println!("{:->40}", "");
        }
        let top_value: Word = stack.pop();
        let bottom_value: Word = stack.pop();
        stack.push(bottom_value * top_value);
      },
      19 => { // DIV
        if self.debug {
          println!("DIV");
          println!("{:->40}", "");
        }
        let top_value: Word = stack.pop();
        if top_value == 0 {
          panic!("Error: Division by zero!");
        }
        let bottom_value: Word = stack.pop();
        stack.push(bottom_value / top_value);
      },
      20 => { // MOD
        if self.debug {
          println!("MOD");
          println!("{:->40}", "");
        }
        let top_value: Word = stack.pop();
        if top_value == 0 {
          panic!("Error: Modulo by zero!");
        }
        let bottom_value: Word = stack.pop();
        stack.push(bottom_value % top_value);
      },
      21 => { // EQL
        if self.debug {
          println!("EQL");
          println!("{:->40}", "");
        }
        let top_value: Word = stack.pop();
        let bottom_value: Word = stack.pop();
        match bottom_value == top_value {
          true => stack.push(1),
          false => stack.push(0),
        }
      },
      22 => { // NEQ
        if self.debug {
          println!("NEQ");
          println!("{:->40}", "");
        }
        let top_value: Word = stack.pop();
        let bottom_value: Word = stack.pop();
        match bottom_value != top_value {
          true => stack.push(1),
          false => stack.push(0),
        }
      },
      23 => { // LSS
        if self.debug {
          println!("LSS");
          println!("{:->40}", "");
        }
        let top_value: Word = stack.pop();
        let bottom_value: Word = stack.pop();
        match bottom_value < top_value {
          true => stack.push(1),
          false => stack.push(0),
        }
      },
      24 => { // LEQ
        if self.debug {
          println!("LEQ");
          println!("{:->40}", "");
        }
        let top_value: Word = stack.pop();
        let bottom_value: Word = stack.pop();
        match bottom_value <= top_value {
          true => stack.push(1),
          false => stack.push(0),
        }
      },
      25 => { // GTR
        if self.debug {
          println!("GTR");
          println!("{:->40}", "");
        }
        let top_value: Word = stack.pop();
        let bottom_value: Word = stack.pop();
        match bottom_value > top_value {
          true => stack.push(1),
          false => stack.push(0),
        }
      },
      26 => { // GEQ
        if self.debug {
          println!("GEQ");
          println!("{:->40}", "");
        }
        let top_value: Word = stack.pop();
        let bottom_value: Word = stack.pop();
        match bottom_value >= top_value {
          true => stack.push(1),
          false => stack.push(0),
        }
      },
      27 => { // PSP
        if self.debug {
          println!("PSP");
          println!("{:->40}", "");
        }
        stack.push(stack.size());
      },
      28 => { // PBP
        if self.debug {
          println!("PBP");
          println!("{:->40}", "");
        }
        stack.push(stack.ar_base());
      },
      29 => { // PPC
        if self.debug {
          println!("PPC");
          println!("{:->40}", "");
        }
        stack.push(self.pc);
      },
      30 => { // JMI
        if self.debug {
          println!("PPC");
          println!("{:->40}", "");
        }
        self.pc = stack.pop();
      },
      _ => {
        panic!("Error: Undefined opcode: {}!", instr.op);
      }
    }
  }
}

pub fn start_machine(file_name: &String, debug: bool, trace: bool) {
  let mut stack: Stack = Stack::initialize();
  let mut machine: Machine = Machine::initialize();

  if debug {
    println!("DEBUG: ON");
    machine.enable_debug();
  }

  if trace {
    println!("TRACE: ON");
    machine.enable_trace();
  }

  if machine.debug {
    println!("Reading file `{}`...", file_name);
  }

  let file_contents: String = open_file(file_name);

  if machine.debug {
    println!("File contents:\n\n{}", file_contents);
  }

  let program_length = machine.read_program(&file_contents);

  machine.run_machine(&mut stack, program_length);
}

fn open_file(file_name: &String) -> String {
  let file_path: String = file_name.to_owned();

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

fn read_console_input() -> i32 {
  print!("INPUT > ");
  let _ = stdout().flush();
  let mut input = String::new();
  let res = stdin().read_line(&mut input);

  match res {
    Ok(_) => {},
    Err(_) => panic!("Error: Could not read input!"),
  }

  let value: i32;
  value = match input.trim().parse() {
    Ok(num) => num,
    Err(_) => {
      println!("ERROR: Could not parse input! Defaulting to 0...");
      0
    },
  };

  value
}