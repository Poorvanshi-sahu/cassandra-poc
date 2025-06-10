mod db;
mod handlers;
mod models;
mod redis;

use actix_web::{App, HttpServer, web};
use db::{connect_db, init_db};
use handlers::{add_user, get_user, delete_user, get_users, update_user};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let session = connect_db().await;
    init_db(&session).await;
    let session_data = web::Data::new(session);

    HttpServer::new(move || {
        App::new()
            .app_data(session_data.clone())
            .service(get_user)
            .service(get_users)
            .service(add_user)
            .service(delete_user)
            .service(update_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
