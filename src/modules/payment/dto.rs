use async_graphql::InputObject;

#[derive(InputObject)]
pub struct payment {
     pub id: String,

    pub order_id: String,
    pub user_id: String,

    pub gateway: String,
    pub status: String,

    pub amount: f64,
    pub currency: String,

    pub transaction_id: String,
    pub gateway_signature: String,
}