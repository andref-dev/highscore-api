

#[derive(Debug)]
pub enum AppError {
    DuplicateEntryError,
    MongoDbError,
    NotFound,
}

impl From<mongodb::error::Error> for AppError {
    fn from(_mongo_eror: mongodb::error::Error) -> Self {
        Self::MongoDbError
    }
}