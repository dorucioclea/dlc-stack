use displaydoc::Display;
use thiserror::Error;
use dlc_clients::ApiError;

pub type Result<T> = std::result::Result<T, OracleError>;

#[derive(Debug, Display, Error)]
pub enum OracleError {
    /// nonpositive announcement time offset: {0}; announcement must happen before attestation
    InvalidAnnouncementTimeError(time::Duration),

    /// database error: {0}
    DatabaseError(#[from] sled::Error),

    /// storage api error: {0}
    StorageApiError(#[from] ApiError),

    /// event not found in redis
    EventNotFoundError,
}
