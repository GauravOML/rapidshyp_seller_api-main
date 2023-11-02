use mongodb::bson;
use mongodb::bson::doc;
use mongodb::options::{FindOneOptions, UpdateOptions};

use crate::models::sellers::bank_details::bank_data_details;
use crate::models::sellers::early_cod::early_cod_data;
use crate::models::sellers::seller_details::{bank_data, BankDetails, get_seller_profile};
use crate::queries::database_connection::DataStoreSession;

impl DataStoreSession{
    pub async fn set_bank_details(&self, data: &bank_data) -> bool {


        println!("{}", { data.email.to_string() } );
        // Create a filter and an update document
        let filter = doc! {
            "email":data.email.to_string() ,
        };



        let mut document = doc! {
             "$set": {
             "bank_details": {
            "account_name": data.account_name.to_string(),
            "account_no": data.account_no.to_string(),
            "account_type": data.account_type.to_string(),
            "ifsc_code": data.ifsc_code.to_string(),
            "bank_name":data.bank_name.to_string(),
            "bank_branch":data.bank_branch.to_string(),
            "is_account_activated":data.is_account_activated.to_string(),
                    }
                }
        };



        // Configure update options to enable upsert
        let options = UpdateOptions::builder().upsert(true).build();

        // Perform the upsert operation
        let collection = self.mongo_store.get_db().await.collection::<BankDetails>("sellers");
        // collection.update_one(filter, update, options).await.unwrap();
        collection.update_one(filter, document,options).await.is_ok()
    }

    pub async fn get_bank_details(&self, email: String) -> Option<BankDetails>{

        let filter = doc! {
        "email": email,
    };

        let options = FindOneOptions::builder()
            .projection(doc! { "bank_details": true, "_id": false })
            .build();

        let collection = self.mongo_store.get_db().await.collection::<BankDetails>("sellers");

        let res = collection.find_one(filter, options).await;

        if res.is_err(){
            return None;
        }

        res.unwrap()
    }
}

