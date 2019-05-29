use lambda_runtime::{error::HandlerError, Context};
use rusoto_core::Region;
use rusoto_sqs::{Sqs, SqsClient, SendMessageRequest};
use serde::{Deserialize, Serialize};
use serde_json::Value;
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

#[derive(Serialize, Debug)]
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

    println!("performing work: {}", work);
    Ok(())
}
