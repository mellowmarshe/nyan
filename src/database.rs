use std::{error::Error, path::Path};

use sqlx::{migrate::Migrator, postgres::PgPoolOptions, PgPool};

use crate::constants;

#[derive(Clone)]
pub struct ConnectionPool {
    pub pool: PgPool,
}

impl ConnectionPool {
    pub async fn new() -> Result<Self, Box<dyn Error + Send + Sync>> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&constants::CONFIG.database.postgres)
            .await?;

        Ok(Self { pool })
    }

    pub async fn migrations(&self) -> Result<(), sqlx::Error> {
        Migrator::new(Path::new("./migrations"))
            .await?
            .run(&self.pool)
            .await?;

        Ok(())
    }
}
