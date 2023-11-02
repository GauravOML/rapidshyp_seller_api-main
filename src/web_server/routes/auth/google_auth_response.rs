use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_web::web::Redirect;
use chrono::Utc;
use image::EncodableLayout;
use jsonwebtoken::{encode, EncodingKey, Header};
use oauth2::basic::{BasicClient, BasicTokenResponse};
use oauth2::{AuthorizationCode, AuthUrl, ClientId, ClientSecret, PkceCodeChallenge, PkceCodeVerifier, TokenResponse, TokenUrl};
use oauth2::reqwest::async_http_client;
use serde_json::{Deserializer, Value};
use crate::{config, log_writer};
use crate::config::configuration::session_store;
use crate::models::google_callback::{google_callback, google_callback_resp};
use crate::models::sessions::{session_data, user_claims};
use crate::utils::google_basic_client::google_basic_client;
use crate::utils::messages::INVALID_LOGIN;

pub async fn google_auth_response(req: HttpRequest, gcp: web::Query<google_callback>) -> Redirect {

    // getting config
    let conf = config::configuration::GLOBAL::get_config().as_ref().unwrap();

    // getting redirection from url
    let final_uri = conf.google_oauth2.client_ui_uri.to_string();

    // create a basic client for google oauth2
    let client = google_basic_client().await;

    // getting pkce verifier
    let pkce_verifier_str = config::configuration::pkce_verifier.read().await.as_ref().unwrap().secret().to_string();

    let pkce_verifier = PkceCodeVerifier::new(pkce_verifier_str);

    // getting user token
    match client
        .exchange_code(AuthorizationCode::new(gcp.code.to_string()))
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await{
        Ok(token_result) => {
            // Use the access token to fetch the user's profile info
            let access_token = token_result.access_token().secret();
            let user_info_url = conf.google_oauth2.user_info.to_string();
            let user_info_response = reqwest::Client::new()
                .get(user_info_url)
                .header("Authorization", format!("Bearer {}", access_token))
                .send()
                .await
                .unwrap();

            // deserializing json bytes
            let user_info: google_callback_resp = serde_json::from_slice(user_info_response.bytes().await.unwrap().as_bytes()).unwrap();

            // create sessions object
            let session_object = session_data{
                first_name: user_info.given_name.to_string(),
                last_name: user_info.family_name.to_string(),
                email: user_info.email.to_string(),
                dp: user_info.picture.to_string(),
                is_email_verified: true,
                ..Default::default()
            };

            // check if user already exists
            let user_exists = session_store.read().await.as_ref().unwrap().check_user(&session_object).await;

            if user_exists{
                session_store.read().await.as_ref().unwrap().user_auth(session_object.email.to_string(), "google".to_string()).await;
            }else{
                session_store.read().await.as_ref().unwrap().signup(&session_object, "google".to_string()).await;
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

            // create jwt token
            let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(conf.jwt_secret_key.as_ref())).unwrap();

            // send mail

            // redirect with jwt token
            return Redirect::to(final_uri.to_owned()+"?token="+token.as_str()).permanent();
        }
        Err(err) => {
            log_writer!("{:?}", err);
            Redirect::to(final_uri.to_owned()+"?error="+INVALID_LOGIN).permanent()
        }
    }
}