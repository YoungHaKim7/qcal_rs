use qalculate::{gcd, is_prime, lcm, next_prime, totient};
use rug::Integer;
use std::io::{self, Write};

fn main() {
    println!("Qalculate CLI - Interactive Calculator");
    println!("Type 'exit' or 'quit' to exit\n");

    let mut last_result: Option<f64> = None;

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        // Exit commands
        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
            println!("Goodbye!");
            break;
        }

        // Empty input - skip
        if input.is_empty() {
            continue;
        }

        // Simple expression evaluation
        match evaluate_expression(input, last_result) {
            Ok(result) => {
                println!("= {}", result);
                last_result = Some(result);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

fn evaluate_expression(expr: &str, last_result: Option<f64>) -> Result<f64, String> {
    // Replace 'ans' with last result
    let mut expr = expr.to_string();
    if let Some(ans) = last_result {
        expr = expr.replace("ans", &ans.to_string());
    }

    // Simple arithmetic evaluation
    // This is a basic implementation - for more complex expressions,
    // you would want to use a proper expression parser
    let tokens: Vec<&str> = expr.split_whitespace().collect();
    let mut result = 0.0;
    let mut current_op = '+';

    for token in tokens {
        if let Ok(num) = token.parse::<f64>() {
            match current_op {
                '+' => result += num,
                '-' => result -= num,
                '*' => result *= num,
                '/' => {
                    if num == 0.0 {
                        return Err("Division by zero".to_string());
                    }
                    result /= num;
                }
                _ => {}
            }
        } else {
            match token {
                "+" => current_op = '+',
                "-" => current_op = '-',
                "*" | "x" => current_op = '*',
                "/" => current_op = '/',
                _ => {
                    return Err(format!("Unknown token: {}", token));
                }
            }
        }
    }

    Ok(result)
}
