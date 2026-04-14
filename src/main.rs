use std::{io, path::Path};

use tcal_rs::{
    calculator::engine::Engine,
    save_history::{load_history, readline_with_history, save_history},
};

fn main() -> io::Result<()> {
    println!("Qalculate CLI - Interactive Calculator");
    println!("Type 'help' or 'exit' (or press Ctrl+C/Ctrl+D to quit)\n");

    let history_path = Path::new("history.txt");
    let mut history = load_history(history_path)?;
    let mut engine = Engine::new();

    loop {
        let input = match readline_with_history("> ", &history)? {
            Some(input) => input,
            None => break, // User pressed Ctrl+C or Ctrl+D
        };

        if input.is_empty() {
            continue;
        }

        if input == "exit" || input == "quit" {
            break;
        }

        if input == "help" {
            println!(
                r#"Commands:
- math: 2+3*4
- power: 2^10
- bitwise: 5 & 3, 1 << 4
- hex/bin/oct: 0xFF, 0b1010
- convert: 255 to hex bin oct
- unicode: "안녕" to unicode
- variables: x = 10
- res: reuse last result
- Arrow keys: navigate history
"#
            );
            continue;
        }

        match engine.full_eval(&input) {
            Ok(out) => println!("{}", out),
            Err(e) => println!("Error: {}", e),
        }

        history.push(input);
    }

    save_history(history_path, &history)?;
    Ok(())
}
