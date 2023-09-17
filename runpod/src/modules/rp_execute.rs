use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use std::collections::HashMap;
use crate::common::Job;

const SLEEP_TIME: Duration = Duration::from_secs(1);

pub fn job_runner(job_list: Arc<Mutex<HashMap<String, Job>>>) {
    loop {
        for job in job_list.lock().unwrap().values() {
            if !job.input.contains_key("output") {
                match run_job(&job) {
                    Ok(_) => println!("Job {} finished", job.id),
                    Err(e) => println!("Job {} failed: {}", job.id, e),
                }
            }
        }

        sleep(SLEEP_TIME);
    }
}


fn run_job(job: &Job) -> Result<(), Box<dyn Error>> {
    let job_id = &job.id;
    let job_input = &job.input;

    println!("Job {} started", job_id);
    println!("Job input: {:?}", job_input);

    Ok(())
}
