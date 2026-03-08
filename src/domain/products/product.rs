use serde::{Serialize,Deserialize};
use mongodb::bson::{oid:ObjectId,DateTime};

#[derive(Serialize,Deserialize, Debug, Clone)]
pub enum StockStatus {
    OutOfStock,
    InStock,
    LowStock,

}


#[derive(Serialize,Deserialize, Debug, Clone)]
pub struct Product {
    pub title: String,

    #[serde(rename = "_id")]
    pub id : Option<ObjectId>,

    pub description: String,
    pub sku: String,
    pub bullet_point: Vec<String>,
    
    pub weight: String,
    pub dimension: String,
    
    pub gender: String,
    pub quantity: i16,
    
    pub price: f64,
    pub mrp: f64,
    pub stock: StockStatus,
    pub hsn: i32,

    pub created_at: DateTime
}