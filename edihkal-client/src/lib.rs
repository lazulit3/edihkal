mod drugs;
mod entries;

mod edihkal;
mod errors;

pub use edihkal::Client;
pub use errors::Error;

pub use entity::NewDrug;
/// Re-exports for client-side apps.
pub use entity::NewEntry;
