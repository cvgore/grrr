use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct GdriveConfig {
    token: String,
    root_folder_id: Option(String)
}
