use actix_web::{Error, HttpMessage, HttpRequest, HttpResponse, web};
use actix_web::web::Redirect;
use image::EncodableLayout;
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope, TokenUrl};
use serde_json::Value;
use yup_oauth2::ServiceAccountAuthenticator;
use crate::config;
use crate::utils::google_basic_client::google_basic_client;

pub async fn google_login(mut payload: web::Payload) -> Redirect {

    // Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
    // token URL.
    let client = google_basic_client().await;

    // Generate a PKCE challenge.
    let pkce_challenge = config::configuration::pkce_challenge.read().await.as_ref().unwrap().clone();
    let pkce_verifier_str = config::configuration::pkce_verifier.read().await.as_ref().unwrap().secret().to_string();

    // Generate the full authorization URL.
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

    // This is the URL you should redirect the user to, in order to trigger the authorization
    // process.
    Redirect::to(auth_url.to_string()).permanent()
}