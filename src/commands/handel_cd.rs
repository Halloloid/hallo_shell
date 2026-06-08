use std::{env,path::Path};

pub fn run(k:&str){
    match k{
        k if k.starts_with("/") => absolute_path(k),
        k if k.starts_with(".") => relative_path(k),
        _ => home_dir(),
    }
}

fn absolute_path(k:&str){
    let path = Path::new(k);
    if path.is_dir(){
        match env::set_current_dir(path){
            Ok(()) => {},
            Err(e) => eprintln!("Unabel to change the directory:{}",e),
        }
    }else {
        println!("cd: {}: No such file or directory",path.display());
    }
}

fn relative_path(k:&str){
    
}

fn home_dir(){
    
}