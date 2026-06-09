pub mod constantfns {
    pub fn split_by_args(command: &str) -> Vec<String> {
        let mut args: Vec<String> = Vec::new();
        let mut curr_arg = String::new();

        let mut in_quote = false;
        for i in command.chars() {
            match i {
                '\'' => in_quote = !in_quote,
                k if k.is_whitespace() => {
                    if !in_quote {
                        args.push(curr_arg);
                        curr_arg = String::new();
                    } else {
                        curr_arg.push(i);
                    }
                }
                _ => {
                    curr_arg.push(i);
                }
            }
        }

        args.push(curr_arg);
        args
    }
}
