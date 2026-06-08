use std::{env, path::Path};

pub fn run(k: &str) {
    match k {
        k if k.starts_with("/") => absolute_path(k),
        k if k.starts_with("~") => home_dir(),
        _ => relative_path(k),
    }
}

fn absolute_path(k: &str) {
    let path = Path::new(k);
    check_path_and_exec(path);
}

fn relative_path(k: &str) {
    let p;
    if !k.starts_with(".") {
        p = format!("./{k}");
    } else {
        p = format!("{k}");
    }

    if p.starts_with("..") {
        let mut splited_dirs: Vec<&str> = p.split("/").collect();
        let mut p = String::new();
        let mut count = 0;
        for i in splited_dirs {
            if i == ".." {
                count += 1;
            } else {
                break;
            }
        }

        let cur = env::current_dir().expect("Unabel to get current dir");
        let cur = cur.to_str();

        let Some(s) = cur else {
            return;
        };

        splited_dirs = s.split("/").collect();
        let mut new_len = splited_dirs.len() - count;
        
        for i in splited_dirs{
            p.push_str(i);
            p.push('/');
            if new_len == 1{
                break;
            }
            new_len-=1;
        }

        let path = Path::new(&p);
        check_path_and_exec(path);

        
    } else {
        let curr = env::current_dir().expect("Unabel to get current dir");
        let p = format!("{}{}", curr.display(), &p[1..]);
        let path = Path::new(&p);
        check_path_and_exec(path);
    }
}

fn home_dir() {
    let home = env::home_dir();
    let Some(home) = home else {
        return;
    };

    let home = format!("{}",home.display());
    let path = Path::new(&home);

    check_path_and_exec(path);
}

fn check_path_and_exec(path: &Path) {
    if path.is_dir() {
        match env::set_current_dir(path) {
            Ok(()) => {}
            Err(e) => eprintln!("Unabel to change the directory:{}", e),
        }
    } else {
        println!("cd: {}: No such file or directory", path.display());
    }
}
