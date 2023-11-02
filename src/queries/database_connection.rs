use cdrs::authenticators::NoneAuthenticator;
use cdrs::cluster::session::Session;
use cdrs::cluster::TcpConnectionPool;
use cdrs::load_balancing::RoundRobin;
use mongodb::Client;
use crate::applications::store::MongodbStore;

pub struct DataStoreSession{
    pub(crate) mongo_store: MongodbStore,
}

pub struct CassandraStoreSession{
    pub(crate) session: Session<RoundRobin<TcpConnectionPool<NoneAuthenticator>>>,
}