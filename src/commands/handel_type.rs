use crate::{commands::BUILT_IN,executor::check_file};
use std::io::Write;

pub fn run<W:Write>(k: &str,desination:&mut W) {
    match k {
        s if BUILT_IN.contains(&k) => writeln!(desination,"{s} is a shell builtin").unwrap(),
        _ => check_file(k, None,&mut vec![None]),
    };
}
