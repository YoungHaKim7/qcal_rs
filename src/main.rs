use meval::{Context, eval_str, eval_str_with_context};
use rustyline::DefaultEditor;

mod fprice_standalone;
use fprice_standalone::fprice;

fn format_binary_64bit(value: i64) -> String {
    // Get 64-bit binary representation
    let bits = format!("{:064b}", value);

    // Split into upper and lower 32 bits
    let upper = &bits[0..32];
    let lower = &bits[32..64];

    // Format each half with double spaces between 4-bit groups
    let upper_formatted: String = upper
        .chars()
        .collect::<Vec<_>>()
        .chunks(4)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("  ");

    let lower_formatted: String = lower
        .chars()
        .collect::<Vec<_>>()
        .chunks(4)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("  ");

    format!(
        "{}\n63                      47                  32\n\n{}\n31                      15                   0",
        upper_formatted, lower_formatted
    )
}

fn evaluate_command(
    input: &str,
    context: &Context,
    last_result: Option<f64>,
) -> Result<(String, Option<f64>), String> {
    let lower = input.to_lowercase();

    // Replace 'ans' with last result in expression
    let expr = if let Some(ans) = last_result {
        lower.replace("ans", &ans.to_string())
    } else {
        lower.clone()
    };

    // Check for conversion suffix: "<expression> to <format>"
    if let Some((expr_part, format_part)) = extract_conversion(&expr) {
        // Process the expression (binary conversion + bitwise ops)
        let processed_expr = preprocess_operators(&expr_part)?;

        // Evaluate the expression
        let result: i64 = eval_str(&processed_expr)
            .map_err(|e| format!("Failed to evaluate expression: {}", e))?
            as i64;

        // Convert the result to the requested format
        return convert_result(result, &format_part).map(|s| (s, Some(result as f64)));
    }

    // Pre-process all operators including bitwise
    let expr = preprocess_operators(&expr)?;

    // Evaluate mathematical expression
    eval_expr_with_context(&expr, context)
}

fn extract_conversion(input: &str) -> Option<(String, String)> {
    if let Some(pos) = input.find(" to ") {
        let expr_part = input[..pos].trim().to_string();
        let format_part = input[pos + 4..].trim().to_string();
        return Some((expr_part, format_part));
    }
    None
}

fn convert_result(value: i64, format: &str) -> Result<String, String> {
    match format {
        "hex" | "hexadecimal" => Ok(format!("0x{:X}", value)),
        "binary" | "bin" => {
            let binary_str = format!("{:b}", value);
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
        "bin4" => {
            // 4-bit binary output
            Ok(format!("{:04b}", value as u8 & 0xF))
        }
        "bin8" => {
            // 8-bit binary output with space in the middle
            Ok(format!(
                "{:04b} {:04b}",
                (value as u8 >> 4) & 0xF,
                value as u8 & 0xF
            ))
        }
        "octal" | "oct" => Ok(format!("0o{:o}", value)),
        _ => Err(format!("Unknown conversion target: {}", format)),
    }
}

fn process_power_operator(expr: &str) -> Result<String, String> {
    let mut result = expr.to_string();

    // Process ^ operator (power) by finding and replacing them with pow() function calls
    loop {
        // Find ^ but not ^^ (which is XOR)
        let mut power_pos = None;
        let chars: Vec<char> = result.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            if chars[i] == '^' {
                // Check if this is ^^ (XOR, not power)
                if i + 1 < chars.len() && chars[i + 1] == '^' {
                    i += 2; // Skip ^^
                    continue;
                }
                // This is a single ^ = power operator
                power_pos = Some(i);
                break;
            }
            i += 1;
        }

        let pos = match power_pos {
            Some(p) => p,
            None => break,
        };

        // Find left operand
        let left_end = pos;
        let left_start = find_operand_start(&result, left_end);
        let left_expr = result[left_start..left_end].trim();

        // Find right operand
        let right_start = pos + 1;
        let right_end = find_operand_end(&result, right_start);
        let right_expr = result[right_start..right_end].trim();

        if left_expr.is_empty() || right_expr.is_empty() {
            break;
        }

        // Evaluate operands
        let left_val: f64 = eval_str(left_expr)
            .map_err(|e| format!("Failed to evaluate left operand '{}': {}", left_expr, e))?;
        let right_val: f64 = eval_str(right_expr)
            .map_err(|e| format!("Failed to evaluate right operand '{}': {}", right_expr, e))?;

        // Compute power result
        let power_result = left_val.powf(right_val);

        // Replace the power expression with the result
        result.replace_range(left_start..right_end, &power_result.to_string());
    }

    Ok(result)
}

