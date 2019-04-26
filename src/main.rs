mod submit;
mod wiring;
pub mod effect;


pub fn main() {
    submit::submit_to_pipeline::<wiring::Runtime, ()>(&wiring::wire(), "", "", "").unwrap();
}
