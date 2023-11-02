use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EarlyCod {
    #[serde(rename = "is_activated")]
    pub(crate) is_activated: Option<String>,
    #[serde(rename = "terms_timestamp")]
    pub(crate) terms_timestamp: Option<String>,
    #[serde(rename = "user")]
    pub(crate) user: Option<String>,
    #[serde(rename = "remarks")]
    pub(crate) remarks: Option<String>,
    #[serde(rename = "plan")]
    pub(crate) plan: Option<String>,
    #[serde(rename = "email")]
    pub(crate) email:Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct early_cod_data {
    pub(crate) is_activated: String,
    pub(crate) terms_timestamp: String,
    pub(crate) user: String,
    pub(crate) remarks: String,
    pub(crate) plan: String,
    pub(crate) email: String,
}

#[derive(Serialize, Deserialize)]
pub struct early_cod_resp {
    pub status: bool
}


