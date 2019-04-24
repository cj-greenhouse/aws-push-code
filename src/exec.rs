use crate::wiring::{Runtime, RuntimeError};
use crate::flow;

pub fn exec() {
    flow::submit_to_pipeline::<Runtime, RuntimeError>("","","").unwrap();
}
