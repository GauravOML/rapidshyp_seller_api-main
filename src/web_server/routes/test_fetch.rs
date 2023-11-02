#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use actix_web::{Error, HttpRequest, HttpResponse, web};
use crate::models::sessions::GetSession;

pub async fn test_fetch(_req: HttpRequest, _data: web::Query<GetSession>) -> Result<HttpResponse, Error> {

    // // println!("{:?}", req.extensions().get::<String>());
    //
    // let session_id = data.session_id.to_string();
    //
    // if !SimpleCacheObj.read().await.as_ref().unwrap().exists_value(&session_id).await{
    //
    //     Ok(HttpResponse::Ok().json(SessionResponse{
    //         session_id,
    //         status: false,
    //         body: String::from("No sessions id found..."),
    //     }))
    //
    // }else{
    //
    //     let body = SimpleCacheObj.read().await.as_ref().unwrap().get_value(&session_id).await;
    //
    //     let response_obj = SessionResponse{
    //         session_id,
    //         status: true,
    //         body: String::from_utf8(body.unwrap().to_vec()).unwrap(),
    //     };
    //
        Ok(HttpResponse::Ok().finish())
    // }
}