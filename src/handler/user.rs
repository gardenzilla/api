use crate::{prelude::*, services::Services};
use gzlib::proto::{email::EmailRequest, user::*};
use serde::{Deserialize, Serialize};
use warp::reply;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewPasswordForm {
  password1: String,
  password2: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
  pub uid: u32,
  pub username: String,
  pub name: String,
  pub email: String,
  pub phone: String,
  pub created_at: String, // RFC3339
  pub created_by: u32,    // UID
}

impl From<UserObj> for User {
  fn from(u: UserObj) -> Self {
    User {
      uid: u.uid,
      username: u.username.to_string(),
      name: u.name.to_string(),
      email: u.email.to_string(),
      phone: u.phone.to_string(),
      created_at: u.created_at,
      created_by: u.created_by,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserNew {
  username: String,
  name: String,
  email: String,
  phone: String,
}

pub async fn new_password(
  userid: u32,
  mut services: Services,
  new_password_form: NewPasswordForm,
) -> ApiResult {
  if &new_password_form.password1 != &new_password_form.password2 {
    return Err(ApiError::BadRequest("A megadott jelszavak nem egyeznek meg!".into()).into());
  }

  services
    .user
    .set_new_password(NewPasswordRequest {
      uid: userid,
      new_password: new_password_form.password1,
    })
    .await
    .map_err(|e| ApiError::from(e))?;

  Ok(reply::json(&()))
}

pub async fn update_profile(_: u32, mut services: Services, u: User) -> ApiResult {
  let res: User = services
    .user
    .update_by_id(UserObj {
      uid: u.uid,
      username: u.username,
      name: u.name,
      email: u.email,
      phone: u.phone,
      created_by: u.created_by,
      created_at: u.created_at,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(warp::reply::json(&res))
}

pub async fn get_all(_: u32, mut services: Services) -> ApiResult {
  let mut all = services
    .user
    .get_all(())
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();

  let mut result: Vec<User> = Vec::new();
  while let Some(user) = all.message().await.map_err(|e| ApiError::from(e))? {
    result.push(user.into());
  }
  Ok(warp::reply::json(&result))
}

pub async fn get_profile(userid: u32, mut services: Services) -> ApiResult {
  let user: User = services
    .user
    .get_by_id(GetByIdRequest { userid })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&user))
}

pub async fn get_by_id(userid: u32, _uid: u32, mut services: Services) -> ApiResult {
  let user: User = services
    .user
    .get_by_id(GetByIdRequest { userid })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&user))
}

pub async fn create_new(userid: u32, mut services: Services, uo: UserNew) -> ApiResult {
  let user: User = services
    .user
    .create_user(NewUserObj {
      username: uo.username,
      name: uo.name,
      email: uo.email,
      phone: uo.phone,
      created_by: userid,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&user))
}
