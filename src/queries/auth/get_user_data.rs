use mongodb::bson::{doc, Document};
use mongodb::options::FindOneOptions;
use crate::models::sessions::{user_claims};
use crate::queries::database_connection::DataStoreSession;

impl DataStoreSession{
    pub async fn get_user_data(&self, email: String, password: String) -> Option<user_claims>{

        let filter = doc!{
            "email": email,
            "password": password,
        };

        let options = FindOneOptions::builder().projection(doc!{
            "_id": false,
            "first_name": true,
            "last_name": true,
            "email": true,
            "dp": true,
        }).build();

        let collection = self.mongo_store.get_db().await.collection::<user_claims>("sellers");

        let res = collection.find_one(filter, options).await;

        if res.is_err(){
            return None;
        }

        res.unwrap()
    }
}