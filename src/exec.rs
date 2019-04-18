use crate::wiring;
use crate::flow;

pub fn exec() {
    let mut w = wiring::wire();
    flow::submit_to_pipeline(&mut w, "a repo").unwrap();
}
