pub mod w3;
pub use w3::*;

pub fn throw_error(input: InputThrowError) -> String {
    match input.arg {
        Some(s) => s,
        None => {
            panic!("Panic here...");
        }
    }
}
