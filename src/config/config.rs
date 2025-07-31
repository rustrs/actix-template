

use crate::db::mysql::MySQLConfig;
use crate::db::redis::RedisConfig;
use serde::{Deserialize,Serialize};
use config::{Config as configLoader,File as ConfigFile};

use crate::utils::logger::{LogConfig};

#[derive(Debug, Clone, Deserialize,Serialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub mysql: MySQLConfig,
    pub redis: RedisConfig,
    pub jwt: JWTConfig,
    pub log: LogConfig,
}

#[derive(Debug, Clone, Deserialize,Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub debug: bool,
}

#[derive(Debug, Clone, Deserialize,Serialize)]
pub struct JWTConfig {
    pub secret: String,
    pub expiration: u64, // in seconds
}



impl AppConfig {
    pub fn new(config_file: &String) -> Self {
        configLoader::builder()
            .add_source(ConfigFile::with_name(config_file))
            .build()
            .expect("Failed to load configuration")
            .try_deserialize::<AppConfig>()
            .expect("Failed to deserialize configuration")  
    }


    pub fn server(&self) -> &ServerConfig {
        &self.server
    }

    pub fn mysql(&self) -> &MySQLConfig {
        &self.mysql
    }

    pub fn redis(&self) -> &RedisConfig {
        &self.redis
    }

    pub fn jwt(&self) -> &JWTConfig {
        &self.jwt
    }
    pub fn log(&self) -> &LogConfig {
        &self.log
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }           
}