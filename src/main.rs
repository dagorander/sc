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
// Make info and help screens not different

// FEATURE WISHLISH:
// Be able to store results into Memories for later display (or use?)
// Remember previous value and access it through arrow-up
// Support implied multiplication: 10/2(1+1)=10
// Support returning values and exiting directly if used with params
// --- probably with a flag to force integers?
// Give correct error codes on bad exits

fn main() {
    // TODO: check arguments and do something else if they exist
    // Start the main loop
    loop {
        // Take input
        let input: String = take_input();

        // Check input, exit loop if "quit", etc
        match input.as_str() {
            "quit" | "q" | "exit" | "e" => { break },
            "c" | "clear" => { clear_screen(); continue },
            "h" | "help" => { help_screen(); continue },
            "i" | "info" => { info_screen(); continue },
            // We now assume the input was mathematical and pass it for solving
            // with a little check to not crash if input was invalid.
            _ => match meval::eval_str(&input) {
                Ok(r) => {
                    println!("> {} = {}", input, r.to_string());
                }
                Err(_e) => {
                    eprintln!("ERR: unable to resolve expression")
                }
            }
        };

    }

    process::exit(0);
}

// Still have to figure out why there's that 'static bs here...
fn take_input() -> String {
    let standard_user_input: &str = "";
    let input: String = same_line_input(standard_user_input.to_string());
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
        // Maybe it should use a real error handling instead?
    let input: String = text_io::read!("{}\n");
    return input;
}

fn clear_screen() {
    // Because I will forget this import without something super simple
    clearscreen::clear().expect("failed to clear screen");
}
