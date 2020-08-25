use crate::prelude::ApiError;
use serde::Serialize;
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::Reply;

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

// impl warp::Reply for ApiError {
//     fn into_response(self) -> warp::reply::Response {
//         match self {
//             ApiError::NotFound => warp::http::Response::builder()
//                 .status(404)
//                 .body(hyper::body::Body::empty())
//                 .unwrap(),
//             ApiError::BadRequest(msg) => warp::http::Response::builder()
//                 .status(400)
//                 .body(msg.into())
//                 .unwrap(),
//             ApiError::InternalError(msg) => warp::http::Response::builder()
//                 .status(500)
//                 .body(msg.into())
//                 .unwrap(),
//             ApiError::Unauthorized => warp::http::Response::builder()
//                 .status(401)
//                 .body("".into())
//                 .unwrap(),
//         }
//     }
// }

#[derive(Debug)]
pub struct ApiRejection {
    code: warp::http::StatusCode,
    message: String,
}

impl warp::reject::Reject for ApiRejection {}

impl ApiRejection {
    pub(crate) fn new(code: warp::http::StatusCode, message: String) -> Self {
        ApiRejection { code, message }
    }
}

impl From<ApiError> for warp::reject::Rejection {
    fn from(error: ApiError) -> Self {
        match error {
            ApiError::NotFound => warp::reject::custom(ApiRejection::new(
                warp::http::StatusCode::NOT_FOUND,
                "".to_string(),
            )),
            ApiError::BadRequest(msg) => {
                warp::reject::custom(ApiRejection::new(warp::http::StatusCode::BAD_REQUEST, msg))
            }
            ApiError::InternalError(msg) => warp::reject::custom(ApiRejection::new(
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                msg,
            )),
            ApiError::Unauthorized => warp::reject::custom(ApiRejection::new(
                warp::http::StatusCode::UNAUTHORIZED,
                "".to_string(),
            )),
        }
    }
}

impl From<tonic::Status> for ApiError {
    fn from(error: tonic::Status) -> Self {
        match error.code() {
            tonic::Code::Internal => ApiError::internal_error(error.message()),
            tonic::Code::NotFound => ApiError::not_found(),
            tonic::Code::InvalidArgument => ApiError::bad_request(error.message()),
            _ => ApiError::internal_error(&format!(
                "Unhandled error! Code is {}, message is {}",
                error.code(),
                error.message()
            )),
        }
    }
}

impl warp::reject::Reject for ApiError {}

pub async fn handle_rejection(err: warp::Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if let Some(err) = err.find::<ApiRejection>() {
        code = err.code;
        message = err.message.as_str();
    } else if let Some(_e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        // This error happens if the body could not be deserialized correctly
        // We can use the cause to analyze the error and customize the error message
        message = "Body deserialisation error!";
        code = StatusCode::BAD_REQUEST;
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        // We can handle a specific error, here METHOD_NOT_ALLOWED,
        // and render it however we want
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED";
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
