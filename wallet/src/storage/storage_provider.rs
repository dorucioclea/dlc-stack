use log::info;
use std::env;
use dlc_manager::{ContractId, Storage};
use dlc_manager::contract::{Contract, PreClosedContract};
use dlc_manager::contract::offered_contract::OfferedContract;
use dlc_manager::contract::signed_contract::SignedContract;
use dlc_manager::error::Error;
use dlc_sled_storage_provider::SledStorageProvider;
use crate::storage::memory_storage::MemoryStorage;

pub struct StorageProvider {

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
        info!("Sled enabled: {}", use_sled);
        let sled_path: String = env::var("SLED_PATH").unwrap_or("contracts_db".to_string());
        if use_storage_api {
            Self {memory_storage: memory_storage, sled_storage: None, storage_api: Some(StorageApiClient::new(storage_api_endpoint))}
        } else if use_sled {
            Self {memory_storage: memory_storage, sled_storage: Some(SledStorageProvider::new(sled_path.as_str()).unwrap()), storage_api: None}
        } else {
            Self {memory_storage: memory_storage, sled_storage: None, storage_api: None}
        }
    }

    pub fn delete_contracts(&self) {
        if self.storage_api.is_some() {
            // TODO
        } else if self.sled_storage.is_some() {
            // TODO
        } else {
            self.memory_storage.delete_contracts()
        }
    }
}

impl Default for StorageProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl Storage for StorageProvider {
    
    fn get_contract(&self, id: &ContractId) -> Result<Option<Contract>, Error> {
        //if self.storage_api.is_some() {
        //    self.storage_api.as_ref().unwrap();
        //} else
        if self.sled_storage.is_some() {
            self.sled_storage.as_ref().unwrap().get_contract(id)
        } else {
            self.memory_storage.get_contract(id)
        }
    }

    fn get_contracts(&self) -> Result<Vec<Contract>, Error> {
        if self.sled_storage.is_some() {
            self.sled_storage.as_ref().unwrap().get_contracts()
        } else {
            self.memory_storage.get_contracts()
        }
    }

    fn create_contract(&mut self, contract: &OfferedContract) -> Result<(), Error> {
        if self.sled_storage.is_some() {
            self.sled_storage.as_mut().unwrap().create_contract(contract)
        } else {
            self.memory_storage.create_contract(contract)
        }
    }

    fn delete_contract(&mut self, id: &ContractId) -> Result<(), Error> {
        if self.sled_storage.is_some() {
            self.sled_storage.as_mut().unwrap().delete_contract(id)
        } else {
            self.memory_storage.delete_contract(id)
        }
    }

    fn update_contract(&mut self, contract: &Contract) -> Result<(), Error> {
        if self.sled_storage.is_some() {
            self.sled_storage.as_mut().unwrap().update_contract(contract)
        } else {
            self.memory_storage.update_contract(contract)
        }
    }

    fn get_contract_offers(&self) -> Result<Vec<OfferedContract>, Error> {
        if self.sled_storage.is_some() {
            self.sled_storage.as_ref().unwrap().get_contract_offers()
        } else {
            self.memory_storage.get_contract_offers()
        }
    }

    fn get_signed_contracts(&self) -> Result<Vec<SignedContract>, Error> {
        if self.sled_storage.is_some() {
            self.sled_storage.as_ref().unwrap().get_signed_contracts()
        } else {
            self.memory_storage.get_signed_contracts()
        }
    }

    fn get_confirmed_contracts(&self) -> Result<Vec<SignedContract>, Error> {
        if self.sled_storage.is_some() {
            self.sled_storage.as_ref().unwrap().get_confirmed_contracts()
        } else {
            self.memory_storage.get_confirmed_contracts()
        }
    }

    fn get_preclosed_contracts(&self) -> Result<Vec<PreClosedContract>, Error> {
        if self.sled_storage.is_some() {
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
use dlc_clients::StorageApiClient;

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