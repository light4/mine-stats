use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// Our app's top level error type.
enum AppError {
    /// Something went wrong when calling the user repo.
    UserRepo(UserRepoError),
}

/// Errors that can happen when using the user repo.
#[derive(Debug)]
enum UserRepoError {
    #[allow(dead_code)]
    NotFound,
    #[allow(dead_code)]
    InvalidUsername,
}

/// This makes it possible to use `?` to automatically convert a `UserRepoError`
/// into an `AppError`.
impl From<UserRepoError> for AppError {
    fn from(inner: UserRepoError) -> Self {
        AppError::UserRepo(inner)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::UserRepo(UserRepoError::NotFound) => {
                (StatusCode::NOT_FOUND, "User not found")
            }
            AppError::UserRepo(UserRepoError::InvalidUsername) => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Invalid username")
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
