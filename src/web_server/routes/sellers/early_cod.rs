use actix_web::{HttpResponse, web, Error};
use rust_decimal::prelude::ToPrimitive;
use crate::config::configuration::session_store;
// use crate::models ::sellers::seller_details::BankDetails;
use crate::models::responses::common_resp;
use crate::request_validations::bank::bank_validation::validate_bank_details;
use crate::utils::messages::{INVALID_E_COD_DETAILS, INVALID_JSON};
use crate::models::sellers::seller_details::{bank_data, BankDetails,bank_detail_resp};
use crate::models::sellers::early_cod::{early_cod_resp,early_cod_data,EarlyCod};

pub async fn add_early_cod_details(client_obj: web::Json<EarlyCod>) -> Result<HttpResponse, Error> {
    // let status =validate_bank_details(&client_obj).await;

    // println!("result: {:?}", client_obj.early_cod.to_owned());
    let early_cod_obj = early_cod_data{
        is_activated: client_obj.is_activated.to_owned().unwrap().to_string(),
        terms_timestamp: client_obj.terms_timestamp.to_owned().unwrap().to_string(),
        user: client_obj.terms_timestamp.to_owned().unwrap().to_string(),
        remarks: client_obj.terms_timestamp.to_owned().unwrap().to_string(),
        plan: client_obj.terms_timestamp.to_owned().unwrap().to_string(),
        email:client_obj.email.to_owned().unwrap().to_string(),
    };
    // if !status.status{
    //     return Ok(HttpResponse::BadRequest().json(status));
    // }

    let ecod_data = session_store.read().await.as_ref().unwrap().set_early_cod_details(&early_cod_obj).await;
    if !ecod_data {
        return Ok(HttpResponse::BadRequest().json(common_resp {
            status: false,
            msg: INVALID_E_COD_DETAILS.to_string(),
        }));
    }
    return Ok(HttpResponse::Ok().json(early_cod_resp {
        status: true
    }));
}