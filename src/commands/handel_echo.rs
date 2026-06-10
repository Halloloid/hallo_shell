pub fn run(k: &str) {
    if k.len() > 4 {
        let arg = k[5..].trim();
        for_backslash(arg);
    }
}

fn for_quote(arg: &str) {
    let quote = if arg.starts_with('\"') { '\"' } else { '\'' };

    let mut chars = arg.chars().peekable();

    let mut in_quote = false;
    let mut msg = String::new();
    let mut v: Vec<String> = Vec::new();

    while let Some(c) = chars.next() {
        match c {
            _ if c == quote => in_quote = !in_quote,
            k if k.is_whitespace() => {
                if !in_quote {
                    v.push(msg);
                    msg = String::new();
                } else {
                    msg.push(k);
                }
            }
            '\\' => {
                if in_quote && quote == '\'' {
                    msg.push('\\');
                }else {
                    if let Some(&nxt_chr) = chars.peek(){
                        match nxt_chr {
                            '\"' => msg.push('\"'),
                            '\\' => msg.push('\\'),
                            _ => {}
                        }
                        chars.next();
                    }
                }
            }
            '\"' => {
                if in_quote && quote == '\'' {
                    msg.push('\"');
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

fn for_backslash(arg: &str) {
    let mut chars = arg.chars().peekable();
    let mut s = String::new();
    let mut v: Vec<String> = Vec::new();

    while let Some(cur) = chars.next() {
        if cur == '\\' {
            if let Some(&nxt_chr) = chars.peek() {
                match nxt_chr {
                    ' ' => s.push(' '),
                    'n' => s.push('n'),
                    '\\' => s.push('\\'),
                    '\'' => s.push('\''),
                    '\"' => s.push('\"'),
                    _ => {}
                }
                chars.next();
            }
        } else if cur == ' ' {
            v.push(s);
            s = String::new();
        } else {
            if cur == '\"' || cur == '\'' {
                for_quote(arg);
                return;
            }
            s.push(cur);
        }
    }
    v.push(s);
    s = String::new();

    for i in &v {
        if !i.is_empty() {
            s.push_str(i);
            s.push(' ');
        }
    }
    println!("{}", s);
}
