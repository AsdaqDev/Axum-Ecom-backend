use async_graphql::InputObject;

#[derive(InputObject)]
pub struct order {
    pub id: String,

    pub user_id: String,

    pub items: String,

    pub shipping_address: String,

    // order pricing
    pub subtotal: f64,
    pub discount: f64,
    pub tax: f64,
    pub delivery_fee: f64,
    pub total_amount: f64,

    // payment info
    pub payment_method: String,
    pub payment_status: String,
    pub transaction_id: String,

    // order status
    pub order_status: String,

}