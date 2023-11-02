use crate::models::responses::common_resp;
use crate::models::sessions::general_signup_payload;
use crate::utils::messages::{EMAIL_EMPTY, FIRST_NAME_EMPTY, LAST_NAME_EMPTY, PASSWORD_EMPTY};

pub async fn validate_general_signup(client_obj: &general_signup_payload) -> common_resp{

    // checking email
    if client_obj.email.is_none() || client_obj.email.as_ref().unwrap().is_empty(){
        return common_resp{
            status: false,
            msg: EMAIL_EMPTY.to_string(),
        };
    }

    // checking first name
    if client_obj.first_name.is_none() || client_obj.first_name.as_ref().unwrap().is_empty(){
        return common_resp{
            status: false,
            msg: FIRST_NAME_EMPTY.to_string(),
        };
    }

    // checking last name
    if client_obj.last_name.is_none() || client_obj.last_name.as_ref().unwrap().is_empty(){
        return common_resp{
            status: false,
            msg: LAST_NAME_EMPTY.to_string(),
        };
    }

    // checking password
    if client_obj.password.is_none() || client_obj.password.as_ref().unwrap().is_empty(){
        return common_resp{
            status: false,
            msg: PASSWORD_EMPTY.to_string(),
        };
    }

    // all good
    return common_resp{
        status: true,
        msg: "".to_string(),
    };
}