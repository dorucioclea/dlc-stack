use config::Config as Conf;
use log::info;
use sled::{Config, Db};
use redis::cluster::{ClusterClient, ClusterConnection};
use std::{
    env
};
use redis::Commands;
use crate::oracle::OracleError;

extern crate futures;
extern crate tokio;

#[derive(Clone)]
pub struct EventHandler {

    pub use_redis: bool,

    pub redis: Option<RedisConn>,

    pub sled_db: Option<Db>
}

type RedisKey = Vec<(String, Vec<String>)>;

impl  EventHandler {
    pub fn new(_config: Conf) -> Self {
        let sled;
        let redis_conn;
        let use_redis: bool = env::var("REDIS_ENABLED")
            .unwrap_or("false".to_string())
            .parse().unwrap();
        info!("Redis enabled: {}", use_redis);
        if use_redis {
            sled = None; 
            // TODO: support auth + tls
            // let nodes = vec!["redis://127.0.0.1:6379/", "redis://127.0.0.1:6380/", "redis://127.0.0.1:6381/", "redis://127.0.0.1:6382/", "redis://127.0.0.1:6383/"];
            let cluster_client = ClusterClient::builder(env::var("REDIS_URLS")
                .unwrap_or("redis://127.0.0.1:6379/".to_string())
                .split(",").collect::<Vec<&str>>()).build();
            redis_conn = Some(RedisConn::new(cluster_client.unwrap()));
        } else {
            let path = "events";
            info!("creating sled event database at {}", path);
            sled = Some(Config::new()
                .path(path)
                .cache_capacity(128 * 1024 * 1024)
                .open()
                .unwrap());
            redis_conn = None;
        }

        Self { use_redis: use_redis, redis: redis_conn, sled_db: sled }
    }

    pub fn is_empty(&self) -> bool {
        if self.use_redis {
            info!("Skip checking all events in redis...");
            return false;
        } else {
            return self.sled_db.as_ref().unwrap().is_empty()
        }
    }
}


#[derive(Clone)]
pub struct RedisConn {
    pub client: ClusterClient
}

impl RedisConn {
    pub fn new(client: ClusterClient) -> Self {
        Self { client }
    }

    pub fn insert(&self, uuid : String, new_event: Vec<u8>) -> Result<Option<Vec<u8>>, OracleError> {
        info!("New event will be stored by redis. [uuid: {}]", uuid.clone());
        let mut connection : ClusterConnection = self.client.get_connection().unwrap();
        let () = connection.set(uuid.clone(), new_event.clone()).unwrap();
        return Ok(Some(new_event.clone()));
    }

    pub fn get_all(&self) -> Result<Option<Vec<(String, Vec<u8>)>>, OracleError> {
        let mut connection: ClusterConnection = self.client.get_connection().unwrap();
        let keys: Vec<RedisKey> = connection.keys("*").unwrap();
        let mut result: Vec<(String, Vec<u8>)> = vec![];
        for key_type in keys {
            for key_bulk_type in key_type {
                for k in key_bulk_type.1 {
                    let event: Vec<u8> = connection.get(k.clone())?;
                    result.push((k.clone(), event));
                }
            }
        }
        return Ok(Some(result));
    }

    pub fn get(&self, key: String) -> Result<Option<Vec<u8>>, OracleError> {
        let mut connection: ClusterConnection = self.client.get_connection().unwrap();
        let result : Vec<u8> = connection.get(key)?;
        return Ok(Some(result));
    }
}
