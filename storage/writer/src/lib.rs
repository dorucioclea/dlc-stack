use dlc_storage_common;
use diesel::PgConnection;
use dlc_storage_common::models::{Contract, NewContract, UpdateContract};

pub fn apply_migrations(conn: &mut PgConnection) {
    let _ = dlc_storage_common::run_migrations(conn);
}

pub fn create_contract(conn: &mut PgConnection, contract: NewContract) -> Result<Contract, diesel::result::Error> {
    return dlc_storage_common::create_contract(conn, contract);
}

pub fn update_contract(conn: &mut PgConnection, cuuid: &str, contract: UpdateContract) -> Result<usize, diesel::result::Error> {
    return dlc_storage_common::update_contract(conn, cuuid, contract);
}

pub fn delete_contract(conn: &mut PgConnection, cuuid: &str) -> Result<usize, diesel::result::Error> {
    return dlc_storage_common::delete_contract(conn, cuuid);
}
