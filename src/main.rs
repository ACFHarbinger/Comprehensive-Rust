use day_one_afternoon::commands::arrays::*;
use day_one_afternoon::commands::patterns::*;
use day_one_afternoon::commands::tuples::*;
use day_one_morning::functions::arithmetic::*;
use day_one_morning::functions::blocks::*;
use day_one_morning::functions::loops::*;
use day_one_morning::functions::sequences::*;
use day_one_morning::functions::types::*;
use std::env;
use std::string;

fn main() {
    // Collect arguments into a Vector of Strings
    let args: Vec<String> = env::args().collect();

    // The first argument (index 0) is always the path to the executable
    let program_name = &args[0];
    println!("Program: {program_name}");

    let first_arg: &string::String;
    let second_arg: &string::String;
    let third_arg: &string::String;
    if args.len() > 3 {
        first_arg = &args[1];
        second_arg = &args[2];
        third_arg = &args[3];
        println!(
            "Crate: {}, File: {}, Function: {}",
            first_arg, second_arg, third_arg
        );
    } else {
        println!("Not enough arguments provided.");
        return;
    }

    let result = match (first_arg.as_str(), second_arg.as_str(), third_arg.as_str()) {
        ("day_one_morning" | "dom", "arithmetic" | "a", "interproduct" | "ip") => {
            println!(
                "The interproduct of 120, 100, 248 is {}",
                interproduct(120, 100, 248)
            );
            "Success"
        }
        ("day_one_morning" | "dom", "arithmetic" | "a", "gcd" | "greatest_common_divisor") => {
            println!("The GCD of 120 and 100 is {}", gcd(120, 100));
            "Success"
        }
        ("day_one_morning" | "dom", "types" | "t", "type_inf" | "type_inference" | "ti") => {
            let x = 10;
            let y = 20;
            takes_u32(x);
            takes_i8(y);
            //takes_u32(y);
            "Success"
        }
        ("day_one_morning" | "dom", "sequences" | "s", "fibonacci" | "fib") => {
            println!("The 10th Fibonacci number is {}", fib(10));
            "Success"
        }
        (
            "day_one_morning" | "dom",
            "sequences" | "s",
            "fib_iter" | "fibonacci_iter" | "fib_iterative" | "fibonacci_iterative" | "fi",
        ) => {
            println!("The 10th Fibonacci number is {}", fib_iter(10));
            "Success"
        }
        ("day_one_morning" | "dom", "sequences" | "s", "collatz" | "col") => {
            println!("The 11th Collatz number is {}", collatz(11, 0));
            "Success"
        }
        (
            "day_one_morning" | "dom",
            "sequences" | "s",
            "collatz_iter" | "collatz_iterative" | "ci",
        ) => {
            println!("The 11th Collatz number is {}", collatz_iter(11));
            "Success"
        }
        ("day_one_morning" | "dom", "blocks" | "b", "block" | "block_and_scope" | "bas") => {
            block_and_scope();
            "Success"
        }
        ("day_one_morning" | "dom", "blocks" | "b", "if" | "if_expression" | "ie") => {
            let x = 10;
            let size_str = if_expression(x);
            println!("{size_str}");
            "Success"
        }
        ("day_one_morning" | "dom", "loops" | "l", "match" | "match_expression" | "me") => {
            let x = 10;
            let num_str = match_expression(x);
            println!("The number {x} is {num_str}");
            "Success"
        }
        ("day_one_morning" | "dom", "loops" | "l", "break" | "break_label" | "bl") => {
            break_label();
            "Success"
        }
        ("day_one_morning" | "dom", "loops" | "l", "while" | "while_loop" | "wl") => {
            let x = 200;
            while_loop(x);
            "Success"
        }
        ("day_one_morning" | "dom", "loops" | "l", "for" | "for_loop" | "fl") => {
            for_loop();
            "Success"
        }
        ("day_one_morning" | "dom", "loops" | "l", "loop" | "loop_loop" | "ll") => {
            loop_loop();
            "Success"
        }
        (
            "day_one_morning" | "dom",
            "loops" | "l",
            "nested" | "nested_loop" | "nested_for" | "nested_for_loop" | "nfl",
        ) => {
            nested_for_loop();
            "Success"
        }
        ("day_one_afternoon" | "doa", "arrays" | "a" | "arr", "mutable_arrays" | "ma") => {
            mutable_arrays();
            "Success"
        }
        ("day_one_afternoon" | "doa", "arrays" | "a" | "arr", "immutable_arrays" | "ia") => {
            immutable_arrays();
            "Success"
        }
        ("day_one_afternoon" | "doa", "arrays" | "a" | "arr", "array_iteration" | "ai") => {
            array_iteration();
            "Success"
        }
        ("day_one_afternoon" | "doa", "tuples" | "t" | "tup", "mutable_tuples" | "mt") => {
            mutable_tuples();
            "Success"
        }
        ("day_one_afternoon" | "doa", "tuples" | "t" | "tup", "immutable_tuples" | "it") => {
            immutable_tuples();
            "Success"
        }
        ("day_one_afternoon" | "doa", "patterns" | "p" | "pat", "pattern_destructuring" | "pd") => {
            pattern_destructuring();
            "Success"
        }
        ("day_one_afternoon" | "doa", "patterns" | "p" | "pat", "refutable_pattern" | "rp") => {
            println!("{}", refutable_pattern());
            "Success"
        }
        ("day_one_afternoon" | "doa", "arrays" | "a" | "arr", "nested_arrays" | "na") => {
            nested_arrays();
            "Success"
        }
        (c, file, func) => {
            println!("Unknown combination: '{c}' and '{file}' and '{func}'");
            "Failure"
        }
    };
    println!("Result: {result}");
}
