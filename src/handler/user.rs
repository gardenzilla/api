use crate::prelude::*;
use crate::UserId;
use protos;
use protos::user::user_client::UserClient;
use protos::user::*;
use serde::{Deserialize, Serialize};
use tonic::transport::Channel;
use warp::reply;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewPasswordForm {
  password1: Option<String>,
  password2: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct User {
  pub uid: u32,
  pub username: String,
  pub email: String,
  pub phone: String,
  pub name: String,
  pub date_created: i64,
  pub created_by: u32,
  // ================
  // Important!
  // ================
  // Only READONLY
  // We do not use it to store any value from form
  // Instead we use direct API call for update customers
  //      ||
  //      \/
  pub customers: Vec<u32>,
}

impl From<&UserObj> for User {
  fn from(u: &UserObj) -> Self {
    User {
      uid: u.id,
      username: u.alias.to_string(),
      email: u.email.to_string(),
      phone: u.phone.to_string(),
      name: u.name.to_string(),
      date_created: u.created_at,
      created_by: u.created_by,
      customers: u.customers.to_owned(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserNew {
  username: String,
  email: String,
  name: String,
  phone: String,
}

impl UserNew {
  fn to_request(self, created_by: UserId) -> CreateNewRequest {
    CreateNewRequest {
      alias: self.username,
      email: self.email,
      name: self.name,
      phone: self.phone,
      created_by: *created_by,
    }
  }
}

pub async fn new_password(
  userid: UserId,
  mut client: UserClient<Channel>,
  new_password_form: NewPasswordForm,
) -> ApiResult {
  let p1 = match new_password_form.password1 {
    Some(_pwd) => _pwd,
    None => return Err(ApiError::bad_request("jelszó1 kötelező").into()),
  };
  let p2 = match new_password_form.password2 {
    Some(_pwd) => _pwd,
    None => return Err(ApiError::bad_request("jelszó2 kötelező").into()),
  };
  if &p1 != &p2 {
    return Err(ApiError::BadRequest("A megadott jelszavak nem egyeznek meg!".into()).into());
  }
  client
    .set_new_password(NewPasswordRequest {
      userid: *userid,
      new_password: p1,
    })
    .await
    .map_err(|e| ApiError::from(e))?;
  Ok(reply::json(&()))
}

pub async fn update_profile(
  userid: UserId,
  mut client: UserClient<Channel>,
  profile: User,
) -> ApiResult {
  let res = client
    .update_by_id(UpdateByIdRequest {
      user: Some(UserObj {
        id: *userid,
        alias: profile.username,
        name: profile.name,
        email: profile.email,
        phone: profile.phone,
        customers: profile.customers,
        created_by: profile.created_by,
        created_at: profile.date_created,
      }),
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();
  let user: User = (&res.user.unwrap_or_default()).into();
  Ok(warp::reply::json(&user))
}

pub async fn get_all(_: UserId, mut client: UserClient<Channel>) -> ApiResult {
  let mut all = client
    .get_all(())
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();

  let mut result: Vec<User> = Vec::new();
  while let Some(user) = all.message().await.map_err(|e| ApiError::from(e))? {
    result.push((&user).into());
  }
  Ok(warp::reply::json(&result))
}

pub async fn get_profile(userid: UserId, mut client: UserClient<Channel>) -> ApiResult {
  let user = client
    .get_by_id(GetByIdRequest { userid: *userid })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();
  if let Some(user) = user.user {
    let _user: User = (&user).into();
    return Ok(reply::json(&_user));
  }
  Err(ApiError::not_found().into())
}

pub async fn get_by_id(id: u32, _userid: UserId, mut client: UserClient<Channel>) -> ApiResult {
  // Get user object
  let user = client
    .get_by_id(GetByIdRequest { userid: id })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();

  // Create response if user exist
  if let Some(user) = user.user {
    let _user: User = (&user).into();
    return Ok(reply::json(&_user));
  }
  Err(ApiError::not_found().into())
}

pub async fn create_new(
  userid: UserId,
  mut client: UserClient<Channel>,
  user_object: UserNew,
) -> ApiResult {
  let user = client
    .create_new(user_object.to_request(userid))
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();
  if let Some(user) = user.user {
    let _user: User = (&user).into();
    return Ok(reply::json(&_user));
  }
  Err(ApiError::not_found().into())
}
