// A Simple Calculator
// Be like bc, or something, but not shit at stuff like
// 164/9 and 199/3

use std::env;
use std::process;
use std::io::Write; // Needed to force Print buffer in spite of no newline?

// Possible tools and crates:
// https://crates.io/crates/mexprp - math eval from string, but data types weird
// Also https://crates.io/crates/meval


// TODO: Create a data file for strings
//       That will mkake so many things so much cleaner

fn main() {
    welcome_screen();

    // Rudimentary decision loop
    loop {
        let choice: &str = info_screen();
        // Also probably want a case/switch here...
        if choice == "quit" { break }
        if choice == "hamster" { println!("Hamsters are qute.") }
        if choice == "continue" { continue }
        if choice == "math" { math_parser() }
        if choice == "clear" { clear_screen() }
    }

    process::exit(0);
}

fn welcome_screen() {
    clear_screen();
    println!("\n\nSIMPLE CALCULATOR\n\n");
    println!("A totally not shit calculator by Turnip McTurnipface.");

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("I probably don't support arguments, yet. Working on that.");
        println!("Unless you see them working now, then welcome() needs updating.")
    }
}

fn math_parser() {
    loop {
        let prompt: &str = "> ";
        let input: String = same_line_input(prompt.to_string());
        // Need "or" operands for this below too
        if input == "q" { break }


        let r = meval::eval_str(&input).unwrap();
        println!("{} = {}", &input, r);
    }
}

// TODO: Figure out what the fuck the 'static is all about.
fn info_screen() -> &'static str {
    println!("#\n# This is the info screen\n#");
    println!("Type q to exit, or m to do math");
    let standard_user_input: &str = ">>> ";
    let selection: String = same_line_input(standard_user_input.to_string());
    // This is where I should learn rust case/switch/etc. And see if there is
    // a way to implement "or" operators on if statements?
    if selection == "q" { return "quit" };
    if selection == "quit" { return "quit" };
    if selection == "h" { return "hamster" };
    if selection == "math" { return "math" };
    if selection == "m" { return "math" };
    if selection == "c" { return "clear" };
    return "continue";    
}

fn same_line_input(text: String) -> String {
    print!("{}", text);
    std::io::stdout().flush().unwrap(); // Why need unwrap to silence warning?
    let input: String = text_io::read!("{}\n");
    return input;
}

fn clear_screen() {
    // Because I will forget this import without something super simple
    clearscreen::clear().expect("failed to clear screen");
}
