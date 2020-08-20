// #![deny(warnings)]
use protos;
use protos::user::{user_client::UserClient, UserObj};
use serde::Serialize;
use tonic::transport::Channel;
use warp::reply::Json;
use warp::*;

mod error;
use error::handle_rejection;

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
struct ApiRejection {
    code: warp::http::StatusCode,
    message: String,
}

impl warp::reject::Reject for ApiRejection {}

impl ApiRejection {
    fn new(code: warp::http::StatusCode, message: String) -> Self {
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

impl warp::reject::Reject for ApiError {}

#[derive(Serialize)]
struct User<'a> {
    username: &'a str,
    email: &'a str,
    phone: &'a str,
}

impl<'a> From<&'a UserObj> for User<'a> {
    fn from(u: &'a UserObj) -> Self {
        User {
            username: &u.id,
            email: &u.email,
            phone: &u.phone,
        }
    }
}

async fn get_users(_: String, mut client: UserClient<Channel>) -> ApiResult {
    let all = client.get_all(()).await.unwrap().into_inner();
    let v = all.users.iter().map(|u| u.into()).collect::<Vec<User>>();
    Ok(warp::reply::json(&v))
}

fn auth() -> impl Filter<Extract = (String,), Error = Rejection> + Copy {
    warp::header::optional::<String>("Token").and_then(|n: Option<String>| async move {
        if let Some(token) = n {
            Ok(token)
        } else {
            Err(reject::custom(ApiRejection::new(
                warp::http::StatusCode::UNAUTHORIZED,
                "".into(),
            )))
        }
    })
}

#[tokio::main]
async fn main() {
    // let auth = warp::header::optional::<String>("Token");
    let client = UserClient::connect("http://[::1]:50051").await.unwrap();
    let welcome = warp::path::end().map(|| format!("Welcome to Gardenzilla API"));
    let users = warp::path!("hello")
        .and(auth())
        .and(warp::any().map(move || client.clone()))
        .and_then(get_users);

    let routes = warp::any().and(welcome.or(users));
    warp::serve(warp::any().and(routes).recover(handle_rejection))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
