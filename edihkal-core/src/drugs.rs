use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Drug {
    pub name: String,
}
