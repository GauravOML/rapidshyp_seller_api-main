use serde::Deserialize;

#[derive(Deserialize)]
pub struct google_callback{
    pub code: String,
    pub state: String,
    pub scope: String,
    pub authuser: i32,
    pub prompt: String,
}

#[derive(Deserialize)]
pub struct google_callback_resp{
    pub given_name: String,
    pub family_name: String,
    pub name: String,
    pub sub: String,
    pub picture: String,
    pub email: String,
    pub email_verified: bool,
    pub locale: String,
}