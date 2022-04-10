// A Simple Calculator
// Be like bc, or something, but not shit at stuff like
// 164/9 and 199/3

use std::env;
use std::process;
use std::io::Write; // Needed to force Print buffer in spite of no newline

fn main() {
    // Startup busywork, later check for --help etc
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        println!("I don't support any arguments yet");
    }

    main_screen();

    process::exit(0);
}

fn main_screen() {
    // Due to the flushing needed with print without newline we should
    // probably bundle the prints and the std::io::stdout().flush(); in
    // a separate function to keep things sane

    clear_screen();
    println!("\n\nSIMPLE CALCULATOR\n\n");
    print!("State your purpose: ");
    std::io::stdout().flush();  // TODO: "unused Result that must be used"?SS
    let purpose: String = take_input();
    println!("You want to {}", purpose);
}

fn take_input() -> String {
    // Look into the whole "try_read" macro?
    let input: String = text_io::read!("{}\n");
    return input;
}


fn clear_screen() {
    // Because I will forget this import without something super simple
    clearscreen::clear().expect("failed to clear screen");
}
