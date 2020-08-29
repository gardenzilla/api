use crate::prelude::*;
use protos;
use protos::user::user_client::UserClient;
use protos::user::*;
use serde::{Deserialize, Serialize};
use tonic::transport::Channel;
use warp::reply;

#[derive(Serialize)]
pub struct ApiLoginResponse {
    pub username: String,
    pub token: String,
}

#[derive(Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct FormResetPassword {
    email: Option<String>,
}

pub async fn login(mut client: UserClient<Channel>, login_form: LoginForm) -> ApiResult {
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
        .map_err(|e| ApiError::from(e))?
        .into_inner();
    let uid = match res.is_valid {
        true => login_form.username,
        false => return Err(ApiError::bad_request("Hibás belépési adatok").into()),
    };
    let token = crate::login::create_token(&uid).map_err(|err| ApiError::from(err))?;
    Ok(reply::json(&ApiLoginResponse {
        token: token,
        username: res.name,
    }))
}

pub async fn reset_password(mut client: UserClient<Channel>, form: FormResetPassword) -> ApiResult {
    let _ = client
        .reset_password(ResetPasswordRequest {
            userid: form.email.unwrap_or_default(),
        })
        .await
        .map_err(|e| ApiError::from(e))?;
    Ok(reply::json(&()))
}
