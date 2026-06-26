use cel::{Context, Program};

fn main() {
    let expr = "10000 * 0.15";
    let prog = Program::compile(expr).unwrap();
    let ctx = Context::default();
    match prog.execute(&ctx) {
        Ok(v) => println!("Success: {:?}", v),
        Err(e) => println!("Error: {:?}", e),
    }
}
