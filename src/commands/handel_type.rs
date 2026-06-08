use is_executable::IsExecutable;
use std::env;

pub fn run(k:&str){
    match k {
        s if k == "exit" || k == "echo" || k == "type" => {
            println!("{s} is a shell builtin")
        }
        _ => check_file(k),
    };
}

fn check_file(k:&str){
    let mut f = false;
    if let Ok(path_spliter) = env::var("PATH") {
        for mut path in env::split_paths(&path_spliter) {
            path.push(k);

            if path.is_file() && path.is_executable() {
                println!("{} is {}", k, path.display());
                f = true;
                break;
            }
        }
    }
    if f == false {
        println!("{}: not found", k);
    }
}