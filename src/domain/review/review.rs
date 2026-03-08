use serde::{Serialize, Deserialize};
use mongodb::bson::{oid::ObjectId, DateTime};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Review {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,

    pub product_id: ObjectId,
    pub user_id: ObjectId,

    pub rating: i8,          // 1 to 5
    pub comment: Option<String>,
    pub images: Vec<String>, // optional review images

    pub created_at: DateTime,
}
