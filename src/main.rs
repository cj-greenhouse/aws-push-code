use aws_push_code::wiring::{Runtime, RuntimeError};
use aws_push_code::submit::Submit;

fn main() -> Result<(), RuntimeError> {

    let params: Vec<String> = std::env::args().collect();

    let runtime = Runtime::new();

    let repo = params.get(1).unwrap();
    let bucket = params.get(2).unwrap();
    let key = params.get(3).unwrap();

    println!("uploading {} to {}/{}", repo, bucket, key);

    runtime.submit_to_pipeline(repo, bucket, key)
}
