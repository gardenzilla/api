use protos;
use protos::user::*;
use serde::{Deserialize, Serialize};
use tonic::transport::Channel;
use warp::reply::Json;
use warp::*;

mod prelude;
use prelude::*;
mod error;
use error::handle_rejection;
mod login;
use login::UserId;
use user_client::UserClient;
use warp::reply;

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

impl From<tonic::Status> for ApiError {
    fn from(error: tonic::Status) -> Self {
        ApiError::internal_error(&format!("service error: {}", error))
    }
}

impl warp::reject::Reject for ApiError {}

#[derive(Serialize)]
struct User<'a> {
    username: &'a str,
    email: &'a str,
    phone: &'a str,
    name: &'a str,
    date_created: &'a str,
    created_by: &'a str,
    // ================
    // Important!
    // ================
    // Only READONLY
    // We do not use it to store any value from form
    // Instead we use direct API call for update customers
    //      ||
    //      \/
    customers: &'a Vec<String>,
}

impl<'a> From<&'a UserObj> for User<'a> {
    fn from(u: &'a UserObj) -> Self {
        User {
            username: &u.id,
            email: &u.email,
            phone: &u.phone,
            name: &u.name,
            date_created: &u.created_at,
            created_by: &u.created_by,
            customers: &u.customers,
        }
    }
}

#[derive(Serialize)]
struct ApiLoginResponse {
    username: String,
    token: String,
}

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct NewPasswordForm {
    new_password: String,
}

async fn new_password(
    userid: UserId,
    mut client: UserClient<Channel>,
    new_password_form: NewPasswordForm,
) -> ApiResult {
    client
        .set_new_password(NewPasswordRequest {
            userid: userid.into(),
            new_password: new_password_form.new_password,
        })
        .await
        .map_err(|e| ApiError::from(e))?;
    Ok(reply::json(&()))
}

async fn reset_password(userid: UserId, mut client: UserClient<Channel>) -> ApiResult {
    let _ = client
        .reset_password(ReserPasswordRequest {
            userid: userid.into(),
        })
        .await
        .map_err(|e| ApiError::from(e))?;
    Ok(reply::json(&()))
}

async fn login(mut client: UserClient<Channel>, login_form: LoginForm) -> ApiResult {
    // 1. First send username + password to user service
    // 2. Gets back the result true / false
    // 3. Create token
    // 4. Send back the token
    let res = client
        .validate_login(LoginRequest {
            username: login_form.username.clone(),
            password: login_form.password,
        })
        .await
        .unwrap()
        .into_inner();
    let uid = match res.is_valid {
        true => login_form.username,
        false => return Err(ApiError::bad_request("Hibás belépési adatok").into()),
    };
    let token = login::create_token(&uid).map_err(|err| ApiError::from(err))?;
    Ok(reply::json(&ApiLoginResponse {
        token: token,
        username: res.name,
    }))
}

async fn get_users(_: UserId, mut client: UserClient<Channel>) -> ApiResult {
    let all = client.get_all(()).await.unwrap().into_inner();
    let v = all.users.iter().map(|u| u.into()).collect::<Vec<User>>();
    Ok(warp::reply::json(&v))
}

async fn get_user_by_id(userid: UserId, mut client: UserClient<Channel>) -> ApiResult {
    let user = client
        .get_by_id(GetByIdRequest {
            userid: userid.into(),
        })
        .await
        .map_err(|e| ApiError::from(e))?
        .into_inner();
    if let Some(user) = user.user {
        let _user: User = (&user).into();
        return Ok(reply::json(&_user));
    }
    Err(ApiError::not_found().into())
}

fn auth() -> impl Filter<Extract = (UserId,), Error = Rejection> + Copy {
    warp::header::optional::<String>("Token").and_then(|n: Option<String>| async move {
        if let Some(token) = n {
            Ok(login::verify_token(&token).map_err(|err| ApiError::from(err))?)
        } else {
            Err(reject::custom(ApiRejection::new(
                warp::http::StatusCode::UNAUTHORIZED,
                "".into(),
            )))
        }
    })
}

fn with_db<T>(db: T) -> impl Filter<Extract = (T,), Error = std::convert::Infallible> + Clone
where
    T: Clone + Send,
{
    warp::any().map(move || db.clone())
}

//
#[tokio::main]
async fn main() {
    let client = UserClient::connect("http://[::1]:50051").await.unwrap();
    let welcome = warp::path::end().map(|| format!("Welcome to Gardenzilla API"));
    let users = warp::path!("hello")
        .and(auth())
        .and(with_db(client.clone()))
        .and_then(get_users);

    let login = warp::path!("login")
        .and(warp::post())
        .and(with_db(client.clone()))
        .and(warp::body::json())
        .and_then(login);

    let profile_new_password = warp::path!("new_password")
        .and(warp::get())
        .and(auth())
        .and(with_db(client.clone()))
        .and(warp::body::json())
        .and_then(new_password);
    let profile_get = warp::path::end()
        .and(warp::get())
        .and(auth())
        .and(with_db(client.clone()))
        .and_then(get_user_by_id);

    let profile = warp::path!("profile" / ..).and(profile_new_password.or(profile_get));

    let routes = warp::any().and(welcome.or(login).or(users).or(profile));
    warp::serve(warp::any().and(routes).recover(handle_rejection))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
