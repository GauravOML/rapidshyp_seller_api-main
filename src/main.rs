#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use clap::Parser;
use futures::executor::block_on;
use serde::__private::Default;
use crate::applications::store::{CassandraStore, MongodbStore};
use crate::config::configuration::session_store;
use crate::queries::database_connection::DataStoreSession;
use crate::utils::google_basic_client::google_pkce;
use crate::web_server::create_connections;

mod applications;
mod utils;
mod web_server;
mod compute;
mod config;
mod models;
mod queries;
mod request_validations;

// parsing command line argument
#[derive(Parser, Debug)]
struct Args {
    /// environment
    #[clap(short, long, default_value = "local")]
    env: String,
}

#[tokio::main]
async fn main() {

    {log_writer!("Seller api launched...");}

    // getting the command line argument
    let args = Args::parse();

    // reading config file
    log_writer!("Reading --env from command line...");
    config::configuration::GLOBAL::read_etcd_config(&args.env);

    // get own machine ip
    config::configuration::GLOBAL::get_machine_ip().await;

    // connect to etcd for config loading
    block_on(applications::connectEtcd::etcd_client_connect()).expect("Failed to connect to etcd server...");

    // connect cassandra
    let mut store_obj = MongodbStore{
        ..Default::default()
    };

    store_obj.connect("carrier-integration".to_string()).await;

    // add database connection
    session_store.write().await.insert(DataStoreSession{
        mongo_store: store_obj,
    });
    
    // generate pkce
    google_pkce().await;

    // connect actix framework
    block_on(create_connections::connect()).expect("Failed to start tokio task...");
}
