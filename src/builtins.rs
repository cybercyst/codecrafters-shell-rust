use std::{path::PathBuf, process::exit};

use crate::utils::find_path;

pub(crate) struct Builtins;
impl Builtins {
    pub fn echo(msg: String) {
        println!("{}", msg)
    }

    pub fn r#type(paths: Vec<PathBuf>, cmd: &String) {
        let msg = match cmd {
            x if ["echo", "type", "exit"].iter().any(|&i| i == x) => {
                format!("{} is a shell builtin", x)
            }
            _ => match find_path(paths, cmd.clone()) {
                Some(path) => format!("{} is {}", cmd, path.to_str().unwrap()),
                None => format!("{}: not found", cmd),
            },
        };
        println!("{}", msg)
    }

    pub fn exit(exit_code: i32) {
        exit(exit_code);
    }
}
