use std::env;
use std::process;

use minigrep::Config;
// Separation of Concerns for Binary Projects
// This pattern is about separating concerns: main.rs handles running the program, and lib.rs handles all the logic of the task at hand.
// Because you can’t test the main function directly, this structure lets you test all of your program’s logic by moving it into functions in lib.rs.
// The only code that remains in main.rs will be small enough to verify its correctness by reading it. Let’s rework our program by following this process.

fn main() {
    let args: Vec<String> = env::args().collect();
    // collect: Transforms an iterator into a collection.

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        // Writing error messages to standard error instead of standard output using eprintln!

        process::exit(1);
    }
    // Why use if let rather than unwrap_or_else?
    // Because run returns () in the success case, we only care about detecting an error, so we don’t need unwrap_or_else to return the unwrapped value because it would only be ().
}
