use crate::prelude::*;
use crate::UserId;
use protos;
use protos::product::product_client::ProductClient;
use protos::product::*;
use serde::{Deserialize, Serialize};
use tonic::transport::Channel;
use warp::reply;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewProductForm {
  name: String,
  quantity: String,
  unit: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateProductForm {
  name: String,
  quantity: String,
  unit: String,
}

#[derive(Serialize, Deserialize)]
pub struct Product {
  pub sku: String,
  pub name: String,
  pub quantity: String,
  pub unit: String,
  pub date_created: String,
  pub created_by: String,
}

impl From<&ProductObj> for Product {
  fn from(p: &ProductObj) -> Self {
    Self {
      sku: p.sku.to_owned(),
      name: p.name.to_owned(),
      quantity: p.quantity.to_owned(),
      unit: p.unit.to_owned(),
      date_created: p.created_at.to_owned(),
      created_by: p.created_by.to_string(),
    }
  }
}

impl NewProductForm {
  fn to_request(self, created_by: UserId) -> CreateNewRequest {
    CreateNewRequest {
      name: self.name,
      quantity: self.quantity,
      unit: self.unit,
      created_by: created_by.into(),
    }
  }
}

pub async fn get_all(_: UserId, mut client: ProductClient<Channel>) -> ApiResult {
  let all = client
    .get_all(())
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();
  let v = all
    .products
    .iter()
    .map(|u| u.into())
    .collect::<Vec<Product>>();
  Ok(warp::reply::json(&v))
}

pub async fn get_by_id(
  sku: String,
  _userid: UserId,
  mut client: ProductClient<Channel>,
) -> ApiResult {
  let user = client
    .get_by_id(GetByIdRequest { sku: sku })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();
  if let Some(product) = user.product {
    let _product: Product = (&product).into();
    return Ok(reply::json(&_product));
  }
  Err(ApiError::not_found().into())
}

pub async fn update(
  sku: String,
  _: UserId,
  mut client: ProductClient<Channel>,
  p: UpdateProductForm,
) -> ApiResult {
  let res = client
    .update_by_id(UpdateByIdRequest {
      product: Some(ProductUpdateObj {
        sku: sku,
        name: p.name.to_owned(),
        quantity: p.quantity.to_owned(),
        unit: p.unit.to_owned(),
      }),
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();
  let user: Product = (&res.product.unwrap_or_default()).into();
  Ok(warp::reply::json(&user))
}

// pub async fn new_product(
//     userid: UserId,
//     mut client: UserClient<Channel>,
//     new_password_form: NewPasswordForm,
// ) -> ApiResult {
//     let p1 = match new_password_form.password1 {
//         Some(_pwd) => _pwd,
//         None => return Err(ApiError::bad_request("jelszó1 kötelező").into()),
//     };
//     let p2 = match new_password_form.password2 {
//         Some(_pwd) => _pwd,
//         None => return Err(ApiError::bad_request("jelszó2 kötelező").into()),
//     };
//     if &p1 != &p2 {
//         return Err(ApiError::BadRequest("A megadott jelszavak nem egyeznek meg!".into()).into());
//     }
//     client
//         .set_new_password(NewPasswordRequest {
//             userid: userid.into(),
//             new_password: p1,
//         })
//         .await
//         .map_err(|e| ApiError::from(e))?;
//     Ok(reply::json(&()))
// }

// pub async fn update_profile(
//     _: UserId,
//     mut client: UserClient<Channel>,
//     profile: User,
// ) -> ApiResult {
//     let res = client
//         .update_by_id(UpdateByIdRequest {
//             user: Some(UserObj {
//                 id: profile.username,
//                 name: profile.name,
//                 email: profile.email,
//                 phone: profile.phone,
//                 customers: profile.customers,
//                 created_by: profile.created_by,
//                 created_at: profile.date_created,
//             }),
//         })
//         .await
//         .map_err(|e| ApiError::from(e))?
//         .into_inner();
//     let user: User = (&res.user.unwrap()).into();
//     Ok(warp::reply::json(&user))
// }

// pub async fn get_all(_: UserId, mut client: UserClient<Channel>) -> ApiResult {
//     let all = client.get_all(()).await.unwrap().into_inner();
//     let v = all.users.iter().map(|u| u.into()).collect::<Vec<User>>();
//     Ok(warp::reply::json(&v))
// }

// pub async fn get_profile(userid: UserId, mut client: UserClient<Channel>) -> ApiResult {
//     let user = client
//         .get_by_id(GetByIdRequest {
//             userid: userid.into(),
//         })
//         .await
//         .map_err(|e| ApiError::from(e))?
//         .into_inner();
//     if let Some(user) = user.user {
//         let _user: User = (&user).into();
//         return Ok(reply::json(&_user));
//     }
//     Err(ApiError::not_found().into())
// }

// pub async fn get_by_id(id: String, userid: UserId, mut client: UserClient<Channel>) -> ApiResult {
//     let user = client
//         .get_by_id(GetByIdRequest { userid: id })
//         .await
//         .map_err(|e| ApiError::from(e))?
//         .into_inner();
//     if let Some(user) = user.user {
//         let _user: User = (&user).into();
//         return Ok(reply::json(&_user));
//     }
//     Err(ApiError::not_found().into())
// }

pub async fn create_new(
  userid: UserId,
  mut client: ProductClient<Channel>,
  new_product: NewProductForm,
) -> ApiResult {
  let product = client
    .create_new(new_product.to_request(userid))
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();
  if let Some(product) = product.product {
    let _prod: Product = (&product).into();
    return Ok(reply::json(&_prod));
  }
  Err(ApiError::not_found().into())
}
