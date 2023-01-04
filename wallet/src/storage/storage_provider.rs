extern crate tokio;
extern crate base64;
use log::info;
use std::env;
use dlc_manager::{ContractId, Storage};
use dlc_manager::contract::{Contract, PreClosedContract};
use dlc_manager::contract::offered_contract::OfferedContract;
use dlc_manager::contract::signed_contract::SignedContract;
use dlc_manager::error::Error;
use dlc_sled_storage_provider::SledStorageProvider;
use tokio::runtime::Runtime;
use crate::storage::memory_storage::MemoryStorage;

pub struct StorageProvider {

    runtime: Runtime,

    memory_storage: MemoryStorage,

    sled_storage: Option<SledStorageProvider>,

    storage_api: Option<StorageApiClient>
}

impl StorageProvider {

    pub fn new() -> Self {
        let memory_storage = MemoryStorage::new();
        let use_storage_api: bool = env::var("STORAGE_API_ENABLED")
            .unwrap_or("false".to_string())
            .parse().unwrap();
        let storage_api_endpoint: String = env::var("STORAGE_API_ENDPOINT").unwrap_or("http://localhost:8100".to_string());
        let use_sled: bool = env::var("SLED_ENABLED")
            .unwrap_or("false".to_string())
            .parse().unwrap();
        let sled_path: String = env::var("SLED_PATH").unwrap_or("contracts_db".to_string());
        if use_storage_api {
            info!("Storage API enabled: {}", use_storage_api);
            Self {runtime: Runtime::new().unwrap(), memory_storage: memory_storage, sled_storage: None, storage_api: Some(StorageApiClient::new(storage_api_endpoint))}
        } else if use_sled {
            info!("Sled enabled: {}", use_sled);
            Self {runtime: Runtime::new().unwrap(), memory_storage: memory_storage, sled_storage: Some(SledStorageProvider::new(sled_path.as_str()).unwrap()), storage_api: None}
        } else {
            Self {runtime: Runtime::new().unwrap(), memory_storage: memory_storage, sled_storage: None, storage_api: None}
        }
    }

    pub fn delete_contracts(&self) {
        if self.storage_api.is_some() {
            let _res = self.runtime.block_on(self.storage_api.as_ref().unwrap().delete_contracts());
        } else if self.sled_storage.is_some() {
            // TODO
        } else {
            self.memory_storage.delete_contracts()
        }
    }

    pub fn get_contracts_by_state(&self, state: String) -> Result<Vec<Contract>, Error> {
        let contracts_res: Result<Vec<dlc_clients::Contract>, ApiError> = self.runtime.block_on(self.storage_api.as_ref().unwrap().get_contracts_by_state(state.clone()));
        let mut contents: Vec<String> = vec![];
        let mut contracts: Vec<Contract> = vec![];
        for c in contracts_res.unwrap() {
            contents.push(c.content);
        }
        for c in contents {
            let bytes = base64::decode(c.clone()).unwrap();
            let contract = deserialize_contract(&bytes).unwrap();
            contracts.push(contract);
        }
        Ok(contracts)
    }
}

impl Default for StorageProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl Storage for StorageProvider {
    
    fn get_contract(&self, id: &ContractId) -> Result<Option<Contract>, Error> {
        if self.storage_api.is_some() {
            let bytes = id.to_vec();
            let cid = base64::encode(&bytes);
            let contract_res : Result<Option<dlc_clients::Contract>, ApiError> = self.runtime.block_on(self.storage_api.as_ref().unwrap().get_contract(cid.clone()));
            let unw_contract = contract_res.unwrap();
            if unw_contract.is_some() {
                let bytes = base64::decode(unw_contract.unwrap().content).unwrap();
                let contract = deserialize_contract(&bytes)?;
                Ok(Some(contract))
            } else {
                info!("Contract not found with id: {}", cid);
                Ok(None)
            }
        } else if self.sled_storage.is_some() {
            self.sled_storage.as_ref().unwrap().get_contract(id)
        } else {
            self.memory_storage.get_contract(id)
        }
    }

    fn get_contracts(&self) -> Result<Vec<Contract>, Error> {
        if self.storage_api.is_some() {
            let contracts_res : Result<Vec<dlc_clients::Contract>, ApiError> = self.runtime.block_on(self.storage_api.as_ref().unwrap().get_contracts());
            let mut contents: Vec<String> = vec![];
            let mut contracts: Vec<Contract> = vec![];
            for c in contracts_res.unwrap() {
                contents.push(c.content);
            }
            for c in contents {
                let bytes = base64::decode(c.clone()).unwrap();
                let contract = deserialize_contract(&bytes).unwrap();
                contracts.push(contract);
            }
            Ok(contracts)
        } else if self.sled_storage.is_some() {
            self.sled_storage.as_ref().unwrap().get_contracts()
        } else {
            self.memory_storage.get_contracts()
        }
    }

