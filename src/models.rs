use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Vendor {
    pub id: i32,
    pub name: String,
    pub location: String,
    pub price_range: String,
    pub reviews: Vec<Review>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Review {
    pub id: i32,
    pub rating: u8,
    pub comment: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: i32,
    pub vendor_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}
