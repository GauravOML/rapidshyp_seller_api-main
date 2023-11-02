#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct state_payload{
    #[serde(rename = "state_id")]
    pub state_id: Option<String>,
    #[serde(rename = "state_name")]
    pub state_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct state_success{
    pub status: bool,

}


pub struct state_data{
    pub(crate) state_name: String,
    pub(crate) state_id: String,
}