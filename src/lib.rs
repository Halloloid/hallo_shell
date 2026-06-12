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
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
    use std::{env, process::Command};

    pub fn check_file(k: &str, args: Option<&[String]>) {
        let mut check_for_re = false;
        let mut check_for_re_apend = false;
        let mut check_for_er_re = false;
        let mut check_for_er_re_apend = false;
        let mut no_need = true;
        let mut i = "Nothing";
        let re1 = String::from(">");
        let re2 = String::from("1>");
        let re3 = String::from("2>");
        let re1_append = String::from(">>");
        let re2_append = String::from("1>>");
        let re3_append = String::from("2>>");
        let mut f = false;
        if let Ok(path_spliter) = env::var("PATH") {
            for mut path in env::split_paths(&path_spliter) {
                path.push(k);

                if path.is_file() && path.is_executable() {
                    if args.iter().len() != 0 {
                        let mut child = Command::new(k);
                        if let Some(ar) = args {
                            check_for_er_re = ar.contains(&re3);
                            check_for_re = ar.contains(&re1) || ar.contains(&re2);
                            check_for_re_apend =
                                ar.contains(&re1_append) || ar.contains(&re2_append);
                            check_for_er_re_apend = ar.contains(&re3_append);

                            if check_for_re || check_for_er_re || check_for_re_apend || check_for_er_re_apend{
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
                                if check_for_re || check_for_er_re || check_for_re_apend || check_for_er_re_apend{
                                    std_store(
                                        check_for_er_re,
                                        check_for_re_apend,
                                        check_for_er_re_apend,
                                        stdout_str,
                                        stder_str.clone(),
                                        i,
                                    );
                                    no_need = !no_need;
                                } else if !stdout_str.is_empty() {
                                    if stdout_str.ends_with('\n') {
                                        print!("{}", stdout_str);
                                    } else {
                                        println!("{}", stdout_str);
                                    }
                                }

                                if !stder_str.is_empty() && !check_for_er_re && no_need {
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

    pub fn store_in_file(output: String, file_path: &str, append: bool) {
        let clen_path = file_path.trim();

        if let Some(parent) = Path::new(clen_path).parent() {
            if !parent.as_os_str().is_empty() {
                let _ = fs::create_dir_all(parent);
            }
        }

        if append {
            let mut content = fs::read_to_string(file_path).unwrap_or_default();
            if content.ends_with('\n') || content.is_empty() {
                content.push_str(&output);
            } else {
                content.push('\n');
                content.push_str(&output);
            }

            fs::write(file_path, content).expect("unabel to write");
        } else {
            fs::write(file_path, output).expect("Unabel to write");
        }
    }

    pub fn std_store(
        store_err: bool,
        append: bool,
        append_err :bool,
        stdout_str: String,
        stderr_str: String,
        file: &str,
    ) {
        let store = if store_err || append_err{
            stderr_str.clone()
        } else {
            stdout_str.clone()
        };

        store_in_file(store, file.trim(), append||append_err);

        if store_err || append_err{
            if !stdout_str.is_empty() {
                if stdout_str.ends_with('\n') {
                    eprint!("{}", stdout_str);
                } else {
                    eprintln!("{}", stdout_str);
                }
            }
        } else {
            if !stderr_str.is_empty() {
                if stderr_str.ends_with('\n') {
                    eprint!("{}", stderr_str);
                } else {
                    eprintln!("{}", stderr_str);
                }
            }
        }
    }

    use rustyline::{self, Helper};
    
    pub struct ShellHelper{
        pub completer:ShellCompleter,
    }

    impl Helper for ShellHelper {}

    impl rustyline::completion::Completer for ShellHelper {

        type Candidate = String;
        
        fn complete(
            &self, // FIXME should be `&mut self`
            line: &str,
            pos: usize,
            ctx: &rustyline::Context<'_>,
        ) -> rustyline::Result<(usize, Vec<Self::Candidate>)>
        {
            self.completer.complete(line, pos, ctx)
        }
    }

    impl Hinter for ShellHelper {
        type Hint = String;
        fn hint(&self, _line: &str, _pos: usize, _ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
            None
        }
    }

    impl Highlighter for ShellHelper {}

    impl Validator for ShellHelper {
        fn validate(&self, _ctx: &mut rustyline::validate::ValidationContext) -> rustyline::Result<rustyline::validate::ValidationResult> {
            Ok(rustyline::validate::ValidationResult::Valid(None))
        }
    }

    pub struct ShellCompleter;

    impl rustyline::completion::Completer for ShellCompleter {
        type Candidate = String;

        fn complete(
            &self, // FIXME should be `&mut self`
            line: &str,
            pos: usize,
            _ctx: &rustyline::Context<'_>,
        ) -> rustyline::Result<(usize, Vec<Self::Candidate>)>
        {
            let mut candidates = Vec::new();

            let current_word = &line[..pos];
            if "echo".starts_with(current_word) && !current_word.is_empty()
            {
                candidates.push("echo ".to_string());
            }else if "exit".starts_with(current_word) && !current_word.is_empty() {
                candidates.push("exit ".to_string());
            }

            Ok((0,candidates))
        }
    }
}
