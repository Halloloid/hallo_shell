use crate::constantfns::trim_inside;

pub fn run(k: &str) {
    if k.len() > 4 {
        let arg = k[5..].trim();

        if arg.contains('\"') {
            for_doublequote(arg);
        } else {
            for_singlequote(arg);
        }
    }
}

fn for_doublequote(arg: &str) {
    let mut in_quote = false;
    let mut msg = String::new();
    let mut v: Vec<String> = Vec::new();

    for c in arg.chars() {
        match c {
            '\"' => {
                in_quote = !in_quote;
            }
            k if k.is_whitespace() => {
                if !in_quote {
                    v.push(msg);
                    msg = String::new();
                } else {
                    msg.push(k);
                }
            }
            _ => {
                msg.push(c);
            }
        }
    }

    v.push(msg);

    msg = String::new();
    for c in &v {
        if !c.is_empty() {
            msg.push_str(c);
            msg.push(' ');
        }
    }

    println!("{}", msg);
}

fn for_singlequote(arg: &str) {
    let mut s = String::new();
    let mut v: Vec<String> = Vec::new();
    let mut arg = format!("{}'", arg);
    for i in arg.chars() {
        if i == '\'' {
            if !s.trim().is_empty() {
                v.push(format!("{}", s.trim()));
            } else {
                v.push(s);
            }
            s = String::new();
        } else {
            s.push(i);
        }
    }

    s = String::new();
    for i in 0..v.len() - 1 {
        if &v[i] != "" {
            if v[i + 1].trim().is_empty() {
                s.push_str(&v[i]);
            } else {
                if v[i].chars().all(char::is_whitespace) {
                    s.push(' ');
                }
                arg = trim_inside(&v[i]);
                s.push_str(&arg);
            }
        }
    }

    if &v[v.len() - 1] != "" {
        arg = trim_inside(&v[v.len() - 1]);
        s.push_str(&arg);
    }

    println!("{}", s);
}
