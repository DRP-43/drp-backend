use crate::db::models::UserId;

/// The result type that we use for our errors
pub type Result<T> = core::result::Result<T, Error>;

/// The errors that we could encounter.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    /// Database error
    #[error("Database error encountered: {0}")]
    DB(#[from] diesel::result::Error),

    /// Database connection error.
    #[error("Error connecting to the database: {0}")]
    DBConn(#[from] diesel::result::ConnectionError),

    /// User failed to authenticate properly
    #[error("User {id} failed to authenticate.")]
    Auth { id: UserId },
}
