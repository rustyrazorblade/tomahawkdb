use super::database::Database;

pub struct TomahawkServer {
    database: Database,
}

impl TomahawkServer {
    pub fn new() -> TomahawkServer {
        TomahawkServer{ database:Database::new()}
    }
}