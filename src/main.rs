mod transpile;
mod compile;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Please provide some arguments");
        return;
    }
    match args[1].as_str() {
        "transpile" => transpile::transpile(),
        "compile" => compile::compile(),
        _ => println!("Command not found")
    }
}
