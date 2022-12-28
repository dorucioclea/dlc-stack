extern crate serde;

use std::{error, fmt};
use std::fmt::{Debug, Formatter};
use reqwest::{Client, Error, Response, Url};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfferRequest {
    pub uuid: String,
    pub accept_collateral: u64,
    pub offer_collateral: u64,
    pub total_outcomes: i32,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcceptMessage {
    pub accept_message: String,
}

#[derive(Debug)]
pub struct ApiResult {
    pub status: u16,
    pub response: Response,
}

#[derive(Debug)]
pub struct ApiError {
    pub message: String,
    pub status: u16
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ApiError: {} - {}", self.status, self.message)
    }
}

impl error::Error for ApiError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    pub id: i32,
    pub uuid: String,
    pub state: String,
    pub content: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewContract {
    pub uuid: String,
    pub state: String,
    pub content: String,
}

pub struct WalletBackendClient {
    client: Client,
    host: String
}

impl Default for WalletBackendClient {
    fn default() -> Self {
        Self::new("http://localhost:8085".to_string())
    }
}

impl Debug for WalletBackendClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.host)
    }
}

impl WalletBackendClient {

    pub fn new(host: String) -> Self {
        Self { client: Client::new(), host: host }
    }

    pub async fn post_offer_and_accept(&self, offer_request: OfferRequest) -> Result<ApiResult, Error> {
        let uri = format!("{}/offer", String::as_str(&self.host.clone()));
        let url = Url::parse(uri.as_str()).unwrap();
        let res = self.client
            .post(url)
            .json(&offer_request)
            .send()
            .await?;
        let result = ApiResult {
            status: res.status().as_u16(),
            response: res,
        };
        Ok(result)
    }

    pub async fn put_accept(&self, accept_request: AcceptMessage) -> Result<ApiResult, Error> {
        let uri = format!("{}/offer/accept", String::as_str(&self.host.clone()));
        let url = Url::parse(uri.as_str()).unwrap();
        let res = self.client
            .put(url)
            .json(&accept_request)
            .send()
            .await?;
        let result = ApiResult {
            status: res.status().as_u16(),
            response: res,
        };
        Ok(result)
    }
}

pub struct OracleBackendClient {
    client: Client,
    host: String
}

impl Default for OracleBackendClient {
    fn default() -> Self {
        Self::new("http://localhost:8080".to_string())
    }
}

impl Debug for OracleBackendClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.host)
    }
}

impl OracleBackendClient {
    pub fn new(host: String) -> Self {
        Self { client: Client::new(), host: host }
    }

    pub async fn create_event(&self, uuid: String) -> Result<ApiResult, Error> {
        let uri = format!("{}/v1/create_event/{}?maturation=2022-10-08T13:48:00Z", String::as_str(&self.host.clone()), uuid.as_str());
        let url = Url::parse(uri.as_str()).unwrap();
        let res = self.client
            .get(url)
            .send()
            .await?;
        let result = ApiResult {
            status: res.status().as_u16(),
            response: res,
        };
        Ok(result)
    }

    pub async fn get_attestation(&self, uuid: String, outcome: String) -> Result<ApiResult, Error> {
        let uri = format!("{}/v1/attest/{}?outcome={}", String::as_str(&self.host.clone()), uuid.as_str(), outcome.as_str());
        let url = Url::parse(uri.as_str()).unwrap();
        let res = self.client
            .get(url)
            .send()
            .await?;
        let result = ApiResult {
            status: res.status().as_u16(),
            response: res,
        };
        Ok(result)
    }
}

pub struct StorageApiClient {
    client: Client,
    host: String
}

impl Default for StorageApiClient {
    fn default() -> Self {
        Self::new("http://localhost:8100".to_string())
    }
}

impl Debug for StorageApiClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.host)
    }
}

impl StorageApiClient {
    pub fn new(host: String) -> Self {
        Self { client: Client::new(), host: host }
    }

    pub async fn get_contracts(&self) -> Result<Vec<Contract>, ApiError> {
        let uri = format!("{}/contracts", String::as_str(&self.host.clone()));
        let url = Url::parse(uri.as_str()).unwrap();
        let res = match self.client.get(url).send().await {
            Ok(result) => result,
            Err(e) => return Err(ApiError { message: e.to_string(), status: 0 }),
        };
        let status = res.status();
        if status.is_success() {
            let status_clone = status.clone();
            let contracts: Vec<Contract> = res.json().await.map_err(|e| ApiError { message: e.to_string(), status: status_clone.as_u16() })?;
            Ok(contracts)
        } else {
            let status_clone = status.clone();
            let msg: String = res.text().await.map_err(|e| ApiError { message: e.to_string(), status: status_clone.as_u16() })?;
            Err(ApiError { message: msg, status: status_clone.as_u16() })
        }
    }

    pub async fn get_contract(&self, uuid: String) -> Result<Contract, ApiError> {
        let uri = format!("{}/contracts/{}", String::as_str(&self.host.clone()), uuid.as_str());
        let url = Url::parse(uri.as_str()).unwrap();
        let res = match self.client.get(url).send().await {
            Ok(result) => result,
            Err(e) => return Err(ApiError { message: e.to_string(), status: 0 }),
        };
        let status = res.status();
        if status.is_success() {
            let status_clone = status.clone();
            let contract: Contract = res.json().await.map_err(|e| ApiError { message: e.to_string(), status: status_clone.as_u16() })?;
            Ok(contract)
        } else {
            let status_clone = status.clone();
            let msg: String = res.text().await.map_err(|e| ApiError { message: e.to_string(), status: status_clone.as_u16() })?;
            Err(ApiError { message: msg, status: status_clone.as_u16() })
        }
    }

    pub async fn create_contract(&self, contract: NewContract) -> Result<Contract, ApiError> {
        let uri = format!("{}/contracts", String::as_str(&self.host.clone()));
        let url = Url::parse(uri.as_str()).unwrap();
        let res = match self.client.post(url).json(&contract).send().await {
            Ok(result) => result,
            Err(e) => return Err(ApiError { message: e.to_string(), status: 0 }),
        };
        let status = res.status();
        if status.is_success() {
            let status_clone = status.clone();
            let contract: Contract = res.json().await.map_err(|e| ApiError { message: e.to_string(), status: status_clone.as_u16() })?;
            Ok(contract)
        } else {
            let status_clone = status.clone();
            let msg: String = res.text().await.map_err(|e| ApiError { message: e.to_string(), status: status_clone.as_u16() })?;
            Err(ApiError { message: msg, status: status_clone.as_u16() })
        }
    }

    pub async fn delete_contract(&self, uuid: String) -> Result<Contract, ApiError> {
        let uri = format!("{}/contracts/{}", String::as_str(&self.host.clone()), uuid.as_str());
        let url = Url::parse(uri.as_str()).unwrap();
        let res = match self.client.delete(url).send().await {
            Ok(result) => result,
            Err(e) => return Err(ApiError { message: e.to_string(), status: 0 }),
        };
        let status = res.status();
        if status.is_success() {
            let status_clone = status.clone();
            let contract: Contract = res.json().await.map_err(|e| ApiError { message: e.to_string(), status: status_clone.as_u16() })?;
            Ok(contract)
        } else {
            let status_clone = status.clone();
            let msg: String = res.text().await.map_err(|e| ApiError { message: e.to_string(), status: status_clone.as_u16() })?;
            Err(ApiError { message: msg, status: status_clone.as_u16() })
        }
    }
}