use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Drug {
    pub name: String,
}

impl Drug {
    pub fn new<S: Into<String>>(name: S) -> Drug {
        Drug { name: name.into() }
    }
}
