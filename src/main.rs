#[allow(unused_imports)]
use std::io::{self, Write};
use std::collections::HashSet;
use std::process::exit;

fn main() {
    let builtins : HashSet<&str> = vec![
        "echo",
         "exit",
         "type",
         ].into_iter().collect();

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
            "type" => {
                if builtins.contains(words[1]) {
                    println!("{} is a shell builtin", words[1]);
                } else {
                    println!("{}: command not found", trimmed_input);
                }
            }
            &_ => println!("{}: command not found", trimmed_input)
        }
    }
}
