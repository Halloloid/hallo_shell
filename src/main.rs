#[allow(unused_imports)]
use std::io::{self, Write};


use codecrafters_shell::constantfns;
mod commands;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Unabel to Read");
        let command = command.trim();
        match command {
            "exit" => break,
            "pwd" => commands::handel_pwd::run(),
            k if k.starts_with("echo") => commands::handel_echo::run(&k),
            k if k.starts_with("type") => commands::handel_type::run(&k[5..]),
            k if k.starts_with("cd") => commands::handel_cd::run(&k[3..]),
            _ => {
                let arg = constantfns::split_by_args(command);
                if arg.len() == 1 {
                    println!("{}: command not found", command);
                } else {
                    constantfns::check_file(&arg[0], Some(&arg[1..]));
                }
            }
        };
    }
}
