use crate::commands::BUILT_IN;
use crate::constantfns::check_file;

pub fn run(k: &str) {
    match k {
        s if BUILT_IN.contains(&k) => println!("{s} is a shell builtin"),
        _ => check_file(k, None),
    };
}
