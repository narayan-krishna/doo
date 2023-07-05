use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Layout {
    pub vertical: String,
    pub horizontal: String,
}

#[derive(Serialize, Deserialize)]
pub struct DooConfig {
    pub layout: Layout,
}

impl std::default::Default for DooConfig {
    fn default() -> Self {
        Self {
            layout: Layout {
                vertical: String::from("center"),
                horizontal: String::from("center"),
            }
        }
    }
}
