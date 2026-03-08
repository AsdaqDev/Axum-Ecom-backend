use serde::{Serialize,Deserialize};
use mongodb::bson::{oid:ObjectId,DateTime};

#[derive(Serialize,Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id : Option<ObjectId>,
    
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone_number:i64,

    pub created_at:DateTime,
}