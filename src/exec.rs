use crate::wiring::{self, Runtime, RuntimeError};
use crate::flow;

pub fn exec() {
    let wiring = wiring::wire();
    flow::submit_to_pipeline::<Runtime, RuntimeError>(&wiring, "","","").unwrap();
}
