use deadpool_redis::{Config, Pool,Runtime};
use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, Deserialize,Serialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub db: u8,
    pub pool_max_size: u32, // 10~100
    pub pool_min_size: u32, // 2~10
    pub max_lifetime: u64, // 3600
    pub idle_timeout: u64, // 300
    pub connect_timeout: u64, // 5
    pub use_tls: bool, // false
}

impl RedisConfig {
    pub fn to_url(&self) -> String {
        let mut url:String;
        let protocol = if self.use_tls { "rediss" } else { "redis" };
        if let Some(ref user) = self.username && let Some(ref pass) = self.password {
            url = format!("{}://{}:{}@{}:{}", protocol, 
            user,pass, 
            self.host, self.port);
        } else if let Some(ref pass) = self.password {
            url = format!("{}://:{}@{}:{}", protocol, pass, self.host, self.port);
        }else {
            url = format!("{}://{}:{}", protocol, self.host, self.port);
        }
        url.push_str(&format!("/{}", self.db));
        url
    }
}

pub fn init_redis_pool(db_cfg: &RedisConfig) -> Pool {
    
    let cfg = Config::from_url(&db_cfg.to_url());
    cfg.create_pool(Some(Runtime::Tokio1)).unwrap()
   
}