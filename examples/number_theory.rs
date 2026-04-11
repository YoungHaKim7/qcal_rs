use tcal_rs::calculator::engine::Engine;

fn main() {
    let mut engine = Engine::new();

    // Basic arithmetic
    // assert_eq!(engine.eval("2 + 2").unwrap(), "4");
    println!("eval : {:?}", engine.eval("4"));

    // Trigonometry
    assert_eq!(engine.eval("sin(pi/2)").unwrap(), "1");

    // Number theory
    assert_eq!(engine.eval("totient(30)").unwrap(), "8");
}
