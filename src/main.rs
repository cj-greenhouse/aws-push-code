use aws_push_code::submit::Submit;
use aws_push_code::wiring::{Runtime, RuntimeError};

fn main() -> Result<(), RuntimeError> {
    let params: Vec<String> = std::env::args().collect();

    let runtime = Runtime::default();

    let repo = params.get(1).unwrap();
    let bucket = params.get(2).unwrap();
    let key = params.get(3).unwrap();

    println!("uploading {} to {}/{}", repo, bucket, key);

    runtime.submit_to_pipeline(repo, "master", bucket, key)
}
