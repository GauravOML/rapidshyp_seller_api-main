use actix_web::{Error, HttpResponse, web};
use chrono::Utc;
use crate::utils::messages::{INVALID_USERNAME_PASSWORD, TOO_BIG};
use crate::models::responses::common_resp;
use futures::StreamExt;
use jsonwebtoken::{encode, EncodingKey, Header};
use sha2::{Sha512, Digest};
use crate::config;
use crate::config::configuration::session_store;
use crate::models::sessions::{general_signin_payload, signup_success};
use crate::request_validations::auth::validate_general_signin::validate_general_signin;

pub async fn general_signin(client_obj: web::Json<general_signin_payload>) -> Result<HttpResponse, Error>{

    // validate the request
    let status = validate_general_signin(&client_obj).await;
    if !status.status{
        return Ok(HttpResponse::BadRequest().json(status));
    }

    // convert the password to sha512
    let mut hasher = Sha512::new();
    hasher.update(client_obj.password.as_ref().unwrap().to_string());
    let hash_password_wrapper = hasher.finalize();

    // Convert the result to a hexadecimal string
    let hash_password = Option::from(format!("{:x}", hash_password_wrapper)).unwrap();

    // check if user already exists
    let user_data = session_store.read().await.as_ref().unwrap().get_user_data(client_obj.email.as_ref().unwrap().to_string(), hash_password).await;

    // checking user data if null
    if user_data.is_none(){
        return Ok(HttpResponse::BadRequest().json(common_resp{
            status: false,
            msg: INVALID_USERNAME_PASSWORD.to_string(),
        }));
    }

    // update login time
    session_store.read().await.as_ref().unwrap().user_auth(client_obj.email.as_ref().unwrap().to_string(), "general".to_string()).await;

    let mut claims = user_data.unwrap();
    claims.session_time = Option::from(Utc::now().timestamp_millis());
    claims.exp = Option::from((Utc::now().timestamp() + 3600) as usize);

    // generate token
    // getting config
    let conf = config::configuration::GLOBAL::get_config().as_ref().unwrap();

    // create jwt token
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(conf.jwt_secret_key.as_ref())).unwrap();

    // return success with token
    return Ok(HttpResponse::Ok().json(signup_success{
        status: true,
        token,
    }));
}