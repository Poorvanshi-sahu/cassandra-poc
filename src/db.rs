use crate::models::{PartialUser, User};
use scylla::{Session, SessionBuilder};
use uuid::Uuid;
// use scylla::frame::value::Value;

pub async fn connect_db() -> Session {
    SessionBuilder::new()
        .known_node("127.0.0.1:9042")
        .build()
        .await
        .expect("Failed to connect to Cassandra")
}

pub async fn init_db(session: &Session) {
    session
        .query(
            "CREATE KEYSPACE IF NOT EXISTS userks WITH replication = {'class': 'SimpleStrategy', 'replication_factor': 1}",
            &[],
        )
        .await
        .unwrap();

    session
        .query(
            "CREATE TABLE IF NOT EXISTS userks.users (id UUID PRIMARY KEY, name TEXT, email TEXT)",
            &[],
        )
        .await
        .unwrap();
}

pub async fn insert_user(session: &Session, user: &User) {
    session
        .query(
            "INSERT INTO userks.users (id, name, email) VALUES (?, ?, ?)",
            (user.id, &user.name, &user.email),
        )
        .await
        .unwrap();
}

pub async fn get_all_users(session: &Session) -> Vec<User> {
    let rows = session
        .query("SELECT id, name, email FROM userks.users", &[])
        .await
        .unwrap()
        .rows
        .unwrap();

    rows.into_iter()
        .map(|row| {
            let (id, name, email): (Uuid, String, String) = row.into_typed().unwrap();
            User {
                id: Some(id),
                name,
                email,
            }
        })
        .collect()
}

pub async fn delete_user(
    session: &Session,
    user_id: Uuid,
) -> Result<(), scylla::transport::errors::QueryError> {
    let query = "DELETE FROM userks.users WHERE id = ?";
    session.query(query, (user_id,)).await?;
    Ok(())
}

pub async fn update_user(
    session: &Session,
    user_id: Uuid,
    partial_user: PartialUser,
) -> Result<(), scylla::transport::errors::QueryError> {
    let mut query = String::from("UPDATE userks.users SET ");
    let mut params: Vec<&dyn scylla::frame::value::Value> = Vec::new();

    if let Some(name) = &partial_user.name {
        query.push_str("name = ?, ");
        params.push(name as &dyn scylla::frame::value::Value); // Pass as &str
    }

    if let Some(email) = &partial_user.email {
        query.push_str("email = ?, ");
        params.push(email as &dyn scylla::frame::value::Value); // Pass as &str
    }

    // Remove the trailing comma and space, and add the WHERE clause
    query.truncate(query.len() - 2);
    query.push_str(" WHERE id = ?");

    params.push(&user_id as &dyn scylla::frame::value::Value); // Add the user_id

    session.query(query, params).await?;
    Ok(())
}

pub async fn get_user_by_id(
    session: &Session,
    user_id: Uuid,
) -> Result<Option<User>, scylla::transport::errors::QueryError> {
    let query = "SELECT id, name, email FROM userks.users WHERE id = ?";
    let result = session.query(query, (user_id,)).await?;

    if let Some(row) = result.rows.and_then(|mut rows| rows.pop()) {
        let (id, name, email): (Uuid, Option<String>, Option<String>) = row.into_typed().unwrap();
        Ok(Some(User {
            id: Some(id),
            name: name.unwrap_or_default(),
            email: email.unwrap_or_default(),
        }))
    } else {
        Ok(None) // User not found
    }
}

pub async fn get_user_by_email(
    session: &Session,
    email: &str,
) -> Result<Option<User>, scylla::transport::errors::QueryError> {
    let query = "SELECT id, name, email FROM userks.users WHERE email = ? ALLOW FILTERING";
    let result = session.query(query, (email,)).await?;

    if let Some(row) = result.rows.and_then(|mut rows| rows.pop()) {
        let (id, name, email): (Uuid, Option<String>, Option<String>) = row.into_typed().unwrap();
        Ok(Some(User {
            id: Some(id),
            name: name.unwrap_or_default(),
            email: email.unwrap_or_default(),
        }))
    } else {
        Ok(None) // Email not found
    }
}