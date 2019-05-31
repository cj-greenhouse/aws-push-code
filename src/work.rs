use lambda_runtime::lambda;
use std::error::Error;

use aws_push_code::handler;

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(handler::work_handler);

    Ok(())
}
