use std::{env,io::Write};

pub fn run<W:Write>(desination:&mut W){
    match env::current_dir() {
        Ok(path) => writeln!(desination,"{}",path.display()).unwrap(),
        Err(e) => eprintln!("Cannot Retive Current Directory {}.",e),
    };
}