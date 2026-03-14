use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub github: Option<String>,
    pub live: Option<String>,
    pub tech: Vec<String>,
    #[serde(rename = "category")]
    pub category: String,
}
