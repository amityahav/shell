use std::fs;
use std::env;
use std::path::Path;


/// returns the full path of command_name in case it exists in one of the paths under the PATH environment variable.
/// if no path found, an empty string will be returned.
pub fn command_in_path_env(command_name: &str) -> Result<String, String>{
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

pub fn canoncalize(cwd: String, relative_path: String) -> String {
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

pub fn collapse_whitespace(s: &str) -> String {
    let mut result = String::new();
    let mut prev_was_whitespace = false;

    for c in s.chars() {
        if c.is_whitespace() {
            if !prev_was_whitespace {
                result.push(' ');
                prev_was_whitespace = true;
            }
        } else {
            result.push(c);
            prev_was_whitespace = false;
        }
    }

    result.trim().to_string()
}