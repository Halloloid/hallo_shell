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
            let mut value = vars[1];
            let mut extra = "";
            if value.contains('{') && value.contains('}') {
                let start = value.find('{').unwrap();
                let end = value.find('}').unwrap();
                extra = &value[end + 1..];
                value = &value[start + 1..end];
            }
            let value = variables.get(value).unwrap();
            new_cmd.push_str(value);
            new_cmd.push_str(extra);
            new_cmd.push(' ');
        }
    }

    new_cmd
}
