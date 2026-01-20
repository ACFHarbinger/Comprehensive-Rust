use exercise_collatz_sequence::functions::arithmetic::*;
use exercise_collatz_sequence::functions::blocks::*;
use exercise_collatz_sequence::functions::loops::*;
use exercise_collatz_sequence::functions::sequences::*;
use exercise_collatz_sequence::functions::types::*;
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
    if args.len() > 2 {
        first_arg = &args[1];
        second_arg = &args[2];
        println!("Category: {}, Function: {}", first_arg, second_arg);
    } else {
        println!("Not enough arguments provided.");
        return;
    }

    let result = match (first_arg.as_str(), second_arg.as_str()) {
        ("arithmetic" | "a", "interproduct" | "ip") => {
            println!(
                "The interproduct of 120, 100, 248 is {}",
                interproduct(120, 100, 248)
            );
            "Success"
        }
        ("arithmetic" | "a", "gcd" | "greatest_common_divisor") => {
            println!("The GCD of 120 and 100 is {}", gcd(120, 100));
            "Success"
        }
        ("types" | "t", "type_inf" | "type_inference" | "ti") => {
            let x = 10;
            let y = 20;
            takes_u32(x);
            takes_i8(y);
            //takes_u32(y);
            "Success"
        }
        ("sequences" | "s", "fibonacci" | "fib") => {
            println!("The 10th Fibonacci number is {}", fib(10));
            "Success"
        }
        (
            "sequences" | "s",
            "fib_iter" | "fibonacci_iter" | "fib_iterative" | "fibonacci_iterative" | "fi",
        ) => {
            println!("The 10th Fibonacci number is {}", fib_iter(10));
            "Success"
        }
        ("sequences" | "s", "collatz" | "col") => {
            println!("The 11th Collatz number is {}", collatz(11, 0));
            "Success"
        }
        ("sequences" | "s", "collatz_iter" | "collatz_iterative" | "ci") => {
            println!("The 11th Collatz number is {}", collatz_iter(11));
            "Success"
        }
        ("blocks" | "b", "block" | "block_and_scope" | "bas") => {
            block_and_scope();
            "Success"
        }
        ("blocks" | "b", "if" | "if_expression" | "ie") => {
            let x = 10;
            let size_str = if_expression(x);
            println!("{size_str}");
            "Success"
        }
        ("loops" | "l", "match" | "match_expression" | "me") => {
            let x = 10;
            let num_str = match_expression(x);
            println!("The number {x} is {num_str}");
            "Success"
        }
        ("loops" | "l", "break" | "break_label" | "bl") => {
            break_label();
            "Success"
        }
        ("loops" | "l", "while" | "while_loop" | "wl") => {
            let x = 200;
            while_loop(x);
            "Success"
        }
        ("loops" | "l", "for" | "for_loop" | "fl") => {
            for_loop();
            "Success"
        }
        ("loops" | "l", "loop" | "loop_loop" | "ll") => {
            loop_loop();
            "Success"
        }
        ("loops" | "l", "nested" | "nested_loop" | "nested_for" | "nested_for_loop" | "nfl") => {
            nested_for_loop();
            "Success"
        }
        (c, f) => {
            println!("Unknown combination: '{c}' and '{f}'");
            "Failure"
        }
    };
    println!("Result: {result}");
}
