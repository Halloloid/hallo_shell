pub mod constantfns {
    pub fn split_by_args(command: &str) -> Vec<String> {
        let mut args: Vec<String> = Vec::new();
        let mut curr_arg = String::new();

        let mut in_quote = false;
        for i in command.chars() {
            match i {
                '\'' => in_quote = !in_quote,
                k if k.is_whitespace() => {
                    if !in_quote {
                        args.push(curr_arg);
                        curr_arg = String::new();
                    } else {
                        curr_arg.push(i);
                    }
                }
                _ => {
                    curr_arg.push(i);
                }
            }
        }

        args.push(curr_arg);
        args
    }

    use is_executable::IsExecutable;
    use std::{env, process::Command};

    pub fn check_file(k: &str, args: Option<&[String]>) {
        let mut f = false;
        if let Ok(path_spliter) = env::var("PATH") {
            for mut path in env::split_paths(&path_spliter) {
                path.push(k);
    
                if path.is_file() && path.is_executable() {
                    if args.iter().len() != 0 {
                        let mut child = Command::new(k);
                        if let Some(ar) = args {
                            child.args(ar);
                        }
    
                        match child.status() {
                            Ok(_status) => {}
                            Err(e) => println!("Failed to execute process: {}", e),
                        }
    
                        f = true;
                        break;
                    }
                    println!("{} is {}", k, path.display());
                    f = true;
                    break;
                }
            }
        }
        if f == false {
            if args.iter().len() != 0 {
                println!(
                    "Program was passed {} args (including program name).",
                    args.iter().len()+1
                );
            } else {
                println!("{}: not found", k);
            }
        }
    }
}
