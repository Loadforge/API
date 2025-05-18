mod config;
mod routes; 
mod handlers;
mod services;
mod models;
mod repositories;
use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use config::db;
use routes::{health_route, project_route}; 


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_pool = db::connect_db().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .configure(health_route::config) 
            
            .configure(project_route::config) 
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