    fn create_contract(&mut self, contract: &OfferedContract) -> Result<(), Error> {
        if self.storage_api.is_some() {
            let data = serialize_contract(&Contract::Offered(contract.clone()))?;
            let bytes = contract.id.to_vec();
            let uuid = base64::encode(&bytes);
            let req = NewContract{uuid: uuid.clone(), state: "offered".to_string(), content: base64::encode(&data)};
            let _result = self.runtime.block_on(self.storage_api.as_mut().unwrap().create_contract(req));
            Ok(())
        } else if self.sled_storage.is_some() {
            self.sled_storage.as_mut().unwrap().create_contract(contract)
        } else {
            self.memory_storage.create_contract(contract)
        }
    }

    fn delete_contract(&mut self, id: &ContractId) -> Result<(), Error> {
        if self.storage_api.is_some() {
            let bytes = id.to_vec();
            let cid = base64::encode(&bytes);
            let _result = self.runtime.block_on(self.storage_api.as_mut().unwrap().delete_contract(cid.clone()));
            Ok(())
        } else if self.sled_storage.is_some() {
            self.sled_storage.as_mut().unwrap().delete_contract(id)
        } else {
            self.memory_storage.delete_contract(id)
        }
    }

    fn update_contract(&mut self, contract: &Contract) -> Result<(), Error> {
        if self.storage_api.is_some() {
            let c_id = contract.get_id();
            let bytes = c_id.to_vec();
            let contract_id : String = base64::encode(&bytes);
            let curr_state = get_contract_state_str(contract);
            match contract {
                a @ Contract::Accepted(_) | a @ Contract::Signed(_) => {
                    let _res = self.delete_contract(&a.get_temporary_id());
                }
                _ => {}
            };
            let contract_res : Result<Option<dlc_clients::Contract>, ApiError> = self.runtime.block_on(self.storage_api.as_ref().unwrap().get_contract(contract_id.clone()));
            let unw_contract = contract_res.unwrap();
            let data = serialize_contract(contract).unwrap();
            let encoded_content = base64::encode(&data);
            if unw_contract.is_some() {
                let _res = self.runtime.block_on(self.storage_api.as_ref().unwrap().update_contract(contract_id.clone(), UpdateContract{state: Some(curr_state.clone()), content: Some(encoded_content)}));
                Ok(())
            } else {
                let _res = self.runtime.block_on(self.storage_api.as_ref().unwrap().create_contract(NewContract{ uuid: contract_id.clone(), state: curr_state.clone(), content: encoded_content}));
                Ok(())
            }
        } else if self.sled_storage.is_some() {
            self.sled_storage.as_mut().unwrap().update_contract(contract)
        } else {
            self.memory_storage.update_contract(contract)
        }
    }

    fn get_contract_offers(&self) -> Result<Vec<OfferedContract>, Error> {
        if self.storage_api.is_some() {
            let contracts_per_state = self.get_contracts_by_state("offered".to_string()).unwrap();
            let mut res: Vec<OfferedContract> = Vec::new();
            for val in contracts_per_state {
                if let Contract::Offered(c) = val {
                    res.push(c.clone());
                }
            }
            return Ok(res);
        } else if self.sled_storage.is_some() {
            self.sled_storage.as_ref().unwrap().get_contract_offers()
        } else {
            self.memory_storage.get_contract_offers()
        }
    }

    fn get_signed_contracts(&self) -> Result<Vec<SignedContract>, Error> {
        if self.storage_api.is_some() {
            let contracts_per_state = self.get_contracts_by_state("signed".to_string()).unwrap();
            let mut res: Vec<SignedContract> = Vec::new();
            for val in contracts_per_state {
                if let Contract::Signed(c) = val {
                    res.push(c.clone());
                }
            }
            return Ok(res);
        } else if self.sled_storage.is_some() {
            self.sled_storage.as_ref().unwrap().get_signed_contracts()
        } else {
            self.memory_storage.get_signed_contracts()
        }
    }

    fn get_confirmed_contracts(&self) -> Result<Vec<SignedContract>, Error> {
        if self.storage_api.is_some() {
            let contracts_per_state = self.get_contracts_by_state("confirmed".to_string()).unwrap();
            let mut res: Vec<SignedContract> = Vec::new();
            for val in contracts_per_state {
                if let Contract::Confirmed(c) = val {
                    res.push(c.clone());
                }
            }
            return Ok(res);
        } else if self.sled_storage.is_some() {
            self.sled_storage.as_ref().unwrap().get_confirmed_contracts()
        } else {
            self.memory_storage.get_confirmed_contracts()
        }
    }

    fn get_preclosed_contracts(&self) -> Result<Vec<PreClosedContract>, Error> {
        if self.storage_api.is_some() {
            let contracts_per_state = self.get_contracts_by_state("pre_closed".to_string()).unwrap();
            let mut res: Vec<PreClosedContract> = Vec::new();
            for val in contracts_per_state {
                if let Contract::PreClosed(c) = val {
                    res.push(c.clone());
                }
            }
            return Ok(res);
        } else if self.sled_storage.is_some() {
            self.sled_storage.as_ref().unwrap().get_preclosed_contracts()
        } else {
            self.memory_storage.get_preclosed_contracts()
        }
    }
}

