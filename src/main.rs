#[allow(unused_imports)]
use std::io::{self, Write};

mod commands;

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
            k if k.starts_with("echo") => commands::handel_echo::run(&k[5..]),
            k if k.starts_with("type") => commands::handel_type::run(&k[5..]),
            _ => {
                let args: Vec<&str> = command.split(" ").collect();
                if args.len() == 1 {
                    println!("{}: command not found", command);
                } else {
                    commands::handel_type::check_file(&args[0], Some(&args[1..]));
                }
            }
        };
    }
}
