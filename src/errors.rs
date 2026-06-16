use crate::models::RecipeId;

#[cfg(feature = "api")]
use axum::{http::StatusCode, response::IntoResponse};

/// The result type that we use for our errors
pub type Result<T> = core::result::Result<T, Error>;

/// The errors that we could encounter.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    #[cfg(feature = "db")]
    /// Database error
    #[error("Database error encountered: {0}")]
    DB(#[from] diesel::result::Error),

    #[cfg(feature = "db")]
    /// Database connection error.
    #[error("Error connecting to the database: {0}")]
    DBConn(#[from] diesel::result::ConnectionError),

    /// User failed to authenticate properly, no authentication token.
    #[error("Failed to authenticate: no authentication token")]
    AuthNoToken,

    /// User failed to authenticate properly, device ID mismatch
    #[error("Failed to authenticate: device id {id} is not associated with a user")]
    AuthDeviceId { id: String },

    /// User tried editing a recipe that is notowned by them.
    #[error("Failed to edit recipe: Recipe {0} is not owned by you")]
    UserEditingUnownedRecipe(RecipeId),

    /// User tried editing a recipe that does not exist.
    #[error("Failed to edit recipe: Recipe {0} is not owned by you")]
    UserEditingNonExistantRecipe(RecipeId),
}

#[cfg(feature = "api")]
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let err_str = format!("{self}");
        let err_code = match self {
            Error::DB(_) | Error::DBConn(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::AuthNoToken | Error::AuthDeviceId { .. } => StatusCode::UNAUTHORIZED,
            Error::UserEditingUnownedRecipe(_) => StatusCode::FORBIDDEN,
            Error::UserEditingNonExistantRecipe(_) => StatusCode::BAD_REQUEST,
        };

        (err_code, err_str).into_response()
    }
}
