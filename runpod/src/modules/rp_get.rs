// Fetches jobs from the server and adds them to the job list.
// Only gets a job if the job list is less than the concurrency limit.

use std::time::Duration;
use std::thread;
use std::env;
use std::sync::{Arc, Mutex};
use std::error::Error;
use std::collections::HashMap;

use futures::future::join_all;
use reqwest::{Client, header};
use reqwest::header::{HeaderMap, HeaderValue};

use crate::common::Job;


pub fn get_jobs(job_list:Arc<Mutex<HashMap<String, Job>>>, concurrency_limit: usize){
    let worker_id = env::var("RUNPOD_POD_ID").unwrap_or_else(|_| "WORKER_ID_NOT_SET".to_string());

    let api_key = env::var("RUNPOD_AI_API_KEY").unwrap_or_default();
    let mut headers = HeaderMap::new();
    headers.insert(header::AUTHORIZATION, HeaderValue::from_str(&api_key).unwrap());

    let client = Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    let job_get_url = env::var("RUNPOD_WEBHOOK_GET_JOB").unwrap_or_else(|_| "GET_NOT_SET".to_string())
        .replace("$ID", &worker_id);

    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(get_job_controller(&client, &job_get_url, job_list, concurrency_limit));
}

async fn get_job_controller(client: &Client, job_get_url: &str, job_list: Arc<Mutex<HashMap<String, Job>>>, concurrency_limit: usize) {
    loop {
        let active_jobs = { job_list.lock().unwrap().len() };

        if active_jobs == 0 {
            let constructed_url = format!("{}?job_in_progress={}", job_get_url, 0);

            match get_job(Arc::clone(&job_list), &client, &constructed_url).await {
                Ok(_) => println!("Job received"),
                Err(e) => println!("Job failed: {}", e),
            }

        } else {
            let constructed_url = format!("{}?job_in_progress={}", job_get_url, 1);

            let futures: Vec<_> = (active_jobs..concurrency_limit)
                .map(|_| get_job(Arc::clone(&job_list), &client, &constructed_url))
                .collect();
            let results: Vec<_> = join_all(futures).await;

            for result in results {
                match result {
                    Ok(_) => println!("Job received"),
                    Err(e) => println!("Job failed: {}", e),
                }
            }
        }

        thread::sleep(Duration::from_secs(1));
    }
}

async fn get_job(job_list:Arc<Mutex<HashMap<String, Job>>>, client: &Client, job_get_url: &str) -> Result<(), Box<dyn Error>> {
    let res = client.get(job_get_url).send().await?;
    println!("Get job response: {}", res.status());

    if res.status().is_success() {
        if let Ok(job_data) = res.json::<Job>().await {
            let mut jobs = job_list.lock().unwrap();
            println!("Job received: {}", job_data.id);
            jobs.insert(job_data.id.clone(), job_data);
        } else {
            println!("Failed to parse job data");
        }
    }

    Ok(())
}
