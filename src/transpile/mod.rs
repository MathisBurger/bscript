mod std_lib;
mod file_handler;
pub(crate) mod hasher;
mod builder;

use std::env;
use crate::transpile::builder::Builder;

/// This method is called to transpile from command
pub fn transpile() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Not enough arguments");
        return;
    }
    let filename = args.get(2).unwrap();
    if !filename.ends_with(".bs") {
        println!("Wrong file");
        return;
    }
    functional_transpile(filename);
}

/// This method transpiles the content of a specific file
pub fn functional_transpile(filename: &String) -> String {
    let content = file_handler::get_script_content(filename.clone());
    let content_hash = hasher::calculate_hash(&content);
    let mut builder = Builder::new(content);
    let output_file_name = format!("{}{}", content_hash, filename).replace(".", "_");
    let rust_file_name = format!("{}.rs", output_file_name);
    file_handler::write_string(rust_file_name.clone(), builder.build()).expect("Cannot transpile");
    return rust_file_name;
}