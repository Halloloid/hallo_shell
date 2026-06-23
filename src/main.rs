#[allow(unused_imports)]
use std::io::{self, Write,};
use std::{collections::HashMap,process::Child};

use hallo_shell::{executor, parser};
use rustyline::error::ReadlineError;
use rustyline::{Editor};

use hallo_shell::{shell_helper::ShellHelper,completion::ShellCompleter};
mod commands;

fn main() {
    let config = rustyline::Config::builder()
        .completion_type(rustyline::CompletionType::List)
        .build();

    let mut rl = Editor::with_config(config).unwrap();

    let store:HashMap<String,String> = HashMap::new();
    
    rl.set_helper(Some(ShellHelper{
        completer:ShellCompleter{completeion:store},
    }));

    let mut back_jobs = Vec::<Option<(u8,Child,String)>>::new();

    
    loop {

        let readline = rl.readline("$ ");

        let command = match readline {
            Ok(line) => line,
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(err)=>{
                println!("Error: {:?}",err);
                break;
            }
        };

        let command = command.trim();
        if command.is_empty(){
            continue;
        }
        
        match command {
            "exit" => break,
            "pwd" => commands::handel_pwd::run(),
            "hallo" => commands::handel_hallo::splash_screen(),
            k if k.starts_with("jobs") =>commands::handel_jobs::run(&mut back_jobs),
            k if k.starts_with("echo") => commands::handel_echo::run(&k,&mut back_jobs),
            k if k.starts_with("type") => commands::handel_type::run(&k[5..]),
            k if k.starts_with("cd") => commands::handel_cd::run(&k[3..]),
            k if k.starts_with("complete") => {commands::handel_complete::run(&k,&mut rl);},
            _ => {
                let arg = parser::split_by_args(command);
                let re = String::from(">");
                let re2 = String::from("1>");
                let re3 = String::from("2>");

                if arg.len() == 1 {
                    println!("{}: command not found", command);
                } else {
                    if arg.contains(&re) || arg.contains(&re2) || arg.contains(&re3) {
                        let mut file: Vec<String> = Vec::new();
                        file.push(format!("{}", &arg[arg.len() - 1]));

                        executor::check_file("touch", Some(&file),&mut vec![None]);
                    }
                    executor::check_file(&arg[0], Some(&arg[1..]),&mut back_jobs);
                }
            }
        };
    }
}
