use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::models::{Vendor, Order};
use crate::db::DbPool;

// Query parameters for filtering vendors
#[derive(Deserialize)]
pub struct VendorQuery {
    pub location: Option<String>,
    pub sort_by: Option<String>,
}

#[get("/vendors")]
pub async fn get_vendors(
    pool: web::Data<DbPool>,
    query: web::Query<VendorQuery>,
) -> impl Responder {
    // Here, implement the logic to query vendors from the database
    // For now, we return a dummy response.
    let vendors: Vec<Vendor> = vec![]; // Replace with DB query results.
    HttpResponse::Ok().json(vendors)
}

#[derive(Deserialize)]
pub struct NewOrder {
    pub vendor_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}

#[post("/orders")]
pub async fn create_order(
    pool: web::Data<DbPool>,
    order: web::Json<NewOrder>,
) -> impl Responder {
    // Insert the order into the database and return confirmation
    // This is just a stub for illustration.
    let created_order = Order {
        id: 1,
        vendor_id: order.vendor_id,
        product_id: order.product_id,
        quantity: order.quantity,
    };
    HttpResponse::Created().json(created_order)
}
