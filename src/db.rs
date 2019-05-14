use diesel::PgConnection;
use diesel::r2d2::{
    ConnectionManager,
    Pool,
    PooledConnection
};

pub mod models;

pub struct Database {
    pool: Pool<ConnectionManager<PgConnection>>
}

impl Database {
    pub fn connect() -> Self {
        let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .max_size(15)
            .build(manager)
            .expect("Failed to create connection pool");

        Database {
            pool
        }
    }

    fn conn(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool.clone().get().expect("Attempt to get connection timed out")
    }

    // TODO setup database functions
}