#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct session_expired{
    pub(crate) status: bool,
    pub(crate) session_expired: bool,
    pub(crate) msg: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetSession{
    pub(crate) session_id: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct session_data{
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) email: String,
    pub(crate) dp: String,
    pub(crate) id: String,
    pub(crate) password: String,
    pub(crate) is_mobile_verified: bool,
    pub(crate) is_email_verified: bool,
}

#[derive(Deserialize)]
pub struct general_signup_payload{
    #[serde(rename = "first_name")]
    pub first_name: Option<String>,
    #[serde(rename = "last_name")]
    pub last_name: Option<String>,
    #[serde(rename = "email")]
    pub email: Option<String>,
    #[serde(rename = "password")]
    pub password: Option<String>,
    #[serde(rename = "pickup_addresses")]
    pub pickup_addresses: Option<String>, //Option<Vec<PickupAddress>>,
    #[serde(rename = "bank_details")]
    pub bank_details: Option<String>,//Option<BankDetails>
}

// #[derive(Deserialize)]
// pub struct PickupAddress {
//     #[serde(rename = "address_name")]
//     address_name: Option<String>,
//     #[serde(rename = "poc_name")]
//     poc_name: Option<String>,
//     #[serde(rename = "poc_mobile")]
//     poc_mobile: Option<String>,
//     #[serde(rename = "poc_email")]
//     poc_email: Option<String>,
//     #[serde(rename = "poc_alt_mobile")]
//     poc_alt_mobile: Option<String>,
//     #[serde(rename = "address_1")]
//     address_1: Option<String>,
//     #[serde(rename = "address_2")]
//     address_2: Option<String>,
//     #[serde(rename = "landmark")]
//     landmark: Option<String>,
//     #[serde(rename = "pincode")]
//     pincode: Option<String>,
//     #[serde(rename = "city")]
//     city: Option<String>,
//     #[serde(rename = "state")]
//     state: Option<String>,
//     #[serde(rename = "open_at")]
//     open_at: Option<String>,
//     #[serde(rename = "close_at")]
//     close_at: Option<String>,
//     #[serde(rename = "is_vendor_address")]
//     is_vendor_address: Option<bool>,
//     #[serde(rename = "vendor_name")]
//     vendor_name: Option<String>,
//     #[serde(rename = "vendor_gst")]
//     vendor_gst: Option<String>,
//     #[serde(rename = "is_rto_different")]
//     is_rto_different: Option<bool>,
//     #[serde(rename = "rto_address")]
//     rto_address: Option<String>,
//     #[serde(rename = "create_timestamp")]
//     create_timestamp: Option<i64>,
// }
// #[derive(Deserialize)]
// pub struct BankDetails {
//     #[serde(rename = "account_name")]
//     account_name: Option<String>,
//     #[serde(rename = "account_no")]
//     account_no: Option<String>,
//     #[serde(rename = "account_type")]
//     account_type: Option<String>,
//     #[serde(rename = "bank_name")]
//     bank_name: Option<String>,
//     #[serde(rename = "bank_branch")]
//     bank_branch: Option<String>,
//     #[serde(rename = "ifsc_code")]
//     ifsc_code: Option<String>,
//     #[serde(rename = "is_account_activated")]
//     is_account_activated: Option<bool>,
// }

#[derive(Deserialize)]
pub struct general_signin_payload{
    #[serde(rename = "email")]
    pub email: Option<String>,
    #[serde(rename = "password")]
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct signup_success{
    pub status: bool,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct user_claims{
    #[serde(rename = "first_name")]
    pub(crate) first_name: String,
    #[serde(rename = "last_name")]
    pub(crate) last_name: String,
    #[serde(rename = "email")]
    pub(crate) email: String,
    #[serde(rename = "dp")]
    pub(crate) dp: String,
    pub(crate) session_time: Option<i64>,
    pub(crate) exp: Option<usize>,
}