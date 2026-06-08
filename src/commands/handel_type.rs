use is_executable::IsExecutable;
use std::{env, process::Command};

use crate::commands::{BUILT_IN};

pub fn run(k: &str) {
    match k {
        s if BUILT_IN.contains(&k) => println!("{s} is a shell builtin"),
        _ => check_file(k, None),
    };
}

pub fn check_file(k: &str, args: Option<&[&str]>) {
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
