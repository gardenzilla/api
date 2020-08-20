// #![deny(warnings)]
use hyper;
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
}

impl warp::Reply for ApiError {
    fn into_response(self) -> warp::reply::Response {
        match self {
            ApiError::NotFound => warp::http::Response::builder()
                .status(404)
                .body(hyper::body::Body::empty())
                .unwrap(),
            ApiError::BadRequest(msg) => warp::http::Response::builder()
                .status(400)
                .body(msg.into())
                .unwrap(),
            ApiError::InternalError(msg) => warp::http::Response::builder()
                .status(500)
                .body(msg.into())
                .unwrap(),
        }
    }
}

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
        }
    }
}

impl warp::reject::Reject for ApiError {}

#[derive(Serialize)]
struct User {
    username: String,
    email: String,
    phone: String,
}

impl From<&UserObj> for User {
    fn from(u: &UserObj) -> Self {
        User {
            username: u.id.to_string(),
            email: u.email.to_string(),
            phone: u.phone.to_string(),
        }
    }
}

async fn get_users(mut client: UserClient<Channel>) -> ApiResult {
    let all = client.get_all(()).await.unwrap().into_inner();
    let v = all.users.iter().map(|u| u.into()).collect::<Vec<User>>();
    Ok(warp::reply::json(&v))
}

#[tokio::main]
async fn main() {
    let client = UserClient::connect("http://[::1]:50051").await.unwrap();

    let root = warp::path::end().map(|| warp::redirect(warp::http::Uri::from_static("/api")));
    let api = warp::path!("api" / ..);
    let welcome = api
        .and(warp::path::end())
        .map(|| "Welcome to Gardenzilla API");
    let users = api
        .and(warp::path!("hello"))
        .and(warp::any().map(move || client.clone()))
        .and_then(get_users);

    let routes = warp::any().and(root.or(welcome).or(users));

    warp::serve(warp::any().and(routes).recover(handle_rejection))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
