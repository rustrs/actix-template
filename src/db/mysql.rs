
use serde::{Deserialize,Serialize};

use sqlx::MySqlPool;
use sqlx::mysql::MySqlPoolOptions;
use tokio::time::Duration;
use urlencoding::encode;



#[derive(Debug, Clone, Deserialize,Serialize)]
pub struct MySQLConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
    pub pool_max_size:u32, //20~50
    pub pool_min_size:u32, //1~5
    pub connect_timeout: u64, //5
    pub idle_timeout: u64,//connection idle timeout, 300
    pub max_lifetime: u64,// connection max lifetime, 3600
}
impl MySQLConfig {
    pub fn to_url(&self) -> String {
        let password = encode(&self.password);
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.user, password, self.host, self.port, self.database
        )
    }
}

pub async fn init_mysql_pool(db_cfg: &MySQLConfig) -> MySqlPool {
    let url = db_cfg.to_url();
    
    match MySqlPoolOptions::new()
        .min_connections(db_cfg.pool_min_size)
        .max_connections(db_cfg.pool_max_size)
        .acquire_timeout(Duration::from_secs(db_cfg.connect_timeout))
        .idle_timeout(Duration::from_secs(db_cfg.idle_timeout))
        .max_lifetime(Duration::from_secs(db_cfg.max_lifetime))
        .connect(&url)
        .await
    {
        Ok(pool) => pool,
        Err(err) => {
            eprintln!("❌ Failed to connect to MySQL: {}", err);
            std::process::exit(1);
        }
    }
}