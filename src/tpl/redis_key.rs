pub enum RedisKeyTemplate {
    Blacklist(String),
}

impl RedisKeyTemplate {
    pub fn format(&self) -> String {
        match self {
            RedisKeyTemplate::Blacklist(key) => format!("blacklist:{}", key),
        }
    }
}
