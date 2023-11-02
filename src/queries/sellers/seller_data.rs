use mongodb::bson;
use mongodb::bson::doc;
use mongodb::options::{FindOneOptions, UpdateOptions};

use crate::models::sellers::bank_details::bank_data_details;
use crate::models::sellers::early_cod::early_cod_data;
use crate::models::sellers::seller_details::{bank_data, BankDetails, get_seller_profile};
use crate::queries::database_connection::DataStoreSession;

impl DataStoreSession{
    pub async fn get_seller_profile(&self, email: String) -> Option<get_seller_profile>{

        let filter = doc!{
            "email": email,
        };

        let options = FindOneOptions::builder().projection(doc!{
            "_id": false,
            "first_name": true,
            "last_name": true,
            "email": true,
            "dp": true,
            "created_date": true,
            "last_login": true,
            "is_email_verified": true,
            "is_mobile_verified": true,
            "primary_mobile": true,
            "company_name": true,
            "brand_name": true,
            "website": true,
            "company_logo_link": true,
            "company_address": true,
            "company_address_2": true,
            "address_pincode": true,
            "city": true,
            "state": true,
            "country": true,
            "last_billing_update": true,
            "current_tier": true,
            "current_saas_plan": true,
        }).build();

        let collection = self.mongo_store.get_db().await.collection::<get_seller_profile>("sellers");

        let res = collection.find_one(filter, options).await;

        if res.is_err(){
            return None;
        }

        res.unwrap()
    }



    //set bank details



}