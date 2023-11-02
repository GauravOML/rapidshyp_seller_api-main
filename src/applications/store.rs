#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::sync;
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs::authenticators::NoneAuthenticator;
use lazy_static::lazy_static;
use crate::{config, log_writer};
use crate::config::configuration::GLOBAL;
use sync::Arc;
use tokio::sync::{Mutex, RwLock};
use cdrs::cluster::session::{new as new_session, Session};
use cdrs::load_balancing::RoundRobin;
use serde::__private::Default;
use juniper::async_trait;
use mongodb::{Client, Database};
use mongodb::options::{ClientOptions, ResolverConfig};

// #[async_trait]
// pub trait Store{
//     async fn connect(&mut self, app_name: String) -> Session<RoundRobin<TcpConnectionPool<NoneAuthenticator>>>;
// }
//
// pub struct StoreObjects{
//     pub seller_store: Box<dyn Store>
// }
//
// impl StoreObjects {
//     fn new(s: Box<dyn Store>) -> Self {
//         StoreObjects { seller_store: s }
//     }
// }

#[derive(Default)]
pub struct CassandraStore{
    pub app_name: String,
}

impl CassandraStore {

    async fn connect(&mut self, app_name: String) -> Session<RoundRobin<TcpConnectionPool<NoneAuthenticator>>>{

        log_writer!("Connecting to cassandra database...");

        let config = config::configuration::GLOBAL::get_config().as_ref().unwrap();

        let node = NodeTcpConfigBuilder::new(config.cassandra.host.as_str(), NoneAuthenticator {}).build();
        let cluster_config = ClusterTcpConfig(vec![node]);
        self.app_name = app_name;

        log_writer!("Cassandra connected successfully...");

        return new_session(&cluster_config, RoundRobin::new()).expect("sessions should be created");
    }
}

#[derive(Default)]
pub struct MongodbStore{
    pub app_name: String,
    pub client: Option<Client>,
}

impl MongodbStore{
    pub(crate) async fn connect(&mut self, app_name: String) {

        // A Client is needed to connect to MongoDB:
        // An extra line of code to work around a DNS issue on Windows:
        let conf = config::configuration::GLOBAL::get_config().as_ref().unwrap();

        let mut options = ClientOptions::parse_with_resolver_config(conf.mongodb.conn.to_string(), ResolverConfig::cloudflare()).await.unwrap();
        options.max_pool_size = Option::from(500);

        let client = Client::with_options(options).unwrap();
        self.app_name = app_name;
        self.client = Option::from(client);

        log_writer!("Mongodb connected successfully...");
    }

    pub(crate) async fn get_db(&self) -> Database {
        let conf = config::configuration::GLOBAL::get_config().as_ref().unwrap();
        return self.client.as_ref().unwrap().database(conf.mongodb.database.as_str());
    }
}