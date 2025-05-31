#[allow(unused_imports)]
use std::io::{self, Write};
use std::collections::HashSet;
use std::process::exit;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let path = env::current_dir()
    .expect("Failed to get current directory")
    .display()
    .to_string();

    let mut shell= Shell{
        builtins: vec![
            "echo".to_string(),
            "exit".to_string(),
            "type".to_string(),
            "pwd".to_string(),
            "cd".to_string(),
            ].into_iter().collect(),
        cwd: path,
    };

    shell.read_eval_print_loop();
}

struct Shell {
    builtins: HashSet<String>,
    cwd: String
}

impl Shell {
    fn read_eval_print_loop(&mut self) {
        loop {
            print!("$ ");
            io::stdout().flush().unwrap();
    
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
    
            let trimmed_input: &str = input.trim();
            self.handle_input(trimmed_input);
        }
    }
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
                    res = canoncalize(home_var, relative);
                } else if path.is_relative() {
                    res = canoncalize(self.cwd.to_string(), p.to_string());
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
}



/// returns the full path of command_name in case it exists in one of the paths under the PATH environment variable.
/// if no path found, an empty string will be returned.
fn command_in_path_env(command_name: &str) -> Result<String, String>{
    let path_var = env::var("PATH").expect("PATH not set");
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

fn canoncalize(cwd: String, relative_path: String) -> String {
    let mut res: Vec<&str> = cwd
    .split('/')
    .collect();

    let rp_dirs: Vec<&str> = relative_path
    .split('/').
    collect();

    for d in rp_dirs {
        match d {
            "" => {
                // do nothing if the path ends with / .
            }
            "." => {
                // stay in the same directory.
            },
            ".." => {
                // go back to parent.
                res.pop();
            },
            &_ => {
                // move to d.
                res.push(d);
            }
        }
    }

   res.join("/").to_string()
}