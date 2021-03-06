use crate::submit::Submit;
use crate::wiring::Runtime;
use lambda_runtime::{error::HandlerError, Context};
use rusoto_core::Region;
use rusoto_sqs::{SendMessageRequest, Sqs, SqsClient};
use serde::{Deserialize, Serialize};
use serde_json::{map::Map, Value};
use std::env;

#[derive(Deserialize, Debug)]
pub struct HookEnvelope {
    headers: Value,
    #[serde(rename = "queryStringParameters")]
    query_string_parameters: Value,
    body: String,
}

#[derive(Deserialize, Debug)]
pub struct Repository {
    git_http_url: String,
    git_ssh_url: String,
}

#[derive(Deserialize, Debug)]
pub struct HookEvent {
    #[serde(rename = "ref")]
    repo_ref: String,
    repository: Repository,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PushConfig {
    source_url: String,
    source_target: String,
    dest_bucket: String,
    dest_key: String,
}

pub fn accept_handler(he: HookEnvelope, _c: Context) -> Result<(), HandlerError> {
    let he: HookEvent = serde_json::from_str(&he.body).unwrap();
    println!("accepting git event: {:?}", he);

    const BRANCH_PREFIX: &str = "refs/heads/";
    let branch = if he.repo_ref.starts_with(BRANCH_PREFIX) {
        he.repo_ref.trim_start_matches(BRANCH_PREFIX)
    } else {
        "master"
    };

    let cf = PushConfig {
        source_url: he.repository.git_ssh_url,
        source_target: branch.to_owned(),
        dest_bucket: env::var("CJ_PUSHCODE_SOURCE_BUCKET").unwrap(),
        dest_key: format!("{}.zip", branch),
    };

    let sqs = SqsClient::new(Region::default());
    let msg = SendMessageRequest::default();
    let msg = SendMessageRequest {
        message_body: serde_json::to_string(&cf).unwrap(),
        queue_url: env::var("PUSHCODE_WORK_QUEUE").unwrap(),
        ..msg
    };
    let res = sqs.send_message(msg);
    println!("message sent");
    let res = res.sync().unwrap();
    println!("send worked {:?}", res);

    Ok(())
}

pub fn work_handler(work: Value, _c: Context) -> Result<(), HandlerError> {
    let work = work.get("Records").unwrap();
    let work = work.as_array().unwrap();
    let work: Vec<&Map<String, Value>> = work.iter().map(|v| v.as_object().unwrap()).collect();
    let work: Vec<&str> = work
        .iter()
        .map(|v| v.get("body").unwrap())
        .map(|s| s.as_str().unwrap())
        .collect();
    let work: Vec<PushConfig> = work
        .iter()
        .map(|j| serde_json::from_str(j).unwrap())
        .collect();

    println!("performing work: {:?}", work);
    let runtime = Runtime::default();
    for work in work {
        runtime
            .submit_to_pipeline(
                &work.source_url,
                &work.source_target,
                &work.dest_bucket,
                &work.dest_key,
            )
            .unwrap();
    }
    Ok(())
}
