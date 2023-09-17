use serde_derive::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Job {
    pub id: String,
    pub input: HashMap<String, String>,
    pub output: Option<HashMap<String, String>>,
}
