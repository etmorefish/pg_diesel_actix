use crate::error_handle::CustomError;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use r2d2;
use std::env;


type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

embed_migrations!();

lazy_static!{
    static ref POOL:Pool = {
        let db_url=env::var("DB_URL").expect("DB_URL cannot be empty");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("ConnectionManager::new() failed");
    };
}

pub fn connection() -> Result<DbConnection, CustomError> {
    POOL.get().map_err(|e| CustomError::new(500, format!("getting connection error: {}", e)))
}

pub fn init(){
    lazy_static::initialize(&POOL);
    let conn = Connection().expect("DB_URL is not initialized");
    embedded_migrations::run(&conn).unwrap();
}