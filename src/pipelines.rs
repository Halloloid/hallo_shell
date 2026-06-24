use std::process::{Command, Stdio};

use crate::commands::BUILT_IN;


pub fn run(cmd: &str) {
    let cmds: Vec<&str> = cmd.split("|").collect();
    let mut cmds_and_args = Vec::<(&str, Vec<&str>)>::new();
    for i in cmds {
        let k: Vec<&str> = i.trim().split_whitespace().collect();
        if k.len() == 1 {
            cmds_and_args.push((k[0], vec![]));
        } else {
            cmds_and_args.push((k[0], k[1..].to_owned()));
        }
    }


    if BUILT_IN.contains(&cmds_and_args[0].0){
        
    }

    let mut child1 = Command::new(cmds_and_args[0].0);
    let mut child2 = Command::new(cmds_and_args[1].0);

    if !cmds_and_args[0].1.is_empty() {
        child1.args(cmds_and_args[0].1.to_owned());
    }

    child1.stdout(Stdio::piped());

    let mut running_child1 = child1.spawn().unwrap();
    let child_std = running_child1.stdout.take().unwrap();

    if !cmds_and_args[1].1.is_empty() {
        child2
            .args(cmds_and_args[1].1.to_owned());
    }

    let mut running_child2 = child2
        .stdin(child_std)
        .spawn()
        .unwrap();

    let _ = running_child2.wait().unwrap();
}
