mod calculator;
mod fprice;

use calculator::engine::Engine;
use rustyline::DefaultEditor;

fn main() -> rustyline::Result<()> {
    println!("Qalculate CLI - Interactive Calculator");
    println!("Type 'help' or 'exit'\n");

    let mut rl = DefaultEditor::new()?;
    let _ = rl.load_history("history.txt");

    let mut engine = Engine::new();

    loop {
        let input = rl.readline("> ")?;
        let input = input.trim();

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
"#
            );
            continue;
        }

        match engine.eval(input) {
            Ok(out) => println!("{}", out),
            Err(e) => println!("Error: {}", e),
        }

        rl.add_history_entry(input)?;
    }

    let _ = rl.save_history("history.txt");
    Ok(())
}
