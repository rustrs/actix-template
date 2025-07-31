
use sqlx::MySqlPool;
use deadpool_redis::Pool as RedisPool;

use crate::config::config::AppConfig;


#[derive(Clone)]
pub struct AppState {
    pub mysql: MySqlPool,
    pub redis: RedisPool,
    pub config: AppConfig,
}


impl AppState {
    pub fn new(mysql: MySqlPool, redis: RedisPool,config:AppConfig) -> Self {
        AppState { mysql, redis,config }
    }

    pub fn mysql(&self) -> &MySqlPool {
        &self.mysql
    }

    pub fn redis(&self) -> &RedisPool {
        &self.redis
    }

    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    pub async fn close(&self) {
        self.mysql.close().await;
        self.redis.close();
    }
}