use dlc_manager::contract::accepted_contract::AcceptedContract;
use dlc_manager::contract::ser::Serializable;
use dlc_manager::contract::{
    ClosedContract, FailedAcceptContract, FailedSignContract,
};
use dlc_clients::{ApiError, NewContract, StorageApiClient, UpdateContract};

fn to_storage_error<T>(e: T) -> Error
where
    T: std::fmt::Display,
{
    Error::StorageError(e.to_string())
}

macro_rules! convertible_enum {
    (enum $name:ident {
        $($vname:ident $(= $val:expr)?,)*;
        $($tname:ident $(= $tval:expr)?,)*
    }, $input:ident) => {
        #[derive(Debug)]
        enum $name {
            $($vname $(= $val)?,)*
            $($tname $(= $tval)?,)*
        }

        impl From<$name> for u8 {
            fn from(prefix: $name) -> u8 {
                prefix as u8
            }
        }

        impl std::convert::TryFrom<u8> for $name {
            type Error = Error;

            fn try_from(v: u8) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == u8::from($name::$vname) => Ok($name::$vname),)*
                    $(x if x == u8::from($name::$tname) => Ok($name::$tname),)*
                    _ => Err(Error::StorageError("Unknown prefix".to_string())),
                }
            }
        }

        impl $name {
            fn get_prefix(input: &$input) -> u8 {
                let prefix = match input {
                    $($input::$vname(_) => $name::$vname,)*
                    $($input::$tname{..} => $name::$tname,)*
                };
                prefix.into()
            }
        }
    }
}

convertible_enum!(
    enum ContractPrefix {
        Offered = 1,
        Accepted,
        Signed,
        Confirmed,
        PreClosed,
        Closed,
        FailedAccept,
        FailedSign,
        Refunded,;
    },
    Contract
);

fn serialize_contract(contract: &Contract) -> Result<Vec<u8>, ::std::io::Error> {
    let serialized = match contract {
        Contract::Offered(o) => o.serialize(),
        Contract::Accepted(o) => o.serialize(),
        Contract::Signed(o) | Contract::Confirmed(o) | Contract::Refunded(o) => o.serialize(),
        Contract::FailedAccept(c) => c.serialize(),
        Contract::FailedSign(c) => c.serialize(),
        Contract::PreClosed(c) => c.serialize(),
        Contract::Closed(c) => c.serialize(),
    };
    let mut serialized = serialized?;
    let mut res = Vec::with_capacity(serialized.len() + 1);
    res.push(ContractPrefix::get_prefix(contract));
    res.append(&mut serialized);
    Ok(res)
}

fn deserialize_contract(buff: &Vec<u8>) -> Result<Contract, Error> {
    let mut cursor = ::std::io::Cursor::new(buff);
    let mut prefix = [0u8; 1];
    std::io::Read::read_exact(&mut cursor, &mut prefix)?;
    let contract_prefix: ContractPrefix = prefix[0].try_into()?;
    let contract = match contract_prefix {
        ContractPrefix::Offered => {
            Contract::Offered(OfferedContract::deserialize(&mut cursor).map_err(to_storage_error)?)
        }
        ContractPrefix::Accepted => Contract::Accepted(
            AcceptedContract::deserialize(&mut cursor).map_err(to_storage_error)?,
        ),
        ContractPrefix::Signed => {
            Contract::Signed(SignedContract::deserialize(&mut cursor).map_err(to_storage_error)?)
        }
        ContractPrefix::Confirmed => {
            Contract::Confirmed(SignedContract::deserialize(&mut cursor).map_err(to_storage_error)?)
        }
        ContractPrefix::PreClosed => Contract::PreClosed(
            PreClosedContract::deserialize(&mut cursor).map_err(to_storage_error)?,
        ),
        ContractPrefix::Closed => {
            Contract::Closed(ClosedContract::deserialize(&mut cursor).map_err(to_storage_error)?)
        }
        ContractPrefix::FailedAccept => Contract::FailedAccept(
            FailedAcceptContract::deserialize(&mut cursor).map_err(to_storage_error)?,
        ),
        ContractPrefix::FailedSign => Contract::FailedSign(
            FailedSignContract::deserialize(&mut cursor).map_err(to_storage_error)?,
        ),
        ContractPrefix::Refunded => {
            Contract::Refunded(SignedContract::deserialize(&mut cursor).map_err(to_storage_error)?)
        }
    };
    Ok(contract)
}

fn get_contract_state_str(contract: &Contract) -> String {
    let state = match contract {
        Contract::Offered(_) => "offered",
        Contract::Accepted(_) => "accepted",
        Contract::Signed(_) => "signed",
        Contract::Confirmed(_) => "confirmed",
        Contract::PreClosed(_) => "pre_closed",
        Contract::Closed(_) => "closed",
        Contract::Refunded(_) => "refunded",
        Contract::FailedAccept(_) => "failed_accept",
        Contract::FailedSign(_) => "failed_sign",
    };
    return state.to_string();
}
