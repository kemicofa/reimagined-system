use std::process::{Output, self};
use colored::*;

/// Not great that we clone the
/// the bytes here but not really another choice
fn bytes_to_string(bytes: &Vec<u8>) -> String {
    String::from_utf8(bytes.clone())
        .expect("Expected to be able to convert bytes to string.")
        .trim()
        .to_string()
}

pub fn get_stdout_or_terminate(output: &Output) -> String {
    if !output.status.success() {
        let error_message = bytes_to_string(&output.stderr);
        println!("{}", error_message.red());
        process::exit(1);
    } 
    
    bytes_to_string(&output.stdout)
}