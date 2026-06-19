
use codecrafters_shell::shell_helper::ShellHelper;
use rustyline::Editor;

pub fn run(cmd:&str,rl:&mut Editor<ShellHelper,rustyline::history::DefaultHistory>){
    
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
                 println!("complete -C \'{}\' {}",path,command);
            }else {
                eprintln!("complete: {}: no completion specification",command);
            }
        }

    }
}