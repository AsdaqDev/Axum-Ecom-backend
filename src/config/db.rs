use Mongodb::{Client, Database};

#[derive(Clone)]
pub struct Mongodb {
    pub db: Database,
}

