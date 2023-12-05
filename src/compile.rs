use std::env;
use std::process::Command;
use crate::transpile::functional_transpile;

/// Compile from CLI
pub fn compile() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Not enough arguments");
        return;
    }
    let filename = args.get(2).unwrap();
    if filename.ends_with(".rs") {
        functional_compile(filename.clone())
    }
    if filename.ends_with(".bs") {
        let rust_filename = functional_transpile(filename);
        functional_compile(rust_filename);
    }
}

/// Compile from code
fn functional_compile(filename: String){
    Command::new("rustc")
        .arg(filename)
        .spawn()
        .expect("Cannot compile");
}