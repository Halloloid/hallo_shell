use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};

use is_executable::IsExecutable;

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Unabel to Read");
        let command = command.trim_end();
        match command {
            "exit" => break,
            k if k.starts_with("echo") => println!("{}", &k[5..]),
            k if k.starts_with("type") => {
                let k = &k[5..];
                match k {
                    s if k == "exit" || k == "echo" || k == "type" => {
                        println!("{s} is a shell builtin")
                    }
                    _ => {
                        let mut f = false;
                        if let Ok(path_spliter) = env::var("PATH") {
                            for mut path in env::split_paths(&path_spliter) {
                                path.push(k);

                                if path.is_file() && path.is_executable() {
                                    println!("{} is {}", k, path.display());
                                    f = true;
                                    break;
                                }
                            }
                        }
                        if f == false {
                            println!("{}: not found", k);
                        }
                    }
                };
            }
            _ => println!("{}: command not found", command),
        };
    }
}
