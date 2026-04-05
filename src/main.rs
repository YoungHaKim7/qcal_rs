use meval::{eval_str, eval_str_with_context, Context};
use rustyline::DefaultEditor;

fn main() -> rustyline::Result<()> {
    println!("Qalculate CLI - Interactive Calculator");
    println!("Type 'exit' or 'quit' to exit\n");
    println!("Supported: sqrt(72), 2^3 + 5, sin(pi), 133 to hex, etc.");

    let mut context = Context::new();
    let mut last_result: Option<f64> = None;
    let mut rl = DefaultEditor::new()?;

    // Add common constants
    context.var("pi", std::f64::consts::PI);
    context.var("e", std::f64::consts::E);

    loop {
        let input = rl.readline("> ");
        let input = match input {
            Ok(line) => line,
            Err(_) => break,
        };
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

        // Add non-empty input to history
        if !input.is_empty() {
            let _ = rl.add_history_entry(input);
        }
    }

    Ok(())
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

    // Pre-process shift operators
    let expr = preprocess_shift_operators(&expr)?;

    // Evaluate mathematical expression
    eval_expr_with_context(&expr, context)
}

fn preprocess_shift_operators(expr: &str) -> Result<String, String> {
    let mut result = expr.to_string();

    // Process << and >> operators by finding and replacing them with computed values
    loop {
        let left_shift_pos = result.find("<<");
        let right_shift_pos = result.find(">>");

        if left_shift_pos.is_none() && right_shift_pos.is_none() {
            break;
        }

        // Determine which operator comes first
        let (pos, is_left_shift) = match (left_shift_pos, right_shift_pos) {
            (Some(l), Some(r)) if l < r => (l, true),
            (Some(_l), Some(r)) => (r, false),
            (Some(l), None) => (l, true),
            (None, Some(r)) => (r, false),
            _ => break,
        };

        // Find left operand (search backwards for operator boundaries)
        let left_end = pos;
        let left_start = find_operand_start(&result, left_end);
        let left_expr = &result[left_start..left_end];

        // Find right operand (search forwards for operator boundaries)
        let right_start = pos + 2; // << or >> is 2 chars
        let right_end = find_operand_end(&result, right_start);
        let right_expr = &result[right_start..right_end];

        // Evaluate operands
        let left_val: i64 = eval_str(left_expr)
            .map_err(|e| format!("Failed to evaluate left operand '{}': {}", left_expr, e))?
            as i64;
        let right_val: i32 = eval_str(right_expr)
            .map_err(|e| format!("Failed to evaluate right operand '{}': {}", right_expr, e))?
            as i32;

        // Compute shift result
        let shift_result = if is_left_shift {
            left_val << right_val
        } else {
            left_val >> right_val
        };

        // Replace the shift expression with the result
        result.replace_range(left_start..right_end, &shift_result.to_string());
    }

    Ok(result)
}

fn find_operand_start(s: &str, operand_end: usize) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let mut pos = if operand_end > 0 { operand_end - 1 } else { 0 };
    let mut paren_depth = 0;
    let mut found_non_space = false;

    while pos > 0 {
        match chars[pos] {
            ')' => paren_depth += 1,
            '(' => {
                if paren_depth == 0 {
                    return pos;
                }
                paren_depth -= 1;
            }
            ' ' | '\t' if !found_non_space && paren_depth == 0 => {
                pos -= 1;
                continue;
            }
            c if is_operator_char(c) && paren_depth == 0 && found_non_space => return pos + 1,
            ' ' | '\t' if found_non_space && paren_depth == 0 => return pos + 1,
            _ => {
                found_non_space = true;
            }
        }
        pos -= 1;
    }
    0
}

fn find_operand_end(s: &str, op_start: usize) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let mut pos = op_start;
    let mut paren_depth = 0;
    let mut found_non_space = false;

    while pos < chars.len() {
        match chars[pos] {
            '(' => paren_depth += 1,
            ')' => {
                if paren_depth == 0 {
                    return pos;
                }
                paren_depth -= 1;
            }
            ' ' | '\t' if !found_non_space && paren_depth == 0 => {
                pos += 1;
                continue;
            }
            c if is_operator_char(c) && paren_depth == 0 && found_non_space => return pos,
            ' ' | '\t' if found_non_space && paren_depth == 0 => return pos,
            _ => {
                found_non_space = true;
            }
        }
        pos += 1;
    }
    chars.len()
}

fn is_operator_char(c: char) -> bool {
    matches!(c, '+' | '-' | '*' | '/' | '%' | '^' | '<' | '>' | '=' | '!')
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
        "binary" | "bin" => {
            let binary_str = format!("{:b}", num);
            // Pad with leading zeros to make length a multiple of 4
            let padding = (4 - binary_str.len() % 4) % 4;
            let padded = format!("{}{}", "0".repeat(padding), binary_str);
            let spaced: String = padded
                .chars()
                .rev()
                .collect::<Vec<_>>()
                .chunks(4)
                .map(|chunk| chunk.iter().collect::<String>())
                .collect::<Vec<_>>()
                .join(" ")
                .chars()
                .rev()
                .collect();
            Ok(format!("0b{}", spaced))
        }
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
