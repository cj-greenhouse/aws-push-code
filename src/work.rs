use std::error::Error;
use lambda_runtime::lambda;

use aws_push_code::handler;

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(handler::work_handler);

    Ok(())
}
