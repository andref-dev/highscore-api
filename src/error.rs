use std::{fmt::Display, sync::PoisonError};

use actix_web::{ResponseError, HttpResponse};

#[derive(Debug)]
pub enum AppError {
    DuplicateEntryError,
    MongoDbError,
    NotFound,
    PoisonError,
    MissingApiKey,
    InvalidApiKey,
}

impl From<mongodb::error::Error> for AppError {
    fn from(_mongo_error: mongodb::error::Error) -> Self {
        Self::MongoDbError
    }
}

impl<T> From<PoisonError<T>> for AppError {
    fn from(_poison_error: PoisonError<T>) -> Self {
        Self::PoisonError
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::DuplicateEntryError => write!(f, "Duplicate Entry Error"),
            AppError::MongoDbError => write!(f, "MongoDb Error"),
            AppError::NotFound => write!(f, "Not Found"),
            AppError::PoisonError => write!(f, "Poison Error"),
            AppError::MissingApiKey => write!(f, "Missing API KEY"),
            AppError::InvalidApiKey => write!(f, "Invalid API KEY"),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse  {
        match self {
            AppError::NotFound => HttpResponse::NotFound().into(),
            AppError::DuplicateEntryError => HttpResponse::Conflict().into(),
            AppError::MongoDbError => HttpResponse::InternalServerError().into(),
            AppError::PoisonError => HttpResponse::InternalServerError().into(),
            AppError::MissingApiKey => HttpResponse::Unauthorized().into(),
            AppError::InvalidApiKey => HttpResponse::Unauthorized().into(),
        }
    }   
}