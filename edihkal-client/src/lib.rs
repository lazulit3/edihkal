mod drugs;
mod edihkal;
mod errors;

pub use edihkal::Client;
/// Re-export drug Model as Drug for client-side apps.
pub use entity::drug::Model as Drug;
pub use errors::Error;
