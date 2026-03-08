use serde::{Serialize, Deserialize};
use mongodb::bson::{oid::ObjectId, DateTime};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Inventory {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,

    pub product_id: ObjectId,
    pub stock: i32,
    pub reserved: i32,      // For orders placed but not confirmed
    pub warehouse: String,

    pub updated_at: DateTime,
}
