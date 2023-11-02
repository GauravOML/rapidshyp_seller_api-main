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

impl DataStoreSession{

    pub async fn createstate(&self, session: &state_data) -> bool{

        let epoch_time_millis = Utc::now().timestamp_millis();

        let collection = self.mongo_store.get_db().await.collection::<Document>("state");

        let mut document = doc! {
            "state_id": session.state_id.to_string(),
            "state_name": session.state_name.to_string(),
        };


            // document.insert("password", session.password.to_string());


        let option = InsertOneOptions::builder().build();
        collection.insert_one(document, option).await.is_ok()
    }


}
