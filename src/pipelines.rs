use crate::{
    commands::{handel_jobs, handel_pwd, handel_type},
    shell_helper::ShellHelper,
};
use rustyline::Editor;
use std::{
    io::{self, Write}, process::{Child, Command, Stdio},
};

use crate::commands::{BUILT_IN, handel_complete, handel_echo};

pub fn run(
    cmd: &str,
    back_jobs: &mut Vec<Option<(u8, Child, String)>>,
    rl: &mut Editor<ShellHelper, rustyline::history::DefaultHistory>,
) {
    let cmds: Vec<&str> = cmd.split("|").collect();
    let mut cmds_and_args = Vec::<(&str, Vec<&str>)>::new();
    for i in cmds {
        let k: Vec<&str> = i.trim().split_whitespace().collect();
        if k.len() == 1 {
            cmds_and_args.push((k[0], vec![]));
        } else {
            let mut clean_args = Vec::new();
            for arg in k[1..].to_owned() {
                let cleaned = arg.trim_matches(|x| x == '"' || x == '\'');
                clean_args.push(cleaned);
            }
            cmds_and_args.push((k[0], clean_args));
        }
    }

    if BUILT_IN.contains(&cmds_and_args[0].0) {
        let mut child2 = Command::new(cmds_and_args[1].0);

        if !cmds_and_args[1].1.is_empty() {
            child2.args(cmds_and_args[1].1.to_owned());
        }

        let mut running_child = child2.stdin(Stdio::piped()).spawn().unwrap();

        let mut pipe_to_child2 = running_child.stdin.take().unwrap();
        execute_builtin(cmds_and_args[0].0, &cmds_and_args[0].1, &mut pipe_to_child2, back_jobs, rl);

        drop(pipe_to_child2);

        let _ = running_child.wait().unwrap();
    } else if BUILT_IN.contains(&cmds_and_args[1].0) {

        let mut child1 = Command::new(cmds_and_args[0].0);

        if !cmds_and_args[0].1.is_empty() {
            child1.args(cmds_and_args[0].1.to_owned());
        }

        child1.stdout(Stdio::piped());

        let mut running_child1 = child1.spawn().unwrap();
        let _ = running_child1.stdout.take().unwrap();

        execute_builtin(cmds_and_args[1].0, &cmds_and_args[1].1, &mut io::stdout(), back_jobs, rl);
        
    } else {
        let count = cmds_and_args.len();
        let mut childs = Vec::<Child>::new();

        let mut previous_stdout: Option<std::process::ChildStdout> = None;

        for i in 0..count {
            let (cmd, args) = &cmds_and_args[i];

            let mut command = Command::new(cmd);
            if !args.is_empty() {
                command.args(args.to_owned());
            }

            if let Some(pipe) = previous_stdout {
                command.stdin(Stdio::from(pipe));
            } else {
                command.stdin(Stdio::inherit());
            }

            if i < count - 1 {
                command.stdout(Stdio::piped());
            } else {
                command.stdout(Stdio::inherit());
            }

            let mut child = command.spawn().unwrap();

            if i < count - 1 {
                previous_stdout = Some(child.stdout.take().unwrap());
            } else {
                previous_stdout = None;
            }

            childs.push(child);
        }

        for mut child in childs {
            let _ = child.wait().unwrap();
        }
    }
}

fn execute_builtin<W: Write>(
    builtin_name: &str,
    args: &[&str],
    destination: &mut W,
    back_jobs: &mut Vec<Option<(u8, Child, String)>>,
    rl: &mut Editor<ShellHelper, rustyline::history::DefaultHistory>,
) {
    let mut cmd = builtin_name.to_string();
    args.iter().for_each(|c| {
        cmd.push(' ');
        cmd.push_str(c);
    });

    let cmd = cmd.trim();

    match builtin_name {
        "echo" => handel_echo::run(&cmd.trim(), back_jobs, destination),
        "complete" => handel_complete::run(&cmd.trim(), rl, destination),
        "jobs" => handel_jobs::run(back_jobs, destination),
        "pwd" => handel_pwd::run(destination),
        "type" => handel_type::run(&cmd[5..].trim(), destination),
        _ => {}
    }
}
