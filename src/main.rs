#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() -> ! {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let mut tokens = input.trim().split(' ');
        while let Some(token) = tokens.next() {
            match token {
                "exit" => {
                    let exit_code = tokens.next().unwrap().parse::<i32>().unwrap();
                    exit(exit_code)
                }
                _ => {
                    eprintln!("{}: command not found", token);
                    break;
                }
            }
        }
    }
}
