mod submit;
mod wiring;
pub mod effect;

use crate::submit::Pipeline;

pub fn main() {
    wiring::wire()
        .submit_to_pipeline("","","")
        .unwrap();
}
