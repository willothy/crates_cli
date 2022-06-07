//! Loading utilities
use std::thread::JoinHandle;

use spinners::{Spinner, Spinners};

pub fn load_until_join<T>(
    handle: JoinHandle<Result<T, String>>,
    message: String,
) -> Result<T, String> {
    let mut sp = Spinner::new(Spinners::Line, message);
    match handle.join() {
        Ok(result) => match result {
            Ok(result) => {
                sp.stop();
                Ok(result)
            }
            Err(e) => Err(e),
        },
        Err(_) => Err("Network thread could not rejoin".to_owned()),
    }
}
