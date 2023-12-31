pub mod machine;
pub mod lexer;
pub mod lexer_log;
pub mod token;
pub mod reserved_types;

use std::{env, process::exit};
//use machine::machine::start_machine;
use lexer::lexer_open;
fn main() {
    let args: Vec<String> = env::args().collect();

    let mut debug: bool = false;
    let mut trace: bool = false;

    if args.len() < 2 {
        println!("Usage: `rustpl0 <filename> [-d | -debug] [-t | -trace]`");
        exit(0);
    }

    if args.len() > 2 {
      for i in 2..args.len() {
        match args[i].as_str() {
          "-d" | "-debug" => {
            debug = true;
          },
          "-t" | "-trace" => {
            trace = true;
          },
          _ => {
            println!("Usage: `rustpl0 <filename> [-d | -debug] [-t | -trace]`");
            exit(0);
          }
        }
      }
    }
    
    let filepath = &args[1];
    //start_machine(&filepath, debug, trace);
    lexer_open(&filepath, debug);
}