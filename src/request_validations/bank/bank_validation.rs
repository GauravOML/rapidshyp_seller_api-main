use crate::models::responses::common_resp;
use crate::models::sellers::seller_details::BankDetails;
use crate::utils::messages::INVALID_BANK_DETAILS;

pub async fn validate_bank_details(client_obj: &BankDetails) -> common_resp {

    if client_obj.account_no.is_none() || client_obj.account_no.as_ref().unwrap().is_empty() {
        return common_resp {
            status: true,
            msg: INVALID_BANK_DETAILS.to_string(),
        };
    }

    // if client_obj.address_2.is_none() || client_obj.address_2.as_ref().unwrap().is_empty(){
    //     return common_resp{
    //         status: false,
    //         msg: PRODUCT_NAME.to_string(),
    //     };
    // }
    //
    // if client_obj.qty.is_none() || client_obj.qty.as_ref().unwrap().is_empty(){
    //     return common_resp{
    //         status: false,
    //         msg: PRODUCT_QTY.to_string(),
    //     };
    // }
    //
    // if client_obj.order_type.is_none() || client_obj.order_type.as_ref().unwrap().is_empty(){
    //     return common_resp{
    //         status: false,
    //         msg: ORDER_TYPE.to_string(),
    //     };
    // }

    return common_resp {
        status: true,
        msg: "".to_string(),
    };
}
