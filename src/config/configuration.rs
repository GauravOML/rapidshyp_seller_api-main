#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

// contains configuration structures

// importing rust crates
use std::fs::File;
use std::io;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, ToSocketAddrs};
use std::process::Command;
use tokio::sync::{Mutex, RwLock};
use lazy_static::lazy_static;
use serde::Deserialize;
use std::sync::{Arc};
use dashmap::DashMap;
use oauth2::basic::{BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse, BasicTokenResponse, BasicTokenType};
use oauth2::{Client, StandardRevocableToken};
use yup_oauth2::ServiceAccountKey;
use crate::applications;
use crate::queries::database_connection::DataStoreSession;

// Etcd struct
#[derive(Debug, Deserialize)]
pub struct EtcdConfig{
    pub(crate) etcd: String,
    pub(crate) config_path: String,
}

#[derive(Debug, Deserialize)]
pub struct Http{
    pub(crate) host: String,
    pub(crate) port: i32,

}

#[derive(Debug, Deserialize)]
pub struct Cassandra{
    pub (crate) host: String,
    pub (crate) keyspace: String,
}

#[derive(Debug, Deserialize)]
pub struct CacheServer{
    pub (crate) host: String,
    pub (crate) port: i32,
}

#[derive(Debug, Deserialize)]
pub struct CarrierAPI{
    pub (crate) host: String,
    pub (crate) port: i32,
}

#[derive(Debug, Deserialize)]
pub struct Google_oauth2{
    pub client_id: String,
    pub auth_uri: String,
    pub token_uri: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub user_info: String,
    pub client_ui_uri: String,
}

#[derive(Debug, Deserialize)]
pub struct Mongodb{
    pub (crate) conn: String,
    pub (crate) database: String,
}

#[derive(Debug, Deserialize)]
pub struct Config{
    pub (crate) http: Http,
    pub (crate) cassandra: Cassandra,
    pub (crate) cache_server: CacheServer,
    pub (crate) carrier_api: CarrierAPI,
    pub (crate) google_oauth2: Google_oauth2,
    pub (crate) mongodb: Mongodb,
    pub (crate) jwt_secret_key: String,
}

// declaring static mutable variable for containing the config object
pub static mut CONF: Option<Config> = None;
pub static mut ETCD_CONF: Option<EtcdConfig> = None;

lazy_static! {
    pub static ref SYS_IP: RwLock<Option<String>> = RwLock::new(None);
    pub static ref pkce_challenge: RwLock<Option<oauth2::PkceCodeChallenge>> = RwLock::new(Option::from(None));
    pub static ref pkce_verifier: RwLock<Option<oauth2::PkceCodeVerifier>> = RwLock::new(Option::from(None));
    pub static ref sa_key: RwLock<Option<ServiceAccountKey>> = RwLock::new(Option::from(None));
    pub static ref session_store: RwLock<Option<DataStoreSession>> = RwLock::new(Option::from(None));
}

// creating a global config struct
#[derive(Clone, Debug)]
pub struct GLOBAL{}

// adding drop traits
impl Drop for GLOBAL{
    fn drop(&mut self){
        println!("Dropping config object...");
    }
}

// implementing the global struct adding read_config and get_config methods
impl GLOBAL{
    // read methods is reading the json file from disk and loading the json into structure
    pub fn read_etcd_config(env: &str){
        unsafe {
            // checking the command line argument
            let file_name = env.to_string()+".json";

            // reading file
            let file = File::open(file_name).expect("fail");

            // converting the json string to struct
            ETCD_CONF = serde_json::from_reader(file).expect("JSON was not well-formatted");
        }
    }

    // returning the etcd config file
    pub fn get_etcd_config() -> &'static Option<EtcdConfig>{
        unsafe {
            return &ETCD_CONF;
        }
    }

    // setting the config
    pub fn set_app_config(json_string: &str){
        unsafe{
            CONF = serde_json::from_str(json_string).unwrap();
        }
    }

    // returning the config file
    pub fn get_config() -> &'static Option<Config>{
        unsafe {
            return &CONF;
        }
    }

    // get machine ip
    pub async fn get_machine_ip() -> String {
        let _ = SYS_IP.write().await.insert(GLOBAL::get_host_ip().unwrap().to_string());
        println!("{:?}", SYS_IP.read().await);
        return GLOBAL::get_host_ip().unwrap().to_string();
    }

    // get machine ip
    pub(crate) fn get_host_ip() -> Option<IpAddr> {
        let output = Command::new("hostname")//launch a command on cmd or bash to get the hosts name i.e the machine name e.g rustpc
            .output()
            .expect("failed to execute `hostname`");//generate error if the command fails to execute
        let std_out = String::from_utf8(output.stdout).unwrap();//get the returned output from the outcome of the above command as the host name string
        let std_res =std_out.trim();//trim the output string to get rid of any semicolons etc to just leave the host name
        let name_to_ip_res :Vec<IpAddr> = GLOBAL::name_to_ip(std_res).unwrap();//use another predefined function to get the list of all the socket addresses in the running machine or vm
        let host_ip_option = name_to_ip_res.last();//it was found that the host ip adapter socket always shows at the end of the list or is the sole one in the list
        //either way extracting the last value of the list gives us the desired ip address
        let host_ip = host_ip_option.unwrap().to_string();//extract the host ip address from the option wrapping
        let ips: Vec<&str> = host_ip.trim().split(" ").collect::<Vec<&str>>();//generate a vector to test for the ip address type
        let first = ips.first();//generate an option type to work with the match statement
        match first{
            Some(first) =>  {
                if !first.is_empty(){//confirm if no null values are present in the option
                    if let Ok(addr) = first.parse::<Ipv4Addr>() {//if ip type ip v4 return as ip v4 option
                        return Some(IpAddr::V4(addr))
                    }
                    else if let Ok(addr) = first.parse::<Ipv6Addr>() {//if ip type ip v6 return as ip v6 option
                        return Some(IpAddr::V6(addr))
                    }
                    else{
                        None//if the ip type not matched to above values return null value
                    }
                }else{
                    None//if option empty return null value
                }
            }
            None => None//if null value to be returned that return option with null value
        }
    }

    /*
    * Custom function to get the host name as a string and generate a vector list of all the socket addresses present on it
    */
    fn name_to_ip(host: &str) -> io::Result<Vec<IpAddr>> {
        (host, 0).to_socket_addrs().map(|iter| iter.map(|socket_address| socket_address.ip()).collect())//run a map iterator to generate
        //a vector of the list of all the socket address on the given host name
    }
}
