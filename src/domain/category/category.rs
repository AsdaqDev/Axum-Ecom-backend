use serde::{Serialize, Deserialize};
use mongodb::bson::{oid::ObjectId, DateTime};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Category {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,

    pub name: String,
    pub slug: String,
    pub parent_id: Option<ObjectId>, // nested categories

    pub created_at: DateTime,
    pub updated_at: DateTime,
}
