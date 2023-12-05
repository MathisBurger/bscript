mod std_lib;
mod file_handler;
pub(crate) mod hasher;
mod builder;

use std::env;
use std::fmt::format;
use crate::transpile::builder::Builder;

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

pub fn functional_transpile(filename: &String) {
    let content = file_handler::get_script_content(filename.clone());
    let content_hash = hasher::calculate_hash(&content);
    let mut builder = Builder::new(content);
    file_handler::write_string(format!("{}{}.rs", content_hash, filename), builder.build()).expect("Cannot transpile");
}