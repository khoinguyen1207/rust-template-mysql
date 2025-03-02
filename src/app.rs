use actix_web::{middleware, web, App, HttpServer};
use actix_cors::Cors;
use crate::configs::configs;

use crate::handlers::users;
use crate::repositories::db::DB_POOL;
use crate::repositories::db::migrate_db;
use crate::repositories::users::UserDao;
use crate::services::users::UserSrv;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    users::route(cfg);
}

pub async fn create_app() -> std::io::Result<()> {
    // Init DB
    let db_pool = DB_POOL.clone();

    // migrate db
    if let Err(e) = migrate_db().await {
        println!("\nMigrate db failed: {}", e);
    }

    let user_dao = UserDao::new(db_pool.clone());

    let user_service = web::Data::new(UserSrv::new(user_dao.clone()));

    let listen_address: String = configs::get("listen_address");
    println!("\nListening and serving HTTP on {}", listen_address);

    HttpServer::new(move || {
        let cors: Cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .max_age(3600);

        App::new()
            .app_data(user_service.clone())
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .configure(init_routes)
    })
    .bind(listen_address)?
    .run()
    .await
}
