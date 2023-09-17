use std::env;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use std::collections::HashMap;
use crate::common::Job;

use reqwest::{Client, header};
use reqwest::header::{HeaderMap, HeaderValue};

const SLEEP_TIME: Duration = Duration::from_secs(1);

pub fn job_returner(job_list: Arc<Mutex<HashMap<String, Job>>>) {
    let api_key = env::var("RUNPOD_AI_API_KEY").unwrap_or_default();
    let mut headers = HeaderMap::new();
    headers.insert(header::AUTHORIZATION, HeaderValue::from_str(&api_key).unwrap());

    let client = Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    let worker_id = env::var("RUNPOD_POD_ID").unwrap_or_else(|_| "WORKER_ID".to_string());
    let job_done_url_template = env::var("RUNPOD_WEBHOOK_POST_OUTPUT").unwrap_or_else(|_| "JOB_DONE_URL".to_string());
    let job_done_url = job_done_url_template.replace("$RUNPOD_POD_ID", &worker_id);


    let runtime = tokio::runtime::Runtime::new().unwrap();
    loop {
        for job in job_list.lock().unwrap().values() {
            if job.input.contains_key("output") {

                match runtime.block_on(return_job(&client, &job_done_url, &job)) {
                    Ok(_) => println!("Job {} returned", job.id),
                    Err(e) => println!("Job {} failed: {}", job.id, e),
                }

                job_list.lock().unwrap().remove(&job.id);
            }
        }

        sleep(SLEEP_TIME);
    }
}

async fn return_job(client: &Client, job_done_url: &str, job: &Job) -> Result<(), Box<dyn Error>> {
    let response = client.post(job_done_url).json(&job.input).send().await?;

    println!("Job {} returned with status {}", job.id, response.status());
    Ok(())
}
