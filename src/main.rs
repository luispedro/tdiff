#![feature(stdin_forwarders)]
use std::io;

fn main() {
    let input : Result<Vec<String>, _> =
        io::stdin()
        .lines()
        .collect();

    match input {
        Ok(lines) =>
            println!("Number of lines: {}", lines.len()),
        Err(err) =>
            println!("Error!: {}", err),
    }
}
