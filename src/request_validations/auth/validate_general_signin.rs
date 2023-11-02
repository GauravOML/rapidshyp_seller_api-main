use crate::models::responses::common_resp;
use crate::models::sessions::general_signin_payload;
use crate::utils::messages::{EMAIL_EMPTY, PASSWORD_EMPTY};

pub async fn validate_general_signin(client_obj: &general_signin_payload) -> common_resp{

    if client_obj.email.is_none() || client_obj.email.as_ref().unwrap().is_empty(){
        return common_resp{
            status: false,
            msg: EMAIL_EMPTY.to_string(),
        };
    }

    if client_obj.password.is_none() || client_obj.password.as_ref().unwrap().is_empty(){
        return common_resp{
            status: false,
            msg: PASSWORD_EMPTY.to_string(),
        };
    }

    return common_resp{
        status: true,
        msg: "".to_string(),
    };
}