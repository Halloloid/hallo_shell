use codecrafters_shell::constantfns::split_by_args;

pub fn run(k: &str) {
    if k.len() > 4 {
        let arg = k[5..].trim();
        for_backslash(arg);
    }
}

fn for_backslash(arg: &str) {
    let v = split_by_args(arg);
    let mut msg = String::new();
    for c in &v {
        if !c.is_empty() {
            msg.push_str(c);
            msg.push(' ');
        }
    }

    println!("{}", msg);
}
