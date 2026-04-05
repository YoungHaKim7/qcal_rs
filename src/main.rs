use meval::{eval_str, eval_str_with_context, Context};
use std::io::{self, Write};

fn main() {
    println!("Qalculate CLI - Interactive Calculator");
    println!("Type 'exit' or 'quit' to exit\n");
    println!("Supported: sqrt(72), 2^3 + 5, sin(pi), 133 to hex, etc.");

    let mut context = Context::new();
    let mut last_result: Option<f64> = None;

    // Add common constants
    context.var("pi", std::f64::consts::PI);
    context.var("e", std::f64::consts::E);

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
            println!("Goodbye!");
            break;
        }

        if input.is_empty() {
            continue;
        }

        match evaluate_command(input, &context, last_result) {
            Ok(result) => {
                println!("{}", result);
                if let Some(num) = parse_result_number(&result) {
                    last_result = Some(num);
                    context.var("ans", num);
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

fn evaluate_command(input: &str, context: &Context, last_result: Option<f64>) -> Result<String, String> {
    let lower = input.to_lowercase();

    // Handle conversions: "<number> to hex|binary|octal"
    if lower.contains(" to ") {
        return handle_conversion(&lower);
    }

    // Replace 'ans' with last result in expression
    let expr = if let Some(ans) = last_result {
        lower.replace("ans", &ans.to_string())
    } else {
        lower
    };

    // Evaluate mathematical expression
    eval_expr_with_context(&expr, context)
}

fn handle_conversion(input: &str) -> Result<String, String> {
    let parts: Vec<&str> = input.split(" to ").collect();
    if parts.len() != 2 {
        return Err("Invalid conversion format".to_string());
    }

    let num: i128 = parts[0].parse().map_err(|_| "Invalid number".to_string())?;
    let target = parts[1].trim();

    match target {
        "hex" | "hexadecimal" => Ok(format!("0x{:X}", num)),
        "binary" | "bin" => Ok(format!("0b{:b}", num)),
        "octal" | "oct" => Ok(format!("0o{:o}", num)),
        _ => Err(format!("Unknown conversion target: {}", target)),
    }
}

fn eval_expr_with_context(expr: &str, context: &Context) -> Result<String, String> {
    match eval_str_with_context(expr, context) {
        Ok(result) => format_result(result),
        Err(_) => {
            // Try fallback without context (for expressions that might work with built-ins only)
            match eval_str(expr) {
                Ok(result) => format_result(result),
                Err(e) => Err(format!("{}", e)),
            }
        }
    }
}

fn format_result(result: f64) -> Result<String, String> {
    if result.is_nan() {
        Ok("NaN".to_string())
    } else if result.is_infinite() {
        Ok(if result.is_sign_positive() {
            "Infinity"
        } else {
            "-Infinity"
        }
        .to_string())
    } else if result.fract() == 0.0 && result.abs() < 1e15 {
        // Show as integer for whole numbers within range
        Ok(format!("{}", result as i64))
    } else {
        Ok(format!("{}", result))
    }
}

fn parse_result_number(result: &str) -> Option<f64> {
    if result == "NaN" || result == "Infinity" || result == "-Infinity" {
        return None;
    }
    // Skip hex/binary/octal prefixes
    let num_str = result
        .strip_prefix("0x")
        .or_else(|| result.strip_prefix("0b"))
        .or_else(|| result.strip_prefix("0o"))
        .unwrap_or(result);

    num_str.parse().ok()
}
