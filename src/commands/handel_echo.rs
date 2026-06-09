pub fn run(k: &str) {
    if k.len() > 4 {
        let arg = k[5..].trim();
        let mut s = String::new();
        let mut v: Vec<String> = Vec::new();
        let mut arg = format!("{}'", arg);
        for i in arg.chars() {
            if i == '\'' {
                // v.push(s);
                if !s.trim().is_empty(){
                    v.push(format!("{}",s.trim()));
                }else {
                    v.push(s);
                }
                s = String::new();
            } else {
                s.push(i);
            }
        }
        // println!("{:?}", v);

        s = String::new();
        for i in 0..v.len() - 1 {
            if &v[i] != "" {
                if v[i + 1].trim().is_empty() {
                    s.push_str(&v[i]);
                } else {
                    if v[i].chars().all(char::is_whitespace){
                        s.push(' ');
                    }
                    arg = trim_inside(&v[i]);
                    s.push_str(&arg);
                }
            }
        }

        if &v[v.len() - 1] != "" {
            arg = trim_inside(&v[v.len() - 1]);
            s.push_str(&arg);
        }

        println!("{}", s);
    }
}

fn trim_inside(k: &str) -> String {
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
