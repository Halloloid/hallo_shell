pub fn split_by_args(command: &str) -> Vec<String> {
    let mut chars = command.chars().peekable();
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
                    _ => s.push(nxt_chr),
                }
                chars.next();
            }
        } else if cur == ' ' {
            v.push(s);
            s = String::new();
        } else {
            if cur == '\'' {
                let r = split_by_args_quotes(command, '\'');
                return r;
            }
            if cur == '\"' {
                let r = split_by_args_quotes(command, '\"');
                return r;
            }
            s.push(cur);
        }
    }
    v.push(s);
    v
}

fn split_by_args_quotes(command: &str, quote: char) -> Vec<String> {
    let mut chars = command.chars().peekable();

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
                } else {
                    if let Some(&nxt_chr) = chars.peek() {
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
    v
}
