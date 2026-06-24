
use crate::shell_helper::ShellHelper;
use rustyline::Editor;
use std::io::Write;

pub fn run<W:Write>(cmd:&str,rl:&mut Editor<ShellHelper,rustyline::history::DefaultHistory>,destination:&mut W){
    
    if cmd.contains("-C"){
        let idx = cmd.rfind("-C").unwrap();
        let v= &cmd[idx+3..].trim();
        let v:Vec<&str> = v.split(' ').collect();
        let v:Vec<&&str> = v.iter().filter(|x| !x.is_empty()).collect();
        let (path,command) = (v[0].trim().to_string(),v[1].trim().to_string());

        if let Some(helper) = rl.helper_mut(){
            helper.completer.completeion.insert(command,path);
        }
    
    } else if cmd.contains("-p"){
        let idx = cmd.find("-p").unwrap();
        let command = &cmd[idx+3..];
        let command = command.to_string();
        

        if let Some(helper) = rl.helper_mut(){
            if let Some(path) = helper.completer.completeion.get(&command){
                 writeln!(destination,"complete -C \'{}\' {}",path,command).unwrap();
            }else {
                eprintln!("complete: {}: no completion specification",command);
            }
        }

    } else if cmd.contains("-r"){
        let idx = cmd.find("-r").unwrap();
        let command = &cmd[idx+3..];
        let command = command.to_string();

        if let Some(helper) = rl.helper_mut(){
            let _ = helper.completer.completeion.remove(&command);
        }
    }
}