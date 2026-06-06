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
            k if k.starts_with("echo") =>{
                if let Some(s) = k.get(5..){
                    println!("{s}");
                }
            },
            _ => println!("{}: command not found",command)
        };
    }
}
