
use std::collections::HashSet;
use std::process::exit;
use std::path::Path;
use std::process::Command;
use std::io;
use std::io::Write;
use std::env;
use crate::utils;

pub struct Shell {
    builtins: HashSet<String>,
    cwd: String
}

impl Shell {
    /// creates a new shell.
    pub fn new() -> Self {
        let path = env::current_dir()
        .expect("Failed to get current directory")
        .display()
        .to_string();

        Self {
            builtins: vec![
                "echo".to_string(),
                "exit".to_string(),
                "type".to_string(),
                "pwd".to_string(),
                "cd".to_string(),
                ].into_iter().collect(),
            cwd: path,
        }
    }
    /// starts the read-eval-print loop.
    pub fn read_eval_print_loop(&mut self) {
        loop {
            print!("$ ");
            io::stdout().flush().unwrap();
    
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
    
            let trimmed_input: &str = input.trim();
            self.handle_input(trimmed_input);
        }
    }
    /// handles the given input.
    fn handle_input(&mut self, input: &str) {
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
                if self.builtins.contains(command) {
                    println!("{} is a shell builtin", command);
                    return;
                }
    
                // second check if this command exists under PATH.
                match utils::command_in_path_env(command) {
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
            "pwd" => println!("{}", self.cwd),
            "cd" => {
                let p = words[1];
                if p.is_empty() {
                    return;
                }

                let mut path = Path::new(p);
                let res: String;

                if p.chars().nth(0).unwrap() == '~' {
                    let home_var = env::var("HOME").expect("HOME var not set");
                    let substr: String = p.chars().skip(1).collect();
                    let relative = format!("{}{}", ".", substr);
                    res = utils::canoncalize(home_var, relative);
                } else if path.is_relative() {
                    res = utils::canoncalize(self.cwd.to_string(), p.to_string());
                } else {
                    res = p.to_string();
                }

                path = Path::new(&res);
                if !path.exists() {
                    eprintln!("cd: {}: No such file or directory", p);
                    return;
                }

                self.cwd = res;
            }
            &_ => {
                match utils::command_in_path_env(command) {
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
}