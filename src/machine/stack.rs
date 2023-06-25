use super::machine_types::{Word, Address};

const MAX_STACK_HEIGHT: usize = 2048;

pub struct Stack {
  stack: [Word; MAX_STACK_HEIGHT],
  sp: Address,
  bp: Address,
}

impl Stack {
  fn stack_invariant(&self) -> bool {
    if !(self.bp >= 0) {
      panic!("VM stack invariant failure: BP ({}) < 0!", self.bp);
    }  else if !(self.sp >= 0) {
      panic!("VM stack invariant failure: SP ({}) < 0!", self.sp);
    } else if !(self.sp >= self.bp) {
      panic!("VM stack invariant failure: SP ({}) < BP ({})!", self.sp, self.bp);
    } else if !(self.sp < MAX_STACK_HEIGHT.try_into().unwrap()) {
      panic!("VM stack invariant failure: SP ({}) >= MAX_STACK_HEIGHT ({})!", self.sp, MAX_STACK_HEIGHT);
    }

    true
  }

  fn legal_stack_index(addr: Address) -> bool {
    addr >= 0 && addr < MAX_STACK_HEIGHT.try_into().unwrap()
  }

  pub fn initialize() -> Self {
    let new_stack = Stack { stack: [0; MAX_STACK_HEIGHT], sp: 0, bp: 0 };
    new_stack.stack_invariant();
    new_stack
  }

  pub fn size(&self) -> Address {
    self.sp
  }

  pub fn ar_base(&self) -> Address {
    self.bp
  }

  pub fn is_empty(&self) -> bool {
    self.sp == 0
  }

  pub fn is_full(&self) -> bool {
    self.sp == self.stack.len().try_into().unwrap()
  }

  pub fn push(&mut self, value: Word) {
    if self.is_full() {
      panic!("Error: Stack overflow! (SP: {}, Max: {})", self.sp, self.stack.len());
    }
    self.stack[self.sp as usize] = value;
    self.sp += 1;
  }

  pub fn allocate(&mut self, size: i32) {
    let new_sp: i32 = self.sp + size;
    if Stack::legal_stack_index(new_sp) {
      self.sp = new_sp;
    } else {
      panic!("Error: Cannot increase stack size by {}! (New SP: {}, Max: {})", size, new_sp, MAX_STACK_HEIGHT);
    }
    self.stack_invariant();
  }

  pub fn pop(&mut self) -> Word {
    if self.is_empty() {
      panic!("Error: Trying to pop an empty stack!");
    }
    self.sp -= 1;
    self.stack_invariant();
    self.stack[self.sp as usize]
  }

  pub fn peek(&self) -> Word {
    if self.is_empty() {
      panic!("Error: Trying to peek an empty stack!");
    }
    self.stack[(self.sp as usize) - 1]
  }

  pub fn fetch(&self, addr: Address) -> Word {
    if !Stack::legal_stack_index(addr) {
      panic!("Error: Illegal stack index {}!", addr);
    }
    self.stack[addr as usize]
  }

  pub fn assign(&mut self, addr: Address, value: Word) {
    if !Stack::legal_stack_index(addr) {
      panic!("Error: Illegal stack index {}!", addr);
    }
    self.stack[addr as usize] = value;
  }

  pub fn print_stack(&self) {
    for i in self.bp..self.sp {
      println!("S[{}]: {}", i, self.stack[i as usize]);
    }
  }
}