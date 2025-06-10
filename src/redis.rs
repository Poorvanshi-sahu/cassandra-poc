use crate::models::User;
use redis::AsyncCommands;

pub async fn cache_user(user: &User) {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_async_connection().await.unwrap();
    let _: () = con
        .set(
            user.id.expect("REASON").to_string(),
            serde_json::to_string(user).unwrap(),
        )
        .await
        .unwrap();
}

pub async fn get_cached_user(id: &str) -> Option<User> {
    let client: redis::Client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_async_connection().await.unwrap();
    let val: Option<String> = con.get(id).await.unwrap();
    val.map(|v| serde_json::from_str(&v).unwrap())
}

pub async fn remove_cached_user(id: &str) {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_async_connection().await.unwrap();
    let _: () = con.del(id).await.unwrap();
}

// pub async fn cache_users(users: &[User]) {
//     let client = redis::Client::open("redis://127.0.0.1/").unwrap();
//     let mut con = client.get_async_connection().await.unwrap();
//     let _: () = con
//         .set("all_users", serde_json::to_string(users).unwrap())
//         .await
//         .unwrap();
// }

pub async fn cache_users(users: &[User]) {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_async_connection().await.unwrap();
    let _: () = con
        .set("all_users", serde_json::to_string(users).unwrap())
        .await
        .unwrap();
}

// pub async fn get_cached_users() -> Option<Vec<User>> {
//     let client = redis::Client::open("redis://127.0.0.1/").unwrap();
//     let mut con = client.get_async_connection().await.unwrap();
//     let val: Option<String> = con.get("all_users").await.unwrap();
//     val.map(|v| serde_json::from_str(&v).unwrap())
// }

pub async fn get_cached_users() -> Option<Vec<User>> {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_async_connection().await.unwrap();
    let val: Option<String> = con.get("all_users").await.unwrap();

    // Check if the value exists and is valid
    match val {
        Some(data) => {
            match serde_json::from_str(&data) {
                Ok(users) => Some(users), // Return the deserialized users
                Err(_) => None,          // Return None if deserialization fails
            }
        }
        None => None, // Return None if the cache is empty
    }
}