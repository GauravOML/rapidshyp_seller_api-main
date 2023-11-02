use actix_web::{Error, HttpMessage, HttpRequest, HttpResponse, web};
use clap::builder::Str;
use crate::config::configuration::session_store;
use crate::models::responses::common_resp;
use crate::models::sellers::seller_details::seller_profile_resp;
use crate::models::sessions::user_claims;
use crate::utils::messages::INVALID_USER;

pub async fn get_profile(req: HttpRequest) -> Result<HttpResponse, Error>{

    // get the email from the bearer token
    let email = req.extensions().get::<user_claims>().unwrap().email.to_string();

    // get the user profile from database
    let result = session_store.read().await.as_ref().unwrap().get_seller_profile(email).await;

    if result.is_none(){
        return Ok(HttpResponse::BadRequest().json(common_resp{
            status: false,
            msg: INVALID_USER.to_string(),
        }));
    }

    // creating final result
    let resp = seller_profile_resp{
        status: true,
        profile: result.unwrap(),
    };

    Ok(HttpResponse::Ok().json(resp))
}