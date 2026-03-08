use async_graphql::InputObject;

#[derive(InputObject)]
pub struct cart {
    pub id: String,

    pub user_id: String,
    pub items: String,


}