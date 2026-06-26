use std::collections::HashMap;

pub fn run(k: &str, variables: &mut HashMap<String, String>) {
    if k.contains("-p") {
        let variable = k.strip_prefix("declare -p ").unwrap();
        let s = variables.get(variable);

        if let Some(s) = s {
            println!("declare -- {}=\"{}\"",variable,s);
        } else {
            println!("declare: {}: not found", variable);
        }
    } else if k.contains("=") {
        let key_value: Vec<&str> = k
            .strip_prefix("declare")
            .unwrap()
            .trim()
            .split('=')
            .collect();
        let key = key_value[0].trim();
        let value = key_value[1].trim();

        variables.insert(key.to_string(), value.to_string());
    }
}
