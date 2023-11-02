use std::time::{SystemTime, UNIX_EPOCH};
use cdrs::query::QueryExecutor;
use cdrs::query_values;
use chrono::Utc;
use mongodb::bson::{doc, Document, Bson};
use mongodb::options::{FindOneOptions, InsertOneOptions, UpdateOptions, WriteConcern};
use crate::applications::store::{MongodbStore};
use crate::log_writer;
use crate::models::sessions::session_data;
use crate::queries::database_connection::DataStoreSession;

impl DataStoreSession{
    pub async fn user_auth(&self, email: String, login_type: String){

        let epoch_time_millis = Utc::now().timestamp_millis();

        // Create a filter and an update document
        let filter = doc! {
            "email": email,
        };

        let update = doc! {
            "$set": {
                "last_login": epoch_time_millis,
                "last_login_type": login_type,
            }
        };

        // Configure update options to enable upsert
        let options = UpdateOptions::builder().upsert(true).build();

        // Perform the upsert operation
        let collection = self.mongo_store.get_db().await.collection::<Document>("sellers");
        collection.update_one(filter, update, options).await.unwrap();
    }

    pub async fn signup(&self, session: &session_data, login_type: String) -> bool{

        let epoch_time_millis = Utc::now().timestamp_millis();

        let collection = self.mongo_store.get_db().await.collection::<Document>("sellers");

        let mut document = doc! {
            "first_name": session.first_name.to_string(),
            "last_name": session.last_name.to_string(),
            "email": session.email.to_string(),
            "dp": session.dp.to_string(),
            "created_date": epoch_time_millis,
            "last_login": epoch_time_millis,
            "last_login_type": login_type.to_string(),
            "is_email_verified": session.is_email_verified,
            "is_mobile_verified": false,
            "primary_mobile": Bson::Null,
            "company_name": Bson::Null,
            "brand_name": Bson::Null,
            "website": Bson::Null,
            "company_logo_link": Bson::Null,
            "company_address": Bson::Null,
            "company_address_2": Bson::Null,
            "address_pincode": Bson::Null,
            "city": Bson::Null,
            "state": Bson::Null,
            "country": Bson::Null,
            "last_billing_update": Bson::Null,
            "current_tier": Bson::Null,
            "current_saas_plan": Bson::Null,
        };

        if login_type == "general".to_string(){
            document.insert("password", session.password.to_string());
        }

        let option = InsertOneOptions::builder().build();
        collection.insert_one(document, option).await.is_ok()
    }

    pub async fn check_user(&self, session: &session_data) -> bool{

        // Create a filter and an update document
        let mut filter = doc! {
            "email": session.email.to_string(),
        };

        let options = FindOneOptions::builder().projection(doc!{
            "_id": true,
        }).build();

        let collection = self.mongo_store.get_db().await.collection::<Document>("sellers");
        let res = collection.find_one(filter, options).await;

        return match res {
            Ok(result) => {
                match result {
                    None => {
                        false
                    }
                    Some(_) => {
                        true
                    }
                }
            }
            Err(err) => {
                log_writer!("{:?}", err);
                false
            }
        }
    }
}
