use oauth2::basic::{BasicClient, BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse, BasicTokenResponse, BasicTokenType};
use oauth2::{AuthUrl, Client, ClientId, ClientSecret, PkceCodeChallenge, RedirectUrl, StandardRevocableToken, TokenUrl};
use yup_oauth2::read_service_account_key;
use crate::config;

pub async fn google_basic_client() -> Client<BasicErrorResponse, BasicTokenResponse, BasicTokenType, BasicTokenIntrospectionResponse, StandardRevocableToken, BasicRevocationErrorResponse> {

    let conf = config::configuration::GLOBAL::get_config().as_ref().unwrap();

    BasicClient::new(
        ClientId::new(conf.google_oauth2.client_id.to_string()),
        Some(ClientSecret::new(conf.google_oauth2.client_secret.to_string())),
        AuthUrl::new(conf.google_oauth2.auth_uri.to_string()).unwrap(),
        Some(TokenUrl::new(conf.google_oauth2.token_uri.to_string()).unwrap())
    )
        // Set the URL the user will be redirected to after the authorization process.
        .set_redirect_uri(RedirectUrl::new(conf.google_oauth2.redirect_uri.to_string()).unwrap())
}

pub async fn google_pkce(){

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    config::configuration::pkce_challenge.write().await.insert(pkce_challenge);
    config::configuration::pkce_verifier.write().await.insert(pkce_verifier);
}

pub async fn get_sa_key(){

    let sa_key = read_service_account_key("client_secret_329270729890-crr4jib425rtd5lt5ibgrhnsijmfm16u.apps.googleusercontent.com.json")
        .await
        .expect("failed to read service account key file");

    config::configuration::sa_key.write().await.insert(sa_key);
}