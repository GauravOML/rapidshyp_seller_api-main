use actix_web::{HttpResponse, web, Error, error};
use chrono::Utc;
use futures::StreamExt;
use jsonwebtoken::{encode, EncodingKey, Header};
use sha2::{Sha512, Digest};
use crate::config;
use crate::config::configuration::session_store;
use crate::models::responses::common_resp;
use crate::models::sessions::{session_data, signup_success};
use crate::models::state::{state_data, state_payload, state_success};
use crate::request_validations::auth::generale_signup::validate_general_signup;
use crate::utils::messages::{INVALID_JSON, TOO_BIG, USER_EXISTS};

pub async fn create_state(client_obj: web::Json<state_payload>) -> Result<HttpResponse, Error> {

    let session_object = state_data{
        state_name: client_obj.state_name.to_owned().unwrap().to_string(),
        state_id: client_obj.state_id.to_owned().unwrap().to_string(),
    };
        session_store.read().await.as_ref().unwrap().createstate(&session_object).await;

    let conf = config::configuration::GLOBAL::get_config().as_ref().unwrap();

println!("http response ");
    return Ok(HttpResponse::Ok().json(state_success{
        status: true,
    }));
}