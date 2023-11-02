use actix_web::{Error, HttpMessage, HttpRequest, HttpResponse, web};
use rust_decimal::prelude::ToPrimitive;
use crate::config::configuration::session_store;
// use crate::models ::sellers::seller_details::BankDetails;
use crate::models::responses::common_resp;
use crate::models::sellers::bank_details::{bank_claims, bank_details_resp};
use crate::request_validations::bank::bank_validation::validate_bank_details;
use crate::utils::messages::{INVALID_BANK_DETAILS, INVALID_JSON};
use crate::models::sellers::seller_details::{bank_data, BankDetails, bank_detail_resp, seller_profile_resp, bank_profile_resp};
use crate::models::sellers::bank_details::bank_data_details;
use crate::models::sessions::user_claims;

pub async fn add_bank_details(client_obj: web::Json<BankDetails>) -> Result<HttpResponse, Error> {
    let status =validate_bank_details(&client_obj).await;

    println!("resul: {:?}", client_obj.account_no.to_owned());
    let bank_detail_obj = bank_data{
        account_no: client_obj.account_no.to_owned().unwrap().to_string(),
        account_type: client_obj.account_type.to_owned().unwrap().to_string(),
        bank_name: client_obj.bank_name.to_owned().unwrap().to_string(),
        bank_branch: client_obj.bank_branch.to_owned().unwrap().to_string(),
        ifsc_code: client_obj.ifsc_code.to_owned().unwrap().to_string(),
        is_account_activated: client_obj.is_account_activated.to_owned().unwrap().to_string(),
        account_name: client_obj.account_name.to_owned().unwrap().to_string(),
        email: client_obj.email.to_owned().unwrap().to_string(),
    };
    if !status.status{
        return Ok(HttpResponse::BadRequest().json(status));
    }

    let billing_data = session_store.read().await.as_ref().unwrap().set_bank_details(&bank_detail_obj).await;
    if !billing_data {
        return Ok(HttpResponse::BadRequest().json(common_resp {
            status: false,
            msg: INVALID_BANK_DETAILS.to_string(),
        }));
    }
    return Ok(HttpResponse::Ok().json(bank_detail_resp {
        status: true
    }));
}

pub async fn get_bank_details(req: HttpRequest) -> Result<HttpResponse, Error>{

    // get the email from the bearer token
    let email = req.extensions().get::<user_claims>().unwrap().email.to_string();
    println!("{}",email);

    // get the user profile from database
    let result = session_store.read().await.as_ref().unwrap().get_bank_details(email).await;
     println!("hey resp {:?}",result);

    if result.is_none(){
        return Ok(HttpResponse::BadRequest().json(common_resp{
            status: false,
            msg: "".to_string(),
        }));
    }

    // creating final result
    let resp = bank_profile_resp{
        status: true,
        profile: result.unwrap(),
    };

    Ok(HttpResponse::Ok().json(resp))
}




