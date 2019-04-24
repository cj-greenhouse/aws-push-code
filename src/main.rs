mod submit;
mod wiring;
pub mod effect;

use crate::wiring::{Runtime, RuntimeError};

pub fn main() {
    let wiring = wiring::wire();
    submit::submit_to_pipeline::<Runtime, RuntimeError>(&wiring, "","","").unwrap();
}
