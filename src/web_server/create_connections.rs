#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use actix_web::{web, App, HttpServer, middleware, HttpMessage, Error, Handler, HttpRequest};
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use std::{future::{ready, Ready, Future}, pin::Pin};
use std::net::SocketAddr;
use std::str::FromStr;
use actix_cors::Cors;
use crate::config::configuration;
use crate::web_server::routes::test_fetch;
use std::time::Duration;
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::{config, log_writer};
use crate::config::configuration::Config;
use crate::models::responses::common_resp;
use crate::models::sessions::{session_expired, user_claims};
use crate::utils::messages::UNAUTHORIZED_REQUEST;
use crate::web_server::routes::auth::general_signin::general_signin;
use crate::web_server::routes::auth::general_signup::general_signup;
use crate::web_server::routes::auth::google_auth_response::google_auth_response;
use crate::web_server::routes::auth::google_login::google_login;
use crate::web_server::routes::sellers::get_profile::{get_profile};
use crate::web_server::routes::sellers::state::create_state;
use crate::web_server::routes::sellers::bank_details::{add_bank_details, get_bank_details};
use crate::web_server::routes::sellers::early_cod::add_early_cod_details;

pub struct SessionMiddleware;

// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for SessionMiddleware
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = SessionMiddlewareMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SessionMiddlewareMiddleware { service }))
    }
}

pub struct SessionMiddlewareMiddleware<S> {
    /// The next service to call
    service: S,
}

// This future doesn't have the requirement of being `Send`.
// See: futures_util::future::LocalBoxFuture
type LocalBoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

// `S`: type of the wrapped service
// `B`: type of the body - try to be generic over the body where possible
impl<S, B> Service<ServiceRequest> for SessionMiddlewareMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<Result<Self::Response, Self::Error>>;

    // This service is ready when its next service is ready
    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {

        let mut response = session_expired{
            status: false,
            session_expired: true,
            msg: UNAUTHORIZED_REQUEST.to_string(),
        };

        // get the authorization header
        let headers = req.headers();
        if headers.contains_key("authorization"){
            let authorization = headers.get("authorization").unwrap();
            let authorization_header = String::from_utf8(authorization.as_ref().to_vec()).unwrap();
            let authorization_header_arr: Vec<&str> = authorization_header.split(' ').collect();

            if authorization_header_arr.len() == 2{

                let bearer_token = authorization_header_arr[1].to_string();

                // getting config
                let conf = config::configuration::GLOBAL::get_config().as_ref().unwrap();

                let decoding_key = DecodingKey::from_secret(conf.jwt_secret_key.as_ref());
                let validation = Validation::default();

                match decode::<user_claims>(&bearer_token, &decoding_key, &validation) {
                    Ok(token_data) => {
                        req.extensions_mut().insert(token_data.claims);
                        response.status = true;
                    },
                    Err(e) => {
                        log_writer!("{:?}", e);
                    },
                }
            }
        }

        // A more complex middleware, could return an error or an early response here.

        let fut = self.service.call(req);

        Box::pin(async move {
            if response.status{
                let res = fut.await?;
                Ok(res)
            }else{
                let result = serde_json::to_string(&response).unwrap();
                Err(actix_web::error::ErrorBadRequest(result))
            }
        })
    }
}

// method where web-socket server is hosted
pub async fn connect() -> std::io::Result<()> {

    // getting the configuration
    // taking config clone as a reference
    let conf = configuration::GLOBAL::get_config().as_ref();

    // unwrapping the config from Option and then converting to string
    let host = conf.unwrap().http.host.to_string();
    let port = conf.unwrap().http.port;

    // listening to the ip and port specified in the config
    println!("Http server hosted at port: {:?}", port);

    let adder = SocketAddr::from_str(&*String::from(host + ":" + &*port.to_string())).unwrap();

    HttpServer::new(move
        ||
        App::new()
            .wrap(middleware::DefaultHeaders::new().add(("Content-Type", "application/json")))
            .service(
        web::resource("/session/sellers/get_profile")
            .route(web::get().to(get_profile))
            .wrap(SessionMiddleware)
            )
            .service(
                web::resource("/session/state")
                    .route(web::post().to(create_state))
                    .wrap(SessionMiddleware)
            )

            .service(
                web::resource("/session/sellers/bank_details")
                    .route(web::patch().to(add_bank_details))
                    .wrap(SessionMiddleware)
            )
            .service(
                web::resource("/session/sellers/early_cod")
                    .route(web::patch().to(add_early_cod_details))
                    .wrap(SessionMiddleware)
            )
            .service(
                web::resource("/session/sellers/get_bank_details")
                    .route(web::get().to(get_bank_details))
                    .wrap(SessionMiddleware)
            )

            .route("/fetch", web::get().to(test_fetch::test_fetch))
            .route("/auth/google_auth", web::get().to(google_login))
            .route("/auth/google_auth_response", web::get().to(google_auth_response))
            .route("/auth/general_signup", web::post().to(general_signup))
            .route("/auth/general_login", web::post().to(general_signin))
            // .route("/session/sellers/bank_details", web::patch().to(add_bank_details))
            // .route("/session/sellers/early_cod", web::patch().to(add_early_cod_details))
            // .app_data(Data::from(schema.clone()))
            // .service(graphql)
            // .service(graphql_playground)
            // the graphiql UI requires CORS to be enabled
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
    )
        .keep_alive(Duration::from_secs(150))
        .workers(64)
        .bind(adder)?
        .shutdown_timeout(60)
        .run()
        .await
}