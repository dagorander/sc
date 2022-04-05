// A Simple Calculator
// Be like bc, or something, but not shit at stuff like
// 164/9 and 199/3

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    // NOTE: 1st arg is the name we were invoked with
    // TODO: Make this call a --help equivalent output
    if args.len() >= 2 {
        println!("I don't support any arguments yet");
    }
    println!("This is just a test string before taking input.");
    let texty_text: String = text_io::read!("{}\n");    
    clear_screen();
    println!("You wrote:\n");
    println!("{}", texty_text);
    // Need to find a way to validate inputs are numbers below...
    let number_text_1: i32 = text_io::read!();
    let number_text_2: i32 = text_io::read!();
    println!("You wrote {} and {}", number_text_1, number_text_2);

    process::exit(0);
}


fn clear_screen() {
    // Because I will forget this import without something super simple
    clearscreen::clear().expect("failed to clear screen");
}
