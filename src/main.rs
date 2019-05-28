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

fn handler2(he: Value, _: Context) -> Result<Value, HandlerError> {
    println!("{}", he);
    let body = he.get("body").unwrap().as_str().unwrap().to_string();
    let body: Value = serde_json::from_str(&body).unwrap();
    println!("{}", body);
    Ok(json!({
        "statusCode": 200,
        "body": "{}"
        // "body": serde_json::to_value(cf).unwrap().to_string(),
    }))
}

fn handler(he: HookEnvelope, _c: Context) -> Result<Value, HandlerError> {

    let he: HookEvent = serde_json::from_str(&he.body).unwrap();
    println!("hook: {:?}", he);

    let cf = PushConfig {
        source_url: he.repository.git_http_url,
        dest_bucket: "thesourcebucket".to_owned(),
        dest_key: "thesourcekey".to_owned(),
    };
    println!("config: {:?}", cf);
    Ok(json!({
        "statusCode": 200,
        "body": "{}"
        // "body": serde_json::to_value(cf).unwrap().to_string(),
    }))
}
