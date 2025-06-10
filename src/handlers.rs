use crate::{
    db,
    models::{PartialUser, User},
    redis,
};
use actix_web::{HttpResponse, Responder, delete, get, post, web};
use quick_xml::de::from_str;
use scylla::Session;
use serde_json::json;
use uuid::Uuid;
use regex::Regex;

// #[get("/users")]
// pub async fn get_users(session: web::Data<Session>) -> impl Responder {
//     if let Some(cached_users) = redis::get_cached_users().await {
//         return HttpResponse::Ok().json(json!({ "message": "all users from cache", "success": true, "data": cached_users }));
//     }

//     let users = db::get_all_users(&session).await;

//     redis::cache_users(&users).await;
//     HttpResponse::Ok().json(json!({ "message":"all users", "success":true, "data":users }))
// }

#[get("/users")]
pub async fn get_users(session: web::Data<Session>) -> impl Responder {
    // Check if the list of users is cached
    if let Some(cached_users) = redis::get_cached_users().await {
        if !cached_users.is_empty() {
            return HttpResponse::Ok().json(json!({ "message": "all users from cache", "success": true, "data": cached_users }));
        }
    }

    // If not cached or cache is empty, query the database
    let users = db::get_all_users(&session).await;

    // Cache the result in Redis
    redis::cache_users(&users).await;

    HttpResponse::Ok().json(json!({ "message": "all users", "success": true, "data": users }))
}

#[get("/get_user/{id}")]
pub async fn get_user(
    id: web::Path<String>,
    session: web::Data<Session>,
) -> impl Responder {
    let user_id = id.into_inner();

    // Check if the user is cached in Redis
    if let Some(cached_user) = redis::get_cached_user(&user_id).await {
        return HttpResponse::Ok().json(json!({ "message": "user from cache", "success": true, "data": cached_user })); // Return the cached user
    }

    // If not cached, query the database
    match db::get_user_by_id(&session, Uuid::parse_str(&user_id).unwrap()).await {
        Ok(Some(user)) => {
            // Cache the user in Redis for future requests
            redis::cache_user(&user).await;
            HttpResponse::Ok().json(user)
        }
        Ok(None) => HttpResponse::NotFound().body(format!("No user found with ID {}", user_id)),
        Err(err) => HttpResponse::InternalServerError().body(format!("Database error: {}", err)),
    }
}

#[post("/add_user")]
pub async fn add_user(session: web::Data<Session>, body: String) -> impl Responder {
    let mut user: User = match from_str(&body) {
        Ok(user) => user,
        Err(err) => return HttpResponse::BadRequest().body(format!("Invalid input: {}", err)),
    };

    match db::get_user_by_email(&session, &user.email).await {
        Ok(Some(_)) => {
            // Email already exists
            return HttpResponse::BadRequest().body(format!("Email {} is already in use", user.email));
        }
        Ok(None) => {
            // Email does not exist, proceed with adding the user
        }
        Err(err) => {
            // Database error
            return HttpResponse::InternalServerError().body(format!("Database error: {}", err));
        }
    }

    user.id = Some(Uuid::new_v4());
    
    db::insert_user(&session, &user).await;

    let users = db::get_all_users(&session).await;

    redis::cache_users(&users).await;

    HttpResponse::Ok().body("User added")
}

#[delete("/delete_user/{id}")]
pub async fn delete_user(session: web::Data<Session>, id: web::Path<Uuid>) -> impl Responder {
    let user_id = id.into_inner();

    match db::get_user_by_id(&session, user_id).await {
        Ok(Some(_)) => {
            // User exists, proceed with deletion
        }
        Ok(None) => {
            // User does not exist
            return HttpResponse::NotFound().body(format!("No user found with ID {}", user_id));
        }
        Err(err) => {
            // Database error
            return HttpResponse::InternalServerError().body(format!("Database error: {}", err));
        }
    }

    match db::delete_user(&session, user_id).await {
        Ok(_) => {
            redis::remove_cached_user(&user_id.to_string()).await;

            let users = db::get_all_users(&session).await;
            redis::cache_users(&users).await;
            HttpResponse::Ok().body(format!("User with ID {} deleted", user_id)) 
        },
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to delete user: {}", err))
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
            return HttpResponse::NotFound().body(format!("No user found with ID {}", user_id));
        }
        Err(err) => {
            // Database error
            return HttpResponse::InternalServerError().body(format!("Database error: {}", err));
        }
    }

    let partial_user: PartialUser = match from_str(&body) {
        Ok(user) => user,
        Err(err) => return HttpResponse::BadRequest().body(format!("Invalid input: {}", err)),
    };

    if let Some(name) = &partial_user.name {
        if name.trim().is_empty() {
            return HttpResponse::BadRequest().body("Name cannot be blank");
        }
    }

    if let Some(email) = &partial_user.email {
        let email_regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
        if !email_regex.is_match(email) {
            return HttpResponse::BadRequest().body("Invalid email format");
        }
    }

    match db::update_user(&session, user_id, partial_user).await {
        Ok(_) => { 
            if let Ok(Some(updated_user)) = db::get_user_by_id(&session, user_id).await {
                redis::cache_user(&updated_user).await;

                let users = db::get_all_users(&session).await;
                redis::cache_users(&users).await;
            }
            HttpResponse::Ok().body(format!("User with ID {} updated", user_id))  
        },
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to update user: {}", err))
        }
    }
}

