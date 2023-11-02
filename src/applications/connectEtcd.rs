#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use etcd_client::{Client, Error};

use crate::config::configuration::GLOBAL;
use crate::log_writer;

pub async fn etcd_client_connect() -> Result<(), Error>{

    log_writer!("connecting to etcd server for configuration...");

    let config = GLOBAL::get_etcd_config().as_ref().unwrap();

    let mut client = Client::connect([config.etcd.to_string()], None).await?;

    // get kv
    let mut resp = client.get(config.config_path.to_string(), None).await?;

    if let Some(kv) = resp.kvs().first() {
        let json_string = kv.value_str()?;
        GLOBAL::set_app_config(json_string);
    }

    Ok(())
}