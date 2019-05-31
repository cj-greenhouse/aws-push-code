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
    dest_bucket: String,
    dest_key: String,
}

pub fn accept_handler(he: HookEnvelope, _c: Context) -> Result<(), HandlerError> {
    // println!("{}", he.headers);
    // println!("{}", he.queryStringParameters);

    // let body: Value = serde_json::from_str(&he.body).unwrap();
    // println!("{}", body);

    let he: HookEvent = serde_json::from_str(&he.body).unwrap();
    println!("accepting git event: {:?}", he);

    let cf = PushConfig {
        source_url: he.repository.git_http_url,
        dest_bucket: "thesourcebucket".to_owned(),
        dest_key: "thesourcekey".to_owned(),
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
    Ok(())
}
