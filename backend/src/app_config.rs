use std::env::var;

use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

#[derive(Clone)]
pub struct DB(pub DatabaseConnection);

impl DB {
    pub async fn get() -> Result<Self, Box<dyn std::error::Error>> {
        dotenvy::dotenv().expect("failed to read .env file");
        let db_path = var("DATABASE_URL").unwrap();
        log::info!("Connecting to database: {}", db_path);
        let options = ConnectOptions::new(&db_path);
        let conn: DatabaseConnection = Database::connect(options).await.unwrap();
        Migrator::up(&conn, None).await?;
        Ok(DB(conn))
    }
}