fn preprocess_operators(expr: &str) -> Result<String, String> {
    let mut result = expr.to_string();

    // Replace unicode operators with ASCII equivalents
    result = result.replace('¬', "~");
    result = result.replace('∨', "|");
    result = result.replace('∧', "&");
    result = result.replace('⊻', "^");

    // Replace textual XOR operators (use ^^ for XOR since ^ is now power)
    result = result.replace("xor", "^^");
    // Replace ^^ with ^ for XOR processing (^^ is XOR, ^ is power)
    result = result.replace("^^", "^^");

    // Convert ** to ^ for power (we'll process ^ as power later)
    result = result.replace("**", "^");

    // Convert binary literals (0b...) to decimal
    result = convert_binary_literals(&result)?;

    // Process power operator (^) - highest precedence
    result = process_power_operator(&result)?;

    // Process NOT operator (~) - has highest precedence after power
    result = process_not_operator(&result)?;

    // Process AND operator (&) - highest precedence after NOT
    result = process_binary_operator("&", &result, |a, b| a & b)?;

    // Process XOR operator (^^) - medium precedence (^^ is XOR, ^ is power)
    result = process_binary_operator("^^", &result, |a, b| a ^ b)?;

    // Process OR operator (|) - lowest precedence
    result = process_binary_operator("|", &result, |a, b| a | b)?;

    // Process shift operators (<< and >>)
    result = preprocess_shift_operators(&result)?;

    Ok(result)
}

fn convert_binary_literals(expr: &str) -> Result<String, String> {
    let mut result = expr.to_string();
    let mut pos = 0;

    while pos < result.len() {
        // Find "0b" prefix
        if let Some(pb_start) = result[pos..].find("0b") {
            let abs_pb_start = pos + pb_start;
            let binary_start = abs_pb_start + 2;

            // Find the end of the binary literal
            let mut binary_end = binary_start;
            let chars: Vec<char> = result.chars().collect();
            let mut has_valid_digit = false;

            while binary_end < chars.len() {
                let c = chars[binary_end];
                if c == '0' || c == '1' {
                    has_valid_digit = true;
                    binary_end += 1;
                } else if c == ' ' {
                    // Allow spaces in binary literals for readability
                    binary_end += 1;
                } else {
                    break;
                }
            }

            if has_valid_digit {
                // Extract the binary string (removing spaces)
                let binary_str: String = result[binary_start..binary_end]
                    .chars()
                    .filter(|c| *c != ' ')
                    .collect();

                // Parse and convert to decimal
                if let Ok(value) = i64::from_str_radix(&binary_str, 2) {
                    result.replace_range(abs_pb_start..binary_end, &value.to_string());
                    pos = abs_pb_start + value.to_string().len();
                    continue;
                }
            }
        }
        pos += 1;
    }

    Ok(result)
}

fn process_not_operator(expr: &str) -> Result<String, String> {
    let mut result = expr.to_string();

    // Process ~ operator by finding and replacing them with computed values
    loop {
        let not_pos = result.find('~');

        if not_pos.is_none() {
            break;
        }

        let pos = not_pos.unwrap();

        // Find right operand (search forwards for operator boundaries)
        let right_start = pos + 1; // ~ is 1 char
        let right_end = find_operand_end(&result, right_start);
        let right_expr = result[right_start..right_end].trim();

        if right_expr.is_empty() {
            break;
        }

        // Evaluate operand
        let right_val: i64 = eval_str(right_expr)
            .map_err(|e| format!("Failed to evaluate operand '{}': {}", right_expr, e))?
            as i64;

        // Compute NOT result
        let not_result = !right_val;

        // Replace the NOT expression with the result
        result.replace_range(pos..right_end, &not_result.to_string());
    }

    Ok(result)
}

