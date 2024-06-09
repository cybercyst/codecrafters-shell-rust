#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, fs, process::exit};

const BUILTIN_FNS: [&str; 3] = ["exit", "echo", "type"];

fn lookup_executable(executable: &str) -> String {
    let path_env = env::var("PATH").unwrap();
    let paths = path_env.split(':');

    for path in paths {
        let mut contents = fs::read_dir(path).unwrap();
        if contents.any(|file| file.unwrap().file_name() == executable) {
            return format!("{} is {}/{}", executable, path, executable);
        }
    }

    format!("{}: command not found", executable)
}

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
                let exit_code = tokens.next().unwrap_or("0").parse::<i32>().unwrap_or(0);
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
                        _ => lookup_executable(token),
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
