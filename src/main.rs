pub mod machine;

use std::{env, process::exit};
use machine::machine::start_machine;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: `rustpl0 <filename>`");
        exit(0);
    }

    let filepath = &args[1];
    start_machine(&filepath);
}
