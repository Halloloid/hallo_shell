use std::fs;

pub fn run(arg:&str,historys:&mut Vec<String>){

    if arg.contains("-r"){
        read_from_file(arg, historys);
        return;
    }else if arg.contains("-w") {
        write_from_file(arg, historys);
        return;
    }else if arg.contains("-a") {
        append_from_file(arg, historys);
        return;
    }
    
    let limit :usize = arg.trim().parse().unwrap_or_default();
    let down = if limit != 0{
        historys.len()-limit
    }else {
        0
    };
    
    let historys = historys[down..].to_vec(); 
    
    for (no,i) in historys.iter().enumerate(){
        println!("  {}  {}",no+down,i);
    }
}

fn read_from_file(arg:&str,historys:&mut Vec<String>){
    let path = arg.trim()[2..].trim();

    let data = fs::read_to_string(path).unwrap();

    for line in data.lines(){
        if !line.is_empty(){
            historys.push(line.trim().to_string());
        }
    }
}

fn write_from_file(arg:&str,historys:&mut Vec<String>){
    let path = arg.trim()[2..].trim();

    let mut contents = String::new();
    historys.iter().for_each(|history| {
        contents.push_str(history);
        contents.push('\n');
    });

    fs::write(path, contents).unwrap();
}

fn append_from_file(arg:&str,historys:&mut Vec<String>){
    let path = arg.trim()[2..].trim();

    let data = fs::read_to_string(path).unwrap();
    let mut contents = String::new();
    
    contents.push_str(&data);

    if !data.contains("history -a"){
        historys.iter().for_each(|history| {
            contents.push_str(history);
            contents.push('\n');
        });
        
    }else {
        let historys2 = &historys[..historys.len()-2];

        let idx = historys2.iter().rposition(|s| s.starts_with("history -a")).unwrap();
        
        let historys3 = &historys[idx+1..];
        
        historys3.iter().for_each(|history| {
            contents.push_str(history);
            contents.push('\n');
        });

    }
    fs::write(path, contents).unwrap();
}