pub fn run(back_jobs: &Vec<Option<(u8, u32, String)>>) {
    let len = back_jobs.len() as u8;

    for i in back_jobs {
        if let Some(job) = i {
            if len == job.0 {
                println!("[{}]+  Running                 {}", job.0, job.2);
            } else if len - 1 == job.0 {
                println!("[{}]-  Running                 {}", job.0, job.2);
            } else {
                println!("[{}]   Running                 {}", job.0, job.2);
            }
        }
    }
}
