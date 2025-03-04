use actix_web::{web, App, HttpServer};
mod handlers;
mod models;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize DB pool (this could use Diesel or SQLx)
    let pool = db::init_pool();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(
                web::scope("/api")
                    .service(handlers::get_vendors)
                    .service(handlers::create_order),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
