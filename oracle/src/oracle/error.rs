use displaydoc::Display;
use thiserror::Error;
use redis::RedisError;

pub type Result<T> = std::result::Result<T, OracleError>;

#[derive(Debug, Display, Error)]
pub enum OracleError {
    /// nonpositive announcement time offset: {0}; announcement must happen before attestation
    InvalidAnnouncementTimeError(time::Duration),

    /// database error: {0}
    DatabaseError(#[from] sled::Error),

    /// redis error: {0}
    RedisError(#[from] RedisError),

    /// event not found in redis
    EventNotFoundError,
}
