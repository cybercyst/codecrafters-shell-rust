#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

const BUILTIN_FNS: [&str; 3] = ["exit", "echo", "type"];

fn main() -> ! {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let mut tokens = input.trim().split(' ');
        let token = tokens.next().unwrap();
        match token {
            "exit" => {
                let exit_code = tokens.next().unwrap_or("0").parse::<i32>().unwrap();
                exit(exit_code)
            }
            "echo" => {
                let msg = tokens.clone().collect::<Vec<&str>>().join(" ");
                println!("{}", msg);
            }
            "type" => {
                for token in tokens {
                    let msg = match token {
                        x if BUILTIN_FNS.iter().any(|&i| i == x) => {
                            format!("{} is a shell builtin", x)
                        }
                        _ => format!("{} not found", token),
                    };
                    println!("{}", msg);
                }
            }
            _ => {
                eprintln!("{}: command not found", token);
            }
        }
    }
}
