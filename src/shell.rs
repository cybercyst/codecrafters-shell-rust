use std::{
    env,
    io::{self, Write},
    path::PathBuf,
    process::{exit, Command},
};

use crate::{builtins::Builtins, utils::find_path};

pub(crate) struct Shell {
    paths: Vec<PathBuf>,
}
impl Shell {
    pub fn new() -> Self {
        let path = match env::var("PATH") {
            Ok(x) => x,
            Err(_) => {
                eprintln!("environment variable PATH is not set!");
                exit(1)
            }
        };
        let paths = path.split(':').map(PathBuf::from).collect::<Vec<PathBuf>>();
        Self { paths }
    }

    pub fn run(&self) {
        loop {
            let tokens = self.get_user_input();
            self.process_tokens(tokens)
        }
    }

    fn get_user_input(&self) -> Vec<String> {
        print!("$ ");
        io::stdout().flush().unwrap();
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        input
            .trim()
            .split(' ')
            .map(String::from)
            .collect::<Vec<String>>()
    }

    fn process_tokens(&self, tokens: Vec<String>) {
        let mut token_iter = tokens.iter();
        let token = token_iter.next().unwrap().as_str();
        match token {
            "exit" => {
                match token_iter
                    .next()
                    .unwrap_or(&String::from("0"))
                    .as_str()
                    .parse::<i32>()
                {
                    Ok(exit_code) => Builtins::exit(exit_code),
                    Err(_) => eprintln!("exit: numeric argument required"),
                }
            }
            "echo" => {
                let msg = token_iter
                    .clone()
                    .map(|x| x.as_str())
                    .collect::<Vec<&str>>()
                    .join(" ");
                Builtins::echo(msg)
            }
            "type" => {
                for token in token_iter {
                    Builtins::r#type(self.paths.clone(), token)
                }
            }
            _ => match find_path(self.paths.clone(), String::from(token)) {
                Some(cmd) => self.exec(
                    String::from(cmd.to_str().unwrap()),
                    token_iter.collect::<Vec<&String>>(),
                ),
                None => eprintln!("{}: command not found", token),
            },
        }
    }

    fn exec(&self, cmd: String, args: Vec<&String>) {
        _ = Command::new(cmd)
            .args(args)
            .status()
            .expect("process failed to start");
    }
}
