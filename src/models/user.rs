use sqlx::{MySqlPool,mysql::MySqlQueryResult};
use serde::{Deserialize, Serialize};
use chrono::{
    Utc
};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]

pub struct User {
    pub id: u64,
    pub address: String,
    pub create_time:u64,
}

pub async fn find_or_create_user(pool: &MySqlPool, address: &str) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, address, create_time FROM users WHERE address = ?"
    )
    .bind(address)
    .fetch_optional(pool)
    .await?;

    if let Some(user) = user {
        Ok(user)
    } else {
        let create_time = Utc::now().timestamp() as u64;
        let result: MySqlQueryResult = sqlx::query(
            "INSERT INTO users (address, create_time) VALUES (?, ?)"
        )
        .bind(address)
        .bind(create_time)
        .execute(pool)
        .await?;

        // 2. 获取刚插入的自增 id
        let last_id = result.last_insert_id();

        // 3. 根据 id 完整用户信息
        let new_user = User { 
            id:last_id, 
            address: address.to_string(), 
            create_time: create_time 
        };
        Ok(new_user)
    }
}