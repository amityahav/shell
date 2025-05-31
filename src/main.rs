#[allow(unused_imports)]
use std::io::{self, Write};
use std::collections::HashSet;
use std::process::exit;
use std::env;
use std::fs;
use std::path::Path;

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
                let command = words[1];
                if builtins.contains(command) {
                    println!("{} is a shell builtin", command);
                    continue;
                }

                let path_var = env::var("PATH").expect("path not set");
                let paths: Vec<&str> = path_var.split(':').collect();

                for p in paths {
                    let path = Path::new(p);
                    match fs::read_dir(path) {
                        Ok(entries) => {
                            for entry in entries {
                                match entry {
                                    Ok(ent) => {
                                        if ent.file_name() == command {
                                            println!("{} is {}/{}", command, p, command);
                                            continue;
                                        }
                                    },
                                    Err(e) => eprintln!("Failed to read entry: {}", e)
                                }
                            }
                        },
                        Err(e) => eprintln!("Failed to read directory: {}", e)
                    }
                }

                println!("{}: not found", command);
            }
            &_ => println!("{}: command not found", trimmed_input)
        }
    }
}
