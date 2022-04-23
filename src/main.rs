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
// --- Optionally: make it possible to save Memoried values to a text file
// Remember previous value and access it through arrow-up
// Support implied multiplication: 10/2(1+1)=10
// Give correct error codes on bad exit
// Make repeating numbers (like 13/3=4.3333333333333333) a bit more sane
// --- potentially make precision controllable with a flag and option
// --- which also implies creating an options screen to set things like
// ------- decimal precision, force integer

fn main() {
    // Check arguments and do something else if they exist
    check_for_argument();
    // We didn't exit by now, means we are in interactive mode
    interactive_mode();
    process::exit(0);
}

fn check_for_argument() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match meval::eval_str(args[1].as_str()) {
            Ok(r) => {
                println!("{}", r.to_string());
                process::exit(0);
            }
            Err(_e) => {
                match args[1].as_str() {
                    "h" | "help" | "--help" => { 
                        help_screen();
                        process::exit(0);
                    },
                    _ => { eprintln!("ERR: use --help or a math expression");
                        process::exit(1);
                    }
                }
            }
        }
    }
}

fn interactive_mode() {
    loop {
        // Take input
        let input: String = take_input();

        // Check input, exit loop if "quit", etc
        match input.as_str() {
            "quit" | "q" | "exit" | "e" => { break },
            "c" | "clear" => { clear_screen(); continue },
            "h" | "help" => { help_screen(); continue },
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
}

fn take_input() -> String {
    let standard_user_input: &str = "";
    let input: String = same_line_input(standard_user_input.to_string());
    return input;
}

fn help_screen() {
    println!("\nSIMPLE CALCULATOR");
    println!("A totally not shit calculator by Turnip McTurnipface.");
}

// TODO: Figure out what the heck the 'static is all about.
// fn info_screen() -> &'static str { return "continue" }
// etc etc etc. For now, avoiding that through all the Strings... :P

fn same_line_input(prompt: String) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap(); // Why need unwrap to silence warning?
        // Maybe it should use a real error handling instead?
    let input: String = text_io::read!("{}\n");
    return input;
}

fn clear_screen() {
    // Because I will forget this import without something super simple
    clearscreen::clear().expect("failed to clear screen");
}
