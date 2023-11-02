use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct common_resp {
    pub status: bool,
    pub msg: String,
}