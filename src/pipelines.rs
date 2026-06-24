use crate::{
    commands::{handel_jobs, handel_pwd, handel_type},
    shell_helper::ShellHelper,
};
use rustyline::Editor;
use std::{
    io,
    process::{Child, Command, Stdio},
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
            cmds_and_args.push((k[0], k[1..].to_owned()));
        }
    }

    if BUILT_IN.contains(&cmds_and_args[0].0) {
        let mut child2 = Command::new(cmds_and_args[1].0);
        let mut cmd = String::new();
        cmd.push_str(&cmds_and_args[0].0);
        cmd.push(' ');

        for i in &cmds_and_args[0].1 {
            cmd.push_str(i);
            cmd.push(' ');
        }

        if !cmds_and_args[1].1.is_empty() {
            child2.args(cmds_and_args[1].1.to_owned());
        }

        let mut running_child = child2.stdin(Stdio::piped()).spawn().unwrap();

        let mut pipe_to_child2 = running_child.stdin.take().unwrap();

        match cmds_and_args[0].0 {
            "echo" => handel_echo::run(&cmd.trim(), back_jobs, &mut pipe_to_child2),
            "complete" => handel_complete::run(&cmd.trim(), rl, &mut pipe_to_child2),
            "jobs" => handel_jobs::run(back_jobs, &mut pipe_to_child2),
            "pwd" => handel_pwd::run(&mut pipe_to_child2),
            "type" => handel_type::run(&cmd[5..].trim(), &mut pipe_to_child2),
            _ => {}
        }

        drop(pipe_to_child2);

        let _ = running_child.wait().unwrap();
    } else if BUILT_IN.contains(&cmds_and_args[1].0) {
        let mut cmd = String::new();
        cmd.push_str(&cmds_and_args[1].0);
        cmd.push(' ');

        for i in &cmds_and_args[1].1 {
            cmd.push_str(i);
            cmd.push(' ');
        }

        let mut child1 = Command::new(cmds_and_args[0].0);

        if !cmds_and_args[0].1.is_empty() {
            child1.args(cmds_and_args[0].1.to_owned());
        }

        child1.stdout(Stdio::piped());

        let mut running_child1 = child1.spawn().unwrap();
        let _ = running_child1.stdout.take().unwrap();

        match cmds_and_args[1].0 {
            "echo" => handel_echo::run(&cmd.trim(), back_jobs, &mut io::stdout()),
            "complete" => handel_complete::run(&cmd.trim(), rl, &mut io::stdout()),
            "jobs" => handel_jobs::run(back_jobs, &mut io::stdout()),
            "pwd" => handel_pwd::run(&mut io::stdout()),
            "type" => handel_type::run(&cmd[5..].trim(), &mut io::stdout()),
            _ => {}
        }
    } else {
        let mut child1 = Command::new(cmds_and_args[0].0);
        let mut child2 = Command::new(cmds_and_args[1].0);

        if !cmds_and_args[0].1.is_empty() {
            child1.args(cmds_and_args[0].1.to_owned());
        }

        child1.stdout(Stdio::piped());

        let mut running_child1 = child1.spawn().unwrap();
        let child_std = running_child1.stdout.take().unwrap();

        if !cmds_and_args[1].1.is_empty() {
            child2.args(cmds_and_args[1].1.to_owned());
        }

        let mut running_child2 = child2.stdin(child_std).spawn().unwrap();

        let _ = running_child2.wait().unwrap();
    }
}
