pub fn run(historys:&mut Vec<String>){
    for (no,i) in historys.iter().enumerate(){
        println!("  {}  {}",no+1,i);
    }
}