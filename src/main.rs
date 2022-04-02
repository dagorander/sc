// A Simple Calculator
//
// Try to be like bc but be less integery about things like
// 164/9 and 199/3

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    // NOTE: 1st arg is the name we were invoked with
    // TODO: Make this call a --help equivalent output
    if args.len() == 1 {
        println!("Arguments needed.");
        process::exit(1);
    }

}
