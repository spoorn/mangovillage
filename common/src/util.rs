use std::fmt::Display;
use tracing::error;

pub fn validate_register_results<T: Display>(is_client: bool, results: Vec<Result<(), T>>) -> bool {
    let mut has_error = false;
    for result in results.iter() {
        if let Err(e) = result {
            error!("[{}] Error registering packet: {}", if is_client { "client" } else { "server" }, e);
            has_error = true;
        }
    }
    !has_error
}