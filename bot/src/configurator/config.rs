use crate::configurator::gdrive::GdriveConfig;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServiceConfig {
    GoogleDrive(GdriveConfig)
}
