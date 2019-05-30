use aws_push_code::wiring::{Runtime, RuntimeError};
use aws_push_code::submit::Submit;

fn main() -> Result<(), RuntimeError> {

    let runtime = Runtime::new();
    runtime.submit_to_pipeline("gitlaburl", "sourcebucket", "sourcekey")
}
