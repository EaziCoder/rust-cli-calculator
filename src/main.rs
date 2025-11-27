use std::io::{self, Write};

use crate::{
    calculator::{Calculator, compute},
    parse::parse_command,
};

mod calculator;

mod parse;

fn main() {
    let mut calculator = Calculator {
        history: Vec::new(),
    };

    if let Err(_) = calculator.load_history() {
        println!("No previous history found");
    }
    // Automatically load history at startup
    // if let Err(_) is shorthand when you only care about one variant.

    println!("Welcome to RustCalc");
    println!("Type an operation like: add 2 3");
    println!("Type 'quit', to exit program. \n");

    loop {
        let mut calculation = String::new();

        print!("> ");
        io::stdout().flush().unwrap(); // ensurers the print! prompt prints before calculation

        io::stdin()
            .read_line(&mut calculation)
            .expect("Failed to read input");

        let calculation = calculation.trim();
        if calculation.is_empty() {
            continue;
        }

        if calculation.eq_ignore_ascii_case("quit") || calculation.eq_ignore_ascii_case("exit") {
            println!("Goodbye");
            let _ = calculator.save_history();
            // Automatically save history on exit
            break;
        }

        if calculation.eq_ignore_ascii_case("history") {
            println!("{}", calculator.show_history());
            continue;
        }

        if calculation.eq_ignore_ascii_case("clear") {
            calculator.clear_history();
            println!("History cleared.\n");
            continue;
        }

        if calculation.eq_ignore_ascii_case("save") {
            match calculator.save_history() {
                Ok(_) => println!("History saved to history.txt.\n"),
                Err(e) => println!("Error saving history: {e:?}"),
            }
            continue;
        }

        if calculation.eq_ignore_ascii_case("load") {
            match calculator.load_history() {
                Ok(_) => println!("History loaded from history.txt.\n"),
                Err(e) => println!("Error loading history: {e:?}"),
            }
            continue;
        }

        if calculation.eq_ignore_ascii_case("help") {
            println!(
                "Available commands: \n
                add a b -> Adds two numbers \n
                sub a b -> Subtracts b from a \n
                mul a b -> Multiply \n
                div a b -> Divide \n
                pow a b -> a raised to b \n
                sqrt a -> Square root of a \n
                abs a -> Absolute value \n
                neg a -> Negate \n
                history -> Show calculation history \n
                clear -> Clear history \n
                save -> Save history to file \n
                load -> Load history from file \n
                quit/exit -> Exit calculator \n
                help -> Show this help message \n"
            );
            continue;
        }

        let user_line = match parse_command(calculation) {
            Ok(value) => value,
            Err(error) => {
                println!("Error: {error:?}");
                continue;
            }
        };

        let (op, a, b) = user_line;

        let result = match compute(op, a, b) {
            Ok(value) => value,
            Err(error) => {
                println!("Math Error: {error:?}");
                continue;
            }
        };

        calculator.record(op, a, b, result);
        println!("Result = {result}");
    }
}
