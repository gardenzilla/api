use crate::login::LoginError;
use warp::reply::Json;

pub type ApiResult = Result<Json, warp::reject::Rejection>;

#[derive(Debug)]
pub enum ApiError {
  NotFound,
  BadRequest(String),
  InternalError(String),
  Unauthorized,
}

impl ApiError {
  pub fn not_found() -> Self {
    ApiError::NotFound
  }
  pub fn bad_request(msg: &str) -> Self {
    ApiError::BadRequest(msg.into())
  }
  pub fn internal_error(msg: &str) -> Self {
    ApiError::InternalError(msg.into())
  }
  pub fn unauthorized() -> Self {
    ApiError::Unauthorized
  }
}

impl From<LoginError> for ApiError {
  fn from(err: LoginError) -> Self {
    match err {
      LoginError::WrongToken => ApiError::bad_request("Érvénytelen token"),
      LoginError::InternalError => ApiError::internal_error("Token dekódolási hiba"),
    }
  }
}
