use async_graphql::InputObject;

#[derive(InputObject)]
pub struct category {
    pub id: String,
    pub name: String,
    pub slug: String,
}