use is_executable::IsExecutable;
use std::{env, process::Command};

use crate::redirect;

pub fn check_file(k: &str, args: Option<&[String]>) {
    let mut check_for_re = false;
    let mut check_for_re_apend = false;
    let mut check_for_er_re = false;
    let mut check_for_er_re_apend = false;
    let mut no_need = true;
    let mut back = false;
    let mut i = "Nothing";
    let re1 = String::from(">");
    let re2 = String::from("1>");
    let re3 = String::from("2>");
    let re1_append = String::from(">>");
    let re2_append = String::from("1>>");
    let re3_append = String::from("2>>");
    let background = String::from("&");
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
                        check_for_re_apend = ar.contains(&re1_append) || ar.contains(&re2_append);
                        check_for_er_re_apend = ar.contains(&re3_append);

                        if check_for_re
                            || check_for_er_re
                            || check_for_re_apend
                            || check_for_er_re_apend
                        {
                            i = &ar[ar.len() - 1];
                            child.args(&ar[..ar.len() - 2]);
                        } else {
                            if ar.contains(&background) {
                                child.args(&ar[..ar.len() - 1]);
                                match child.spawn() {
                                    Ok(child) => {
                                        let job_no = 1;
                                        let pid = child.id();

                                        println!("[{}] {}", job_no, pid);
                                        back = true;
                                    }
                                    Err(e) => println!("{}: command not found", e),
                                }
                            } else {
                                child.args(ar);
                            }
                        }
                    }

                    if !back {
                        match child.output() {
                            Ok(output) => {
                                let stdout_str =
                                    String::from_utf8_lossy(&output.stdout).into_owned();
                                let stder_str =
                                    String::from_utf8_lossy(&output.stderr).into_owned();
                                if check_for_re
                                    || check_for_er_re
                                    || check_for_re_apend
                                    || check_for_er_re_apend
                                {
                                    redirect::std_store(
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
