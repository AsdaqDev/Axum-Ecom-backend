use async_graphql::InputObject;

#[derive(InputObject)]
pub struct inventory {
    pub product_id: String,
    pub stock: i32,
    pub reserved: i32,      // For orders placed but not confirmed
    pub warehouse: String,
}