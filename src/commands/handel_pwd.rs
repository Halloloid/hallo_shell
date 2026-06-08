use std::env;

pub fn run(){
    match env::current_dir() {
        Ok(path) => println!("{}",path.display()),
        Err(e) => eprintln!("Cannot Retive Current Directory {}.",e),
    };
}