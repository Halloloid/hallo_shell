use std::process::Child;

pub fn run(back_jobs: &mut Vec<Option<(u8, Child, String)>>) {
    let total_jobs = back_jobs.len();
    let mut index_to_remove = Vec::new();

    for (index, jobk) in back_jobs.iter_mut().enumerate() {
        if let Some(job) = jobk {
            let sign = if total_jobs-1 == index{
                "+"
            } else if total_jobs - 2 == index {
                "-"
            } else {
                " "
            };
            
            match job.1.try_wait() {
                Ok(None) => {
                    println!("[{}]{}  Running                 {}", job.0, sign, job.2);
                }
                Ok(Some(_status)) => {
                    println!("[{}]{}  Done                 {}",job.0,sign,&job.2[..job.2.len()-2]);

                    index_to_remove.push(index);
                }
                Err(e) => println!("Error Checking Child State:{}",e),
            }
        }
    }

    for i in index_to_remove.into_iter().rev(){
        back_jobs.remove(i);
    }
}
