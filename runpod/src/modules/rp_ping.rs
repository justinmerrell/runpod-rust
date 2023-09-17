use std::thread;
use std::time::Duration;
use std::env;
use std::sync::{Arc, Mutex};
use std::error::Error;
use std::collections::HashMap;

use crate::common::Job;

use reqwest::{Client, header};
use reqwest::header::{HeaderMap, HeaderValue};



pub fn ping(job_list: Arc<Mutex<HashMap<String, Job>>>)  {
    let api_key = env::var("RUNPOD_AI_API_KEY").unwrap_or_default();
    let mut headers = HeaderMap::new();
    headers.insert(header::AUTHORIZATION, HeaderValue::from_str(&api_key).unwrap());

    let client = Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    let worker_id = env::var("RUNPOD_POD_ID").unwrap_or_else(|_| "WORKER_ID".to_string());
    let ping_url_template = env::var("RUNPOD_WEBHOOK_PING").unwrap_or_else(|_| "PING_NOT_SET".to_string());
    let ping_url = ping_url_template.replace("$RUNPOD_POD_ID", &worker_id);

    let ping_interval_sec: i32 = env::var("RUNPOD_PING_INTERVAL")
        .unwrap_or_else(|_| "10000".to_string())
        .parse::<i32>()
        .unwrap_or(10000)
        / 1000;

    let ping_interval = Duration::from_secs(ping_interval_sec as u64);

    let runtime = tokio::runtime::Runtime::new().unwrap();
    loop {
        let jobs = job_list.lock().unwrap()
            .keys()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        let jobs_str = jobs.join(", ");

        println!("Ping. Simulated.");

        if ping_url != "PING_NOT_SET" {
            let full_ping_url = format!("{}?job_id={}", ping_url, jobs_str);
            match runtime.block_on(send_ping(&client, &full_ping_url)) {
                Ok(_) => println!("Ping sent"),
                Err(e) => println!("Ping failed: {}", e),
            }
        } else {
            println!("Ping not set");
        }

        thread::sleep(ping_interval);
    }
}

async fn send_ping(client: &Client, ping_url: &str) -> Result<(), Box<dyn Error>>  {
    let res = client.get(ping_url).send().await?;
    println!("Ping response: {}", res.status());
    Ok(())
}
