use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct get_seller_profile{
    #[serde(rename = "first_name")]
    pub first_name: Option<String>,
    #[serde(rename = "last_name")]
    pub last_name: Option<String>,
    #[serde(rename = "email")]
    pub email: Option<String>,
    #[serde(rename = "dp")]
    pub dp: Option<String>,
    #[serde(rename = "created_date")]
    pub created_date: Option<i64>,
    #[serde(rename = "last_login")]
    pub last_login: Option<i64>,
    #[serde(rename = "is_email_verified")]
    pub is_email_verified: Option<bool>,
    #[serde(rename = "is_mobile_verified")]
    pub is_mobile_verified: Option<bool>,
    #[serde(rename = "primary_mobile")]
    pub primary_mobile: Option<i64>,
    #[serde(rename = "company_name")]
    pub company_name: Option<String>,
    #[serde(rename = "brand_name")]
    pub brand_name: Option<String>,
    #[serde(rename = "website")]
    pub website: Option<String>,
    #[serde(rename = "company_logo_link")]
    pub company_logo_link: Option<String>,
    #[serde(rename = "company_address")]
    pub company_address: Option<String>,
    #[serde(rename = "company_address_2")]
    pub company_address_2: Option<String>,
    #[serde(rename = "address_pincode")]
    pub address_pincode: Option<i32>,
    #[serde(rename = "city")]
    pub city: Option<String>,
    #[serde(rename = "state")]
    pub state: Option<String>,
    #[serde(rename = "country")]
    pub country: Option<String>,
    #[serde(rename = "last_billing_update")]
    pub last_billing_update: Option<i64>,
    #[serde(rename = "current_tier")]
    pub current_tier: Option<String>,
    #[serde(rename = "current_saas_plan")]
    pub current_saas_plan: Option<String>,
    #[serde(rename = "pickup_addresses")]
    pub pickup_addresses: Option<Vec<PickupAddress>>,
    #[serde(rename = "bank_details")]
    pub bank_details: Option<BankDetails>,
}
#[derive(Serialize, Deserialize)]
pub struct PickupAddress {
 #[serde(rename = "address_name")]
    address_name: Option<String>,
 #[serde(rename = "poc_name")]
    poc_name: Option<String>,
 #[serde(rename = "poc_mobile")]
    poc_mobile: Option<String>,
 #[serde(rename = "poc_email")]
    poc_email: Option<String>,
 #[serde(rename = "poc_alt_mobile")]
    poc_alt_mobile: Option<String>,
 #[serde(rename = "address_1")]
 address_1: Option<String>,
#[serde(rename = "address_2")]
address_2: Option<String>,
 #[serde(rename = "landmark")]
    landmark: Option<String>,
 #[serde(rename = "pincode")]
    pincode: Option<String>,
 #[serde(rename = "city")]
    city: Option<String>,
 #[serde(rename = "state")]
    state: Option<String>,
 #[serde(rename = "open_at")]
    open_at: Option<String>,
 #[serde(rename = "close_at")]
    close_at: Option<String>,
 #[serde(rename = "is_vendor_address")]
    is_vendor_address: Option<bool>,
 #[serde(rename = "vendor_name")]
    vendor_name: Option<String>,
 #[serde(rename = "vendor_gst")]
    vendor_gst: Option<String>,
 #[serde(rename = "is_rto_different")]
    is_rto_different: Option<bool>,
 #[serde(rename = "rto_address")]
    rto_address: Option<String>,
 #[serde(rename = "create_timestamp")]
    create_timestamp: Option<i64>,
}
#[derive(Serialize, Deserialize,Debug)]
pub struct BankDetails {
    #[serde(rename = "account_name")]
    pub(crate) account_name: Option<String>,
    #[serde(rename = "account_no")]
    pub(crate) account_no: Option<String>,
    #[serde(rename = "account_type")]
    pub(crate) account_type: Option<String>,
    #[serde(rename = "bank_name")]
    pub(crate) bank_name: Option<String>,
    #[serde(rename = "bank_branch")]
    pub(crate) bank_branch: Option<String>,
    #[serde(rename = "ifsc_code")]
    pub(crate) ifsc_code: Option<String>,
    #[serde(rename = "is_account_activated")]
    pub(crate) is_account_activated:Option<String>,
    #[serde(rename = "email")]
    pub(crate) email:Option<String>,
}

#[derive(Serialize)]
pub struct seller_profile_resp{
    pub(crate) status: bool,
    pub(crate) profile: get_seller_profile
}


#[derive(Serialize, Deserialize, Default)]
pub struct bank_data {
    pub(crate) account_name: String,
    pub(crate) account_no: String,
    pub(crate) account_type: String,
    pub(crate) bank_name: String,
    pub(crate) bank_branch: String,
    pub(crate) ifsc_code: String,
    pub(crate) is_account_activated: String,
    pub(crate) email : String
}

#[derive(Serialize, Deserialize)]
pub struct bank_detail_resp {
    pub status: bool
}

#[derive(Serialize)]
pub struct bank_profile_resp{
    pub(crate) status: bool,
    pub(crate) profile: BankDetails
}
