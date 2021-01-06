use crate::{prelude::*, services::Services};
use gzlib::proto::{email::EmailRequest, user::*};
use serde::{Deserialize, Serialize};
use warp::reply;

#[derive(Serialize)]
pub struct ApiLoginResponse {
  pub uid: u32,
  pub token: String,
}

#[derive(Deserialize)]
pub struct LoginForm {
  pub username: String,
  pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct FormResetPassword {
  email: String,
}

pub async fn login(mut services: Services, login_form: LoginForm) -> ApiResult {
  // 1. First send username + password to user service
  // 2. Gets back the result true / false
  // 3. Create token
  // 4. Send back the token
  let res = services
    .user
    .login(LoginRequest {
      username: login_form.username,
      password: login_form.password,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();

  let token = crate::login::create_token(res.uid).map_err(|err| ApiError::from(err))?;
  Ok(reply::json(&ApiLoginResponse {
    token: token,
    uid: res.uid,
  }))
}

pub async fn reset_password(mut services: Services, form: FormResetPassword) -> ApiResult {
  let res = services
    .user
    .reset_password(ResetPasswordRequest {
      email: form.email.clone(),
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();

  // Send email
  services
    .email
    .send_email(EmailRequest {
      to: res.email,
      subject: "Elfelejtett jelszó".into(),
      body: format!("A Gardenzilla fiókodban töröltük a régi jelszavadat,\n és új jelszót állítottunk be.\n\n Az új jelszavad: {}", res.new_password),
    })
    .await
    .map_err(|_| ApiError::internal_error("Hiba az email elküldésekor"))?;

  Ok(reply::json(&()))
}
