pub fn run(cmd:&str){
    if cmd.contains("-p"){
        let idx = cmd.find("-p").unwrap();
        let command = &cmd[idx+3..];
        eprintln!("complete: {}: no completion specification",command);
    }
}