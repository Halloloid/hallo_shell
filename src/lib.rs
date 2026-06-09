pub mod constantfns {
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
                if cur == '\'' || cur == '\"'{
                    let r =split_by_args_quotes(command);
                    return r;
                }
                s.push(cur);
            }
        }
        v.push(s);
        v     
    }

    fn split_by_args_quotes(command: &str) -> Vec<String>{
        let quote = if command.contains('\"') { '\"' } else { '\'' };
        let mut args: Vec<String> = Vec::new();
        let mut curr_arg = String::new();

        let mut in_quote = false;
        for i in command.chars() {
            match i {
                i if i == quote => in_quote = !in_quote,
                k if k.is_whitespace() => {
                    if !in_quote {
                        args.push(curr_arg);
                        curr_arg = String::new();
                    } else {
                        curr_arg.push(i);
                    }
                }
                _ => {
                    curr_arg.push(i);
                }
            }
        }
        args.push(curr_arg);
        args
    }
    

    use is_executable::IsExecutable;
    use std::{env, process::Command};

    pub fn check_file(k: &str, args: Option<&[String]>) {
        let mut f = false;
        if let Ok(path_spliter) = env::var("PATH") {
            for mut path in env::split_paths(&path_spliter) {
                path.push(k);
    
                if path.is_file() && path.is_executable() {
                    if args.iter().len() != 0 {
                        let mut child = Command::new(k);
                        if let Some(ar) = args {
                            child.args(ar);
                        }
    
                        match child.status() {
                            Ok(_status) => {}
                            Err(e) => println!("Failed to execute process: {}", e),
                        }
    
                        f = true;
                        break;
                    }
                    println!("{} is {}", k, path.display());
                    f = true;
                    break;
                }
            }
        }
        if f == false {
            if args.iter().len() != 0 {
                println!(
                    "Program was passed {} args (including program name).",
                    args.iter().len()+1
                );
            } else {
                println!("{}: not found", k);
            }
        }
    }

    pub fn trim_inside(k: &str) -> String {
        let k = k.trim();
        let spit: Vec<&str> = k.split(" ").collect();
        let mut s = String::new();
        for i in spit {
            if i != "" {
                s.push_str(i);
                s.push(' ');
            }
        }
        s
    }
    
}
