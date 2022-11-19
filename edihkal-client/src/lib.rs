mod drugs;
mod edihkal;
mod errors;

pub use edihkal::Client;
pub use errors::Error;

/// Re-exports for client-side apps.
pub use entity::drug::Model as Drug;
pub use entity::drug::NewDrug;
