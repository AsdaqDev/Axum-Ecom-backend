use crate::infrastructure::mongo::client::connect_db;
use crate::core::state::Appstate;

pub async fn bootstrap() -> Appstate {
    let db = connect_db().await
    Appstate {db}
}