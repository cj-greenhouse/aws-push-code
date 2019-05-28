// mod submit;
// mod wiring;
// pub mod effect;

// use crate::submit::Submit;
use std::error::Error;

use lambda_runtime::{error::HandlerError, lambda, Context};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value,};

// pub fn main() {
//     wiring::wire().submit_to_pipeline("", "", "").unwrap();
// }

#[derive(Deserialize, Debug)]
struct HookEnvelope {
    headers: Value,
    queryStringParameters: Value,
    body: String,
}

#[derive(Deserialize, Debug)]
struct Repository {
    git_http_url: String,
}

#[derive(Deserialize, Debug)]
struct HookEvent {
    #[serde(rename = "ref")]
    repo_ref: String,
    repository: Repository,
}


#[derive(Serialize, Debug)]
struct PushConfig {
    source_url: String,
    dest_bucket: String,
    dest_key: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(handler);

    Ok(())
}

fn handler(he: HookEnvelope, _c: Context) -> Result<(), HandlerError> {

    println!("{}", he.headers);
    println!("{}", he.queryStringParameters);

    let body: Value = serde_json::from_str(&he.body).unwrap();
    println!("{}", body);

    let he: HookEvent = serde_json::from_str(&he.body).unwrap();
    println!("hook: {:?}", he);

    let cf = PushConfig {
        source_url: he.repository.git_http_url,
        dest_bucket: "thesourcebucket".to_owned(),
        dest_key: "thesourcekey".to_owned(),
    };
    println!("config: {:?}", cf);
    Ok(())
}
