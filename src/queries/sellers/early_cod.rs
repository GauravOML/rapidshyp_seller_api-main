use std::time::{SystemTime, UNIX_EPOCH};
use cdrs::query::QueryExecutor;
use cdrs::query_values;
use chrono::Utc;
use mongodb::bson::{doc, Document, Bson};
use mongodb::options::{FindOneOptions, InsertOneOptions, UpdateOptions, WriteConcern};
use crate::applications::store::{MongodbStore};
use crate::log_writer;
use crate::models::sessions::session_data;
use crate::models::state::state_data;
use crate::queries::database_connection::DataStoreSession;
use crate::models::sellers::early_cod::{early_cod_data,early_cod_resp,EarlyCod};

impl DataStoreSession{

    pub async fn set_early_cod_details(&self, data: &early_cod_data) -> bool {



        // Create a filter and an update document
        let filter = doc! {
            "email": data.email.to_string(),
        };
        let mut document = doc! {
             "$set": {
             "early_cod": {
            "is_activated": data.is_activated.to_string(),
            "terms_timestamp": data.terms_timestamp.to_string(),
            "user": data.user.to_string(),
            "remarks": data.remarks.to_string(),
            "plan": data.plan.to_string()
                    }
                }
        };



        // Configure update options to enable upsert
        let options = UpdateOptions::builder().upsert(true).build();
        // Perform the upsert operation
        let collection = self.mongo_store.get_db().await.collection::<EarlyCod>("sellers");
        // collection.update_one(filter, update, options).await.unwrap();
        collection.update_one(filter, document,options).await.is_ok()
    }


}
