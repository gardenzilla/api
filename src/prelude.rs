use crate::login::LoginError;
use crate::ApiError;

impl From<LoginError> for ApiError {
    fn from(err: LoginError) -> Self {
        match err {
            LoginError::WrongToken => ApiError::bad_request("Érvénytelen token"),
            LoginError::InternalError => ApiError::internal_error("Token dekódolási hiba"),
        }
    }
}
