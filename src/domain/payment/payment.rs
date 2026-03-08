use serde::{Serialize, Deserialize};
use mongodb::bson::{oid::ObjectId, DateTime};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PaymentGateway {
    Razorpay,
    Stripe,
    Cashfree,
    Paytm,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PaymentStatus {
    Pending,
    Success,
    Failed,
    Refunded,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Payment {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,

    pub order_id: ObjectId,
    pub user_id: ObjectId,

    pub gateway: PaymentGateway,
    pub status: PaymentStatus,

    pub amount: f64,
    pub currency: String,

    pub transaction_id: Option<String>,
    pub gateway_signature: Option<String>,

    pub created_at: DateTime,
    pub updated_at: DateTime,
}
