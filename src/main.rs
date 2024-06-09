mod builtins;
mod shell;
mod utils;

use crate::shell::Shell;

fn main() {
    Shell::new().run();
}
