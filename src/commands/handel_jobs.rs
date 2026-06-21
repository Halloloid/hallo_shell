pub fn run(back_jobs:&Vec<Option<(u8,u32,String)>>){
    for i in back_jobs{
        if let Some(job) = i {
            println!("[{}]+  Running                 {}",job.0,job.2);
        }
    }
}