use std::collections::HashMap;

pub fn check_for_expansion(cmd: &str, variables: &mut HashMap<String, String>) -> String {
    let mut new_cmd = String::new();
    let cmd_and_args: Vec<&str> = cmd.split(' ').collect();
    new_cmd.push_str(cmd_and_args[0]);
    new_cmd.push(' ');

    for i in &cmd_and_args[1..] {
        if !i.contains('$') {
            new_cmd.push_str(i);
            new_cmd.push(' ');
        } else {
            let vars: Vec<&str> = i.split('$').collect();
            new_cmd.push_str(vars[0]);
            let value = variables.get(vars[1]).unwrap();
            new_cmd.push_str(value);
            new_cmd.push(' ');
        }
    }

    new_cmd
}
