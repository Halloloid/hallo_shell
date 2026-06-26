pub fn run(k:&str){
    if k.contains("-p"){
        let variable = k.strip_prefix("declare -p ").unwrap();
        println!("declare: {}: not found",variable);
    }
}