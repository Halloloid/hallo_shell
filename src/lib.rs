pub mod parser;
pub mod executor;
pub mod redirect;
pub mod shell_helper;
pub mod completion;
pub mod pipelines;
pub mod commands;

pub mod constantfns {
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
