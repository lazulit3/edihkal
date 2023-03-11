mod drugs;
mod entries;

mod edihkal;
mod errors;

pub use edihkal::Client;
pub use errors::Error;

/// Re-exports for client-side apps.
pub use entity::NewDrug;
pub use entity::NewEntry;
