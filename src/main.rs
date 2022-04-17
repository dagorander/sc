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
//       That will mkake so many things so much cleaner.

// FEATURE WISHLISH:
// Be able to store results into Memories for later display (or use?)
// Remember previous value and access it through arrow-up
// Support implied multiplication: 10/2(1+1)=10
// Support returning values and exiting directly if used with params
// --- probably with a flag to force integers?

fn main() {
    // TODO: check arguments and do something else if they exist
    // Start the main loop
    loop {
        // Take input
        let input: String = take_input();

        // Check input, exit loop if "quit", etc
        // Some switch/match/case instead of this soup?
        if input == "quit" || input == "exit" { 
            break
        };
        if input == "clear" { 
            clear_screen();
            continue
        };
        if input == "help" { 
            help_screen();
            continue 
        };
        if input == "info" { 
            info_screen();
            continue
        };

        // We now assume the input was mathematical and pass it for solving
        // TODO: create some form of error handling for this, eprintln! for err
        let r = meval::eval_str(&input).unwrap();
        println!("> {} = {}", input, r);        
    }

    process::exit(0);
}

// Still have to figure out why there's that 'static bs here...
fn take_input() -> String {
    let standard_user_input: &str = "";
    let input: String = same_line_input(standard_user_input.to_string());
    // This is where I should learn rust case/switch/etc. And see if there is
    // a way to implement "or" operators on if statements?
    // TODO: Can omit this if included in the main control flow through adding
    // stuff like if input == "hamster" || input == "h" {..}
    if input == "q" { return "quit".to_string() };
    if input == "c" { return "clear".to_string() };
    if input == "h" { return "help".to_string() };
    if input == "i" { return "info".to_string() };

    return input;
}

fn help_screen() {
    println!("\nSIMPLE CALCULATOR");
    println!("A totally not shit calculator by Turnip McTurnipface.");

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("I probably don't support arguments, yet. Working on that.");
        println!("Unless you see them working now, then welcome() needs updating.")
    }
}

// TODO: Figure out what the heck the 'static is all about.
// fn info_screen() -> &'static str { return "continue" }
// etc etc etc. For now, avoiding that through all the Strings... :P
fn info_screen() {
    println!("\nSIMPLE CALCULATOR");
    println!("the boring legal stuff? Or delete this info screen?")  
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
