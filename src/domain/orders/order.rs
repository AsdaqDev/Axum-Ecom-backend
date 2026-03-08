use serde::{Serialize, Deserialize};
use mongodb::bson::{oid::ObjectId, DateTime};

/// Payment Methods
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PaymentMethod {
    COD,
    UPI,
    Card,
    NetBanking,
}

/// Payment Status
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PaymentStatus {
    Pending,
    Paid,
    Failed,
    Refunded,
}

/// Order Status
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OrderStatus {
    Placed,
    Confirmed,
    Packed,
    Shipped,
    OutForDelivery,
    Delivered,
    Cancelled,
    Returned,
}

/// Order Item Structure
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderItem {
    #[serde(rename = "product_id")]
    pub product_id: ObjectId,

    pub title: String,
    pub quantity: i16,
    pub price: f64,
    pub mrp: f64,
}

/// Address model
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Address {
    pub full_name: String,
    pub phone: String,
    pub pincode: String,
    pub state: String,
    pub city: String,
    pub address_line: String,
    pub landmark: Option<String>,
}

/// Order model
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,

    // user who placed order
    pub user_id: ObjectId,

    // list of items
    pub items: Vec<OrderItem>,

    // delivery address
    pub shipping_address: Address,

    // order pricing
    pub subtotal: f64,
    pub discount: f64,
    pub tax: f64,
    pub delivery_fee: f64,
    pub total_amount: f64,

    // payment info
    pub payment_method: PaymentMethod,
    pub payment_status: PaymentStatus,
    pub transaction_id: Option<String>,

    // order status
    pub order_status: OrderStatus,

    pub created_at: DateTime,
    pub updated_at: DateTime,
}
