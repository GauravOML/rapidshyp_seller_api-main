use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct bank_data_details{
    #[serde(rename = "account_name")]
    pub(crate) account_name: String,
    #[serde(rename = "account_no")]
    pub(crate) account_no: String,
    #[serde(rename = "account_type")]
    pub(crate) account_type: String,
    #[serde(rename = "ifsc_code")]
    pub(crate) ifsc_code: String,
    #[serde(rename = "bank_name")]
    pub(crate) bank_name: String,
    #[serde(rename = "bank_branch")]
    pub(crate) bank_branch: String,
    #[serde(rename = "is_account_activated")]
    pub(crate) is_account_activated: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct bank_claims{
    #[serde(rename = "email")]
    pub(crate) email: String,
    #[serde(rename = "account_name")]
    pub(crate) account_name: String,
    #[serde(rename = "account_no")]
    pub(crate) account_no: String,
    #[serde(rename = "account_type")]
    pub(crate) account_type: String,
    #[serde(rename = "ifsc_code")]
    pub(crate) ifsc_code: String,
    #[serde(rename = "bank_name")]
    pub(crate) bank_name: String,
    #[serde(rename = "bank_branch")]
    pub(crate) bank_branch: String,
    #[serde(rename = "is_account_activated")]
    pub(crate) is_account_activated: String
}


#[derive(Serialize)]
pub struct bank_details_resp{
    pub(crate) status: bool,
    pub(crate) profile: bank_data_details
}