fn process_binary_operator<F>(op: &str, expr: &str, op_func: F) -> Result<String, String>
where
    F: Fn(i64, i64) -> i64,
{
    let mut result = expr.to_string();

    loop {
        let op_pos = result.find(op);

        if op_pos.is_none() {
            break;
        }

        let pos = op_pos.unwrap();

        // Find left operand
        let left_end = pos;
        let left_start = find_operand_start(&result, left_end);
        let left_expr = result[left_start..left_end].trim();

        // Find right operand
        let right_start = pos + op.len();
        let right_end = find_operand_end(&result, right_start);
        let right_expr = result[right_start..right_end].trim();

        if left_expr.is_empty() || right_expr.is_empty() {
            break;
        }

        // Evaluate operands
        let left_val: i64 = eval_str(left_expr)
            .map_err(|e| format!("Failed to evaluate left operand '{}': {}", left_expr, e))?
            as i64;
        let right_val: i64 = eval_str(right_expr)
            .map_err(|e| format!("Failed to evaluate right operand '{}': {}", right_expr, e))?
            as i64;

        // Compute result
        let op_result = op_func(left_val, right_val);

        // Replace the expression with the result
        result.replace_range(left_start..right_end, &op_result.to_string());
    }

    Ok(result)
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
    matches!(
        c,
        '+' | '-' | '*' | '/' | '%' | '^' | '<' | '>' | '=' | '!' | '~' | '|' | '&'
    )
}

fn eval_expr_with_context(expr: &str, context: &Context) -> Result<(String, Option<f64>), String> {
    match eval_str_with_context(expr, context) {
        Ok(result) => {
            let formatted = format_result(result)?;
            Ok((formatted, Some(result)))
        }
        Err(_) => {
            // Try fallback without context (for expressions that might work with built-ins only)
            match eval_str(expr) {
                Ok(result) => {
                    let formatted = format_result(result)?;
                    Ok((formatted, Some(result)))
                }
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
        // Show as integer for whole numbers within range, with comma separators
        Ok(fprice(result as i64))
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

/// Strip prefixes (0x, 0b, 0o) from formatted result strings and return only the number part.
/// Also removes spaces from binary representations.

// fn print_val(x: Result<String, String>) -> String {
//     let raw_data = x.unwrap_or_else(|e| e);
//     let raw_data_len = &raw_data.len() - 1;

//     let result_data = &raw_data[1..raw_data_len];
//     result_data.to_string()
// }

fn print_val(x: Result<String, String>) -> String {
    x.unwrap_or_else(|e| e)
}

fn main() -> rustyline::Result<()> {
    // Check if running interactively
    let is_interactive = atty::is(atty::Stream::Stdin);

    if is_interactive {
        println!("Qalculate CLI - Interactive Calculator");
        println!("Type 'exit' or 'quit' to exit\n");
        println!("Supported: sqrt(72), 2^3 + 5, sin(pi), 133 to hex, etc.\n");
    }

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
            Ok((result, num_value)) => {
                println!("\t\t{}", result);
                if let Some(num) = num_value {
                    // Always show 64-bit binary representation
                    println!(
                        "\t ━━━━━━━━━━━━━━━━━━━━━━━━━━━━\nHEX : {:?}\nDEC : {:?}\nOCT : {:?}\nBIN : {:?}\n{}\n\n",
                        print_val(convert_result(num as i64, "hex")),
                        result,
                        print_val(convert_result(num as i64, "oct")),
                        print_val(convert_result(num as i64, "bin")),
                        format_binary_64bit(num as i64)
                    );
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
