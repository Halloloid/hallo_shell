#[allow(unused_imports)]
use std::io::{self, Write};

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
            k if k.starts_with("echo") => println!("{}",&k[5..]),
            k if k.starts_with("type") => {
                match &k[5..] {
                    "exit" => println!("exit is a shell builtin"),
                    "echo" => println!("echo is a shell builtin"),
                    "type" => println!("type is a shell builtin"),
                    _ => println!("{}: not found",&k[5..])
                };  
            },
            _ => println!("{}: command not found",command)
        };
    }
}
