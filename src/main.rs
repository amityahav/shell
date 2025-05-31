mod shell;
mod utils;

use shell::Shell;

fn main() {
    let mut shell= Shell::new();

    shell.read_eval_print_loop();
}