use serde::{Serialize, Deserialize};
use mongodb::bson::{oid::ObjectId, DateTime};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Wishlist {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,

    pub user_id: ObjectId,
    pub products: Vec<ObjectId>,

    pub updated_at: DateTime,
}
