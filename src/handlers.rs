use crate::{
    db,
    models::{PartialUser, User},
    redis,
};

use crate::utils::{success_response, error_response};

use actix_web::{Responder, delete, get, post, web};
use quick_xml::de::from_str;
use scylla::Session;
use serde_json::json;
use uuid::Uuid;
use regex::Regex;

#[get("/users")]
pub async fn get_users(session: web::Data<Session>) -> impl Responder {
    // Check if the list of users is cached
    if let Some(cached_users) = redis::get_cached_users().await {
        if !cached_users.is_empty() {
            return success_response("All users from cache", Some(cached_users));        }
    }

    // If not cached or cache is empty, query the database
    let users = db::get_all_users(&session).await;

    // Cache the result in Redis
    redis::cache_users(&users).await;

    success_response("All users", Some(users))}

#[get("/get_user/{id}")]
pub async fn get_user(
    id: web::Path<String>,
    session: web::Data<Session>,
) -> impl Responder {
    let user_id = id.into_inner();

    // Check if the user is cached in Redis
    if let Some(cached_user) = redis::get_cached_user(&user_id).await {
        return success_response("User from cache", Some(cached_user));    
    }

    // If not cached, query the database
    match db::get_user_by_id(&session, Uuid::parse_str(&user_id).unwrap()).await {
        Ok(Some(user)) => {
            // Cache the user in Redis for future requests
            redis::cache_user(&user).await;
            success_response("User fetched successfully", Some(user))
        }
        Ok(None) => error_response(&format!("No user found with ID {}", user_id), 404),
        Err(err) => error_response(&format!("Database error: {}", err), 500),
    }
}

#[post("/add_user")]
pub async fn add_user(session: web::Data<Session>, body: String) -> impl Responder {
    let mut user: User = match from_str(&body) {
        Ok(user) => user,
        Err(err) => return error_response(&format!("Invalid input: {}", err), 400),
    };

    match db::get_user_by_email(&session, &user.email).await {
        Ok(Some(_)) => {
            // Email already exists
            return error_response(&format!("Email {} is already in use", user.email), 400);
        }
        Ok(None) => {
            // Email does not exist, proceed with adding the user
        }
        Err(err) => {
            // Database error
            return error_response(&format!("Database error: {}", err), 500);
        }
    }

    user.id = Some(Uuid::new_v4());
    
    db::insert_user(&session, &user).await;

    let users = db::get_all_users(&session).await;

    redis::cache_users(&users).await;

    success_response("User added successfully", Some(user))
}

#[delete("/delete_user/{id}")]
pub async fn delete_user(session: web::Data<Session>, id: web::Path<Uuid>) -> impl Responder {
    let user_id = id.into_inner();

    match db::get_user_by_id(&session, user_id).await {
        Ok(Some(_)) => {}
        Ok(None) => return error_response(&format!("No user found with ID {}", user_id), 404),
        Err(err) => return error_response(&format!("Database error: {}", err), 500),
    }

    match db::delete_user(&session, user_id).await {
        Ok(_) => {
            redis::remove_cached_user(&user_id.to_string()).await;

            let users = db::get_all_users(&session).await;
            redis::cache_users(&users).await;

            success_response(&format!("User with ID {} deleted", user_id),  Some(json!({
                "id": user_id,
                "status": "deleted"
            })))
        },
        Err(err) => {
            error_response(&format!("Failed to delete user: {}", err), 500)
        }
    }
}

#[post("/update_user/{id}")]
pub async fn update_user(
    session: web::Data<Session>,
    id: web::Path<Uuid>,
    body: String,
) -> impl Responder {
    let user_id = id.into_inner();

    match db::get_user_by_id(&session, user_id).await {
        Ok(Some(_)) => {
            // User exists, proceed with deletion
        }
        Ok(None) => {
            // User does not exist
            return error_response(&format!("No user found with ID {}", user_id), 404);
        }
        Err(err) => {
            // Database error
            return error_response(&format!("Database error: {}", err), 500);
        }
    }

    let partial_user: PartialUser = match from_str(&body) {
        Ok(user) => user,
        Err(err) => return error_response(&format!("Invalid input: {}", err), 400),
    };

    if let Some(name) = &partial_user.name {
        if name.trim().is_empty() {
            return error_response("Name cannot be blank", 400);
        }
    }

    if let Some(email) = &partial_user.email {
        let email_regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
        if !email_regex.is_match(email) {
            return error_response("Invalid email format", 400);
        }
    }

    match db::update_user(&session, user_id, partial_user).await {
        Ok(_) => { 
            if let Ok(Some(updated_user)) = db::get_user_by_id(&session, user_id).await {
                redis::cache_user(&updated_user).await;

                let users = db::get_all_users(&session).await;
                redis::cache_users(&users).await;

                return success_response("User updated successfully", Some(updated_user));
            }
            success_response("User updated successfully, but failed to fetch updated data", Some(json!({
                "id": user_id,
                "status": "updated"
            })))
        },
        Err(err) => {
            error_response(&format!("Failed to update user: {}", err), 500)
        }
    }
}

