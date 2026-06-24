use std::{process::Child,io::Write};

pub fn run<W:Write>(back_jobs: &mut Vec<Option<(u8, Child, String)>>,desination: &mut W) {
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
                    writeln!(desination,"[{}]{}  Running                 {}", job.0, sign, job.2).unwrap();
                }
                Ok(Some(_status)) => {
                    writeln!(desination,"[{}]{}  Done                 {}",job.0,sign,&job.2[..job.2.len()-2]).unwrap();

                    index_to_remove.push(index);
                }
                Err(e) => eprintln!("Error Checking Child State:{}",e),
            }
        }
    }

    for i in index_to_remove.into_iter().rev(){
        back_jobs.remove(i);
    }
}
