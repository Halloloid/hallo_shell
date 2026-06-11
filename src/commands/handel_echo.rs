use codecrafters_shell::constantfns::{self, check_file, split_by_args};

pub fn run(k: &str) {
    if k.len() > 4 {
        let arg = k[5..].trim();
        if arg.contains(">") && !arg.contains("2>") {
            redirect_output(arg);
        } else if arg.contains("2>") {
            let chr = if arg.contains("2>>") { "2>>" } else { "2>" };
            let split_by_red: Vec<&str> = arg.split(chr).collect();
            let mut file: Vec<String> = Vec::new();
            file.push(format!("{}", split_by_red[1].trim()));
            check_file("touch", Some(&file));
            constantfns::store_in_file(String::from(""), &file[0], false);
            for_backslash(split_by_red[0]);
        } else {
            for_backslash(arg);
        }
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

fn redirect_output(arg: &str) {
    let mut append = false;
    let chr = if arg.contains("1>>") {
        append = !append;
        "1>>"
    } else if arg.contains(">>") {
        append = !append;
        ">>"
    } else if arg.contains("1>") {
        "1>"
    } else {
        ">"
    };

    let split_by_red: Vec<&str> = arg.split(chr).collect();

    let arg = split_by_red[0];
    let mut file: Vec<String> = Vec::new();
    file.push(format!("{}", split_by_red[1].trim()));

    let v = split_by_args(arg);
    let mut msg = String::new();
    for c in &v {
        if !c.is_empty() {
            msg.push_str(c);
            msg.push(' ');
        }
    }

    check_file("touch", Some(&file));

    constantfns::store_in_file(msg, &file[0], append);
}
