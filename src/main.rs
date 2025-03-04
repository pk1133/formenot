use actix_web::{web, App, HttpServer};
mod handlers;
mod models;
mod db;
println!("Hello, world!");
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello World");
    // Initialize DB pool (this could use Diesel or SQLx)
    // let pool = db::init_pool();

    // HttpServer::new(move || {
    //     App::new()
    //         .data(pool.clone())
    //         .service(
    //             web::scope("/api")
    //                 .service(handlers::get_vendors)
    //                 .service(handlers::create_order),
    //         )
    // })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
