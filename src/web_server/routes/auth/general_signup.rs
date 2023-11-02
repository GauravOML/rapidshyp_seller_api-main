use actix_web::{HttpResponse, web, Error, error};
use chrono::Utc;
use futures::StreamExt;
use jsonwebtoken::{encode, EncodingKey, Header};
use sha2::{Sha512, Digest};
use crate::config;
use crate::config::configuration::session_store;
use crate::models::responses::common_resp;
use crate::models::sessions::{general_signup_payload, session_data, signup_success, user_claims};
use crate::request_validations::auth::generale_signup::validate_general_signup;
use crate::utils::messages::{INVALID_JSON, TOO_BIG, USER_EXISTS};

pub async fn general_signup(client_obj: web::Json<general_signup_payload>) -> Result<HttpResponse, Error> {

    // validate the request
    let status = validate_general_signup(&client_obj).await;
    if !status.status{
        return Ok(HttpResponse::BadRequest().json(status));
    }

    // convert the password to sha512
    let mut hasher = Sha512::new();
    hasher.update(client_obj.password.as_ref().unwrap().to_string());
    let hash_password_wrapper = hasher.finalize();

    // Convert the result to a hexadecimal string
    let hash_password = Option::from(format!("{:x}", hash_password_wrapper)).unwrap();

    // create sessions object
    let session_object = session_data{
        first_name: client_obj.first_name.to_owned().unwrap().to_string(),
        last_name: client_obj.last_name.to_owned().unwrap().to_string(),
        email: client_obj.email.to_owned().unwrap().to_string(),
        password: hash_password,
        dp: "".to_string(),
        ..Default::default()
    };

    // check if user already exists
    let user_exists = session_store.read().await.as_ref().unwrap().check_user(&session_object).await;

    if user_exists{
        return Ok(HttpResponse::BadRequest().json(common_resp{
            status: false,
            msg: USER_EXISTS.to_string(),
        }));
    }else{
        session_store.read().await.as_ref().unwrap().signup(&session_object, "general".to_string()).await;
    }

    // creating user session claims
    let claims = user_claims{
        first_name: session_object.first_name,
        last_name: session_object.last_name,
        email: session_object.email,
        dp: session_object.dp,
        session_time: Option::from(Utc::now().timestamp_millis()),
        exp: Option::from((Utc::now().timestamp() + 3600) as usize)
    };

    // getting config
    let conf = config::configuration::GLOBAL::get_config().as_ref().unwrap();

    // create jwt token
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(conf.jwt_secret_key.as_ref())).unwrap();

    return Ok(HttpResponse::Ok().json(signup_success{
        status: true,
        token,
    }));
}