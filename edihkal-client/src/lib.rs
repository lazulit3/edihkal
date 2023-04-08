mod drugs;
mod entries;

mod edihkal;
mod errors;

pub use edihkal::Client;
pub use errors::Error;

// Re-export NewModel types from `entity` crate for frontend/client usage.
pub mod entity {
    pub mod drug {
        pub use entity::drug::Model;
        pub use entity::drug::NewModel;
    }
    pub mod entry {
        pub use entity::entry::Model;
        pub use entity::entry::NewModel;
    }
}
