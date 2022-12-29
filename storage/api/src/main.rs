mod contracts;
mod events;

use contracts::{create_contract, delete_contract, get_contracts, get_contracts_by_state, get_contract, update_contract};
extern crate log;
use actix_web::{App, HttpServer};
use diesel::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;
use actix_web::web::Data;
use dlc_storage_writer::apply_migrations;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();
    // e.g.: DATABASE_URL=postgresql://postgres:changeme@localhost:5432/postgres
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
    let mut conn = pool.get().expect("Failed to get connection from pool");
    let migrate: bool = env::var("MIGRATE")
        .unwrap_or("false".to_string())
        .parse().unwrap();
    if migrate {
        apply_migrations(&mut conn);
    }
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(get_contracts)
            .service(get_contract)
            .service(get_contracts_by_state)
            .service(create_contract)
            .service(update_contract)
            .service(delete_contract)
    })
        .bind("127.0.0.1:8100")?
        .run()
        .await
}
