#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed_input: &str = input.trim();

        let words: Vec<&str> = trimmed_input.split(' ').collect();

        let command: &str = words[0];
        match command {
            "echo" =>  println!("{}", words[1..].join(" ")),
            "exit" => {
                let code: i32 = words[1].
                parse().
                expect("not a valid exit code");

                exit(code);
            },
            &_ => println!("{}: command not found", trimmed_input)
        }
    }
}
