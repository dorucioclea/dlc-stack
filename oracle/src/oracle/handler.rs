extern crate base64;
use config::Config as Conf;
use log::info;
use sled::{Config, Db};
use redis::cluster::{ClusterClient, ClusterConnection};
use std::{
    env
};
use redis::Commands;
use dlc_clients::{NewEvent, StorageApiClient, UpdateEvent};
use crate::oracle::OracleError;

extern crate futures;
extern crate tokio;

#[derive(Clone)]
pub struct EventHandler {

    pub use_redis: bool,

    pub redis: Option<RedisConn>,

    pub sled_db: Option<Db>,

    pub storage_api: Option<StorageApiConn>
}

type RedisKey = Vec<(String, Vec<String>)>;

impl  EventHandler {
    pub fn new(_config: Conf) -> Self {
        let sled;
        let redis_conn;
        let storage_api_conn;
        let use_redis: bool = env::var("REDIS_ENABLED")
            .unwrap_or("false".to_string())
            .parse().unwrap();
        let use_storage_api: bool = env::var("STORAGE_API_ENABLED")
            .unwrap_or("false".to_string())
            .parse().unwrap();
        let storage_api_endpoint: String = env::var("STORAGE_API_ENDPOINT").unwrap_or("http://localhost:8100".to_string());
        info!("Storage api enabled: {}", use_storage_api);
        info!("Redis enabled: {}", use_redis);
        if use_storage_api {
            redis_conn = None;
            sled = None;
            let storage_api_client = StorageApiClient::new(storage_api_endpoint);
            storage_api_conn = Some(StorageApiConn::new(storage_api_client));
        } else if use_redis {
            sled = None; 
            // TODO: support auth + tls
            // let nodes = vec!["redis://127.0.0.1:6379/", "redis://127.0.0.1:6380/", "redis://127.0.0.1:6381/", "redis://127.0.0.1:6382/", "redis://127.0.0.1:6383/"];
            let cluster_client = ClusterClient::builder(env::var("REDIS_URLS")
                .unwrap_or("redis://127.0.0.1:6379/".to_string())
                .split(",").collect::<Vec<&str>>()).build();
            redis_conn = Some(RedisConn::new(cluster_client.unwrap()));
            storage_api_conn = None;
        } else {
            let path = "events";
            info!("creating sled event database at {}", path);
            sled = Some(Config::new()
                .path(path)
                .cache_capacity(128 * 1024 * 1024)
                .open()
                .unwrap());
            redis_conn = None;
            storage_api_conn = None;
        }

        Self { use_redis: use_redis, redis: redis_conn, sled_db: sled, storage_api: storage_api_conn }
    }

    pub fn is_empty(&self) -> bool {
        if self.storage_api.is_some() {
            return false;
        } else if self.use_redis {
            info!("Skip checking all events in redis...");
            return false;
        } else {
            return self.sled_db.as_ref().unwrap().is_empty()
        }
    }
}

#[derive(Clone)]
pub struct StorageApiConn {
    pub client: StorageApiClient
}

impl StorageApiConn {
    pub fn new(client: StorageApiClient) -> Self {
        Self { client }
    }

    pub async fn insert(&self, event_id : String, new_event: Vec<u8>) -> Result<Option<Vec<u8>>, OracleError> {
        let new_content = base64::encode(new_event.clone());
        let event = self.client.get_event(event_id.clone()).await?;
        if event.is_some() {
            let update_event = UpdateEvent{content: Some(new_content.clone())};
            let _res = self.client.update_event(event_id.clone(), update_event).await;
            Ok(Some(new_event.clone()))
        } else {
            let event = NewEvent{event_id: event_id.clone(),content: new_content.clone()};
            let _res = self.client.create_event(event).await;
            Ok(Some(new_event.clone()))
        }
    }

    pub async fn get(&self, event_id: String) -> Result<Option<Vec<u8>>, OracleError> {
        let event = self.client.get_event(event_id.clone()).await?;
        if event.is_some() {
            let res = base64::decode(event.unwrap().content).unwrap();
            Ok(Some(res))
        } else {
            Ok(None)
        }
    }

    pub async fn get_all(&self) -> Result<Option<Vec<(String, Vec<u8>)>>, OracleError> {
        let res_events = self.client.get_events().await.unwrap();
        let mut result: Vec<(String, Vec<u8>)> = vec![];
        for event in res_events {
            let content = base64::decode(event.content).unwrap();
            result.push((event.event_id, content));
        }
        return Ok(Some(result));
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
