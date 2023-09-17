
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

mod common;
mod modules;

use crate::modules::rp_ping::ping;

const SLEEP_TIME: Duration = Duration::from_secs(5);

fn main() {
    let concurrency_limit: usize = 1;
    let job_list = Arc::new(Mutex::new(HashMap::new()));

    let ping_job_list = Arc::clone(&job_list);
    thread::spawn(|| {
        ping(ping_job_list);
    });

    let get_job_list = Arc::clone(&job_list);
    thread::spawn(move|| {
        modules::rp_get::get_jobs(get_job_list, concurrency_limit);
    });

    let run_job_list = Arc::clone(&job_list);
    thread::spawn(move|| {
        modules::rp_execute::job_runner(run_job_list);
    });

    let return_job_list = Arc::clone(&job_list);
    thread::spawn(move|| {
        modules::rp_return::job_returner(return_job_list);
    });

    loop {
        println!("Hello, world!");
        thread::sleep(SLEEP_TIME);
    }
}
