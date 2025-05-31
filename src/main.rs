use std::f32::consts::E;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::collections::HashSet;
use std::path;
use std::process::exit;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let trimmed_input: &str = input.trim();
        handle_input(trimmed_input);
    }
}

fn handle_input(input: &str) {
    let builtins : HashSet<&str> = vec![
        "echo",
         "exit",
         "type",
         ].into_iter().collect();

    let words: Vec<&str> = input.split(' ').collect();

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

            // first check if this is a sell builtin command.
            if builtins.contains(command) {
                println!("{} is a shell builtin", command);
                return;
            }

            // second check if this command exists under PATH.
            match command_in_path_env(command) {
                Ok(path_name) => {
                    if path_name.is_empty() {
                        println!("{}: not found", command);
                        return;
                    }

                    println!("{} is {}", command, path_name)
                },
                Err(e) => eprintln!("{}", e)
            }
        },
        "pwd" => {
            match env::current_dir() {
                Ok(path) => println!("{}", path.display()),
                Err(e) => eprintln!("Failed to get current directory: {}", e)
            }
        },
        &_ => {
            match command_in_path_env(command) {
                Ok(path_name) => {
                    if path_name.is_empty() {
                        println!("{}: command not found", input);
                        return;
                    }

                    // run the executable and print its stdout, stderr.
                    let output = Command::new(command)
                    .arg(words[1..].join(" "))
                    .output();

                    match output {
                        Ok(out) => {
                            print!("{}", String::from_utf8_lossy(&out.stdout));
                            eprint!("{}", String::from_utf8_lossy(&out.stderr));
                        },
                        Err(e) => eprintln!("Failed getting executable output: {}", e)
                    }
                },
                Err(e) => eprintln!("{}", e)
            }
        }
    }
}

/// returns the full path of command_name in case it exists in one of the paths under the PATH environment variable.
/// if no path found, an empty string will be returned.
fn command_in_path_env(command_name: &str) -> Result<String, String>{
    let path_var = env::var("PATH").expect("path not set");
    let paths: Vec<&str> = path_var.split(':').collect();

    for p in paths {
        let path = Path::new(p);
        match fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(ent) => {
                            if ent.file_name() == command_name {
                                return Ok( format!("{}/{}", p, command_name));
                            }
                        },
                        Err(e) => return Err(format!("Failed to read entry: {}", e))
                    }
                }
            },
            Err(e) => return Err(format!("Failed to read directory: {}", e))
        }
    }
                    
   Ok("".to_string())
}