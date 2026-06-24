use std::{process::Child,io::Write};

use crate::{parser::split_by_args,redirect,executor};

pub fn run<W:Write>(k: &str,back_jobs:&mut Vec<Option<(u8,Child,String)>>,destination:&mut W) {
    if k.len() > 4 {
        let arg = k[5..].trim();
        if arg.contains(">") && !arg.contains("2>") {
            redirect_output(arg);
        } else if arg.contains("2>") {
            let chr = if arg.contains("2>>") { "2>>" } else { "2>" };
            let split_by_red: Vec<&str> = arg.split(chr).collect();
            let mut file: Vec<String> = Vec::new();
            file.push(format!("{}", split_by_red[1].trim()));
            executor::check_file("touch", Some(&file),&mut vec![None]);
            redirect::store_in_file(String::from(""), &file[0], false);
            for_backslash(split_by_red[0],destination);
        } else {
            for_backslash(arg,destination);
        }

        let mut index_to_remove = Vec::new();
        for (index,job) in back_jobs.iter_mut().enumerate(){
            if let Some(job) = job{
                match job.1.try_wait() {
                    Ok(Some(_status))=>{
                        writeln!(destination,"[{}]+  Done                 {}",job.0,&job.2[..job.2.len()-2]).unwrap();
    
                        index_to_remove.push(index);
                    },
                    _ => {}
                }
            }
        }

        for i in index_to_remove.into_iter().rev(){
            back_jobs.remove(i);
        }
    }
}

fn for_backslash<W:Write>(arg: &str,destination:&mut W) {
    let v = split_by_args(arg);
    let mut msg = String::new();
    for c in &v {
        if !c.is_empty() {
            msg.push_str(c);
            msg.push(' ');
        }
    }

    writeln!(destination,"{}", msg.trim()).unwrap();
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

    executor::check_file("touch", Some(&file),&mut vec![None]);

    redirect::store_in_file(msg, &file[0], append);
}
