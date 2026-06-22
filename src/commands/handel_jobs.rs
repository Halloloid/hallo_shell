use std::process::Child;

pub fn run(back_jobs: &mut Vec<Option<(u8, Child, String)>>) {
    let total_jobs = back_jobs.len() as u8;
    let mut index_to_remove = Vec::new();


    for (index, job) in back_jobs.iter_mut().enumerate() {
        if let Some(job) = job {
            match job.1.try_wait() {
                Ok(None) => {
                    let sign = if total_jobs == job.0 {
                        "+"
                    } else if total_jobs - 1 == job.0 {
                        "-"
                    } else {
                        " "
                    };

                    println!("[{}]{}  Running                 {}", job.0, sign, job.2);
                }
                Ok(Some(_status)) => {
                    println!("[{}]+  Done                 {}",job.0,&job.2[..job.2.len()-2]);

                    index_to_remove.push(index);
                }
                Err(e) => println!("Error Checking Child State:{}",e),
            }
        }
    }

    for i in index_to_remove{
        back_jobs.remove(i);
    }
}
