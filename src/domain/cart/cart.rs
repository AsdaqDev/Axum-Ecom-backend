use serde::{Serialize, Deserialize};
use mongodb::bson::{oid::ObjectId, DateTime};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CartItem {
    pub product_id: ObjectId,
    pub quantity: i16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cart {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,

    pub user_id: ObjectId,
    pub items: Vec<CartItem>,

    pub updated_at: DateTime,
}
