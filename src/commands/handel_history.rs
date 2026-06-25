pub fn run(arg:&str,historys:&mut Vec<String>){
    
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