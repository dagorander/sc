// A Simple Calculator
// Be like bc, or something, but not shit at stuff like
// 164/9 and 199/3

use std::env;
use std::process;
use std::io::Write; // Needed to force Print buffer in spite of no newline?

fn main() {
    welcome();
    main_screen();

    process::exit(0);
}

fn welcome() {
    clear_screen();
    println!("\n\nSIMPLE CALCULATOR\n\n");
    println!("A totally not shit calculator by Turnip McTurnipface.");

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("I probably don't support arguments, yet. Working on that.");
        println!("Unless you see them working now, then welcome() needs updating.")
    }
}

fn main_screen() {
    let default_input: &str = "This is where you enter text: ";
    let purpose: String = same_line_input(default_input.to_string());
    println!("You entered: {}", purpose);

    let test_thingie: &str = "This string tests
        multiline input
        to figure out if that works.
        
        It automatically includes newlines. :(";
    println!("{}", test_thingie);

}

fn same_line_input(text: String) -> String {
    print!("{}", text);
    std::io::stdout().flush(); // Flush buffer without newline, but why complaining?
    let input: String = text_io::read!("{}\n");
    return input;
}

fn clear_screen() {
    // Because I will forget this import without something super simple
    clearscreen::clear().expect("failed to clear screen");
}
