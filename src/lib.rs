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

    use is_executable::IsExecutable;
    use std::{env, process::Command};

    pub fn check_file(k: &str, args: Option<&[String]>) {
        let mut check_for_re = false;
        let mut i = "Nothing";
        let re = String::from(">");
        let re2 = String::from("1>");
        let mut f = false;
        if let Ok(path_spliter) = env::var("PATH") {
            for mut path in env::split_paths(&path_spliter) {
                path.push(k);

                if path.is_file() && path.is_executable() {
                    if args.iter().len() != 0 {
                        let mut child = Command::new(k);
                        if let Some(ar) = args {
                            check_for_re = ar.contains(&re) || ar.contains(&re2);
                            if check_for_re {
                                i = &ar[ar.len() - 1];
                                child.args(&ar[..ar.len() - 2]);
                            } else {
                                child.args(ar);
                            }
                        }

                        match child.output() {
                            Ok(output) => {
                                let stdout_str =
                                    String::from_utf8_lossy(&output.stdout).into_owned();
                                let stder_str =
                                    String::from_utf8_lossy(&output.stderr).into_owned();
                                if check_for_re {
                                    store_in_file(stdout_str, i.trim());
                                } else if !stdout_str.is_empty() {
                                    if stdout_str.ends_with('\n') {
                                        print!("{}", stdout_str);
                                    } else {
                                        println!("{}", stdout_str);
                                    }
                                }

                                if !stder_str.is_empty() {
                                    if stder_str.ends_with('\n') {
                                        eprint!("{}", stder_str);
                                    } else {
                                        eprintln!("{}", stder_str);
                                    }
                                }
                            }
                            Err(e) => {
                                println!("Failed to process {}", e)
                            }
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
                    args.iter().len() + 1
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

    use std::{fs, path::Path};

    pub fn store_in_file(output: String, file_path: &str) {
        let clen_path = file_path.trim();

        if let Some(parent) = Path::new(clen_path).parent() {
            if !parent.as_os_str().is_empty() {
                let _ = fs::create_dir_all(parent);
            }
        }

        fs::write(file_path, output).expect("Unabel to write");
    }
}
