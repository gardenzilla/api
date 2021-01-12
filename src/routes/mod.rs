mod route_cash;
mod route_customer;
mod route_login;
mod route_pricing;
mod route_product;
mod route_profile;
mod route_sku;
mod route_stock;
mod route_upl;
mod route_user;

use crate::login;
use crate::prelude::*;
use crate::{error::*, services::Services};
use warp::*;

// Auth helper
// authenticates request TOKEN
pub fn auth() -> impl Filter<Extract = (u32,), Error = Rejection> + Copy {
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

pub fn add<T>(s: T) -> impl Filter<Extract = (T,), Error = std::convert::Infallible> + Clone
where
  T: Clone + Send,
{
  warp::any().map(move || s.clone())
}

pub async fn get_all(services: Services) -> warp::filters::BoxedFilter<(impl Reply,)> {
  let welcome = warp::path::end().map(|| format!("Welcome to Gardenzilla API"));

  // Compose routes
  let routes = warp::any().and(balanced_or_tree!(welcome
    .or(route_login::routes(services.clone()))
    .or(route_profile::routes(services.clone()))
    .or(route_user::routes(services.clone()))
    .or(route_product::routes(services.clone()))
    .or(route_sku::routes(services.clone()))
    .or(route_upl::routes(services.clone()))
    .or(route_customer::routes(services.clone()))
    .or(route_pricing::routes(services.clone()))
    .or(route_cash::routes(services.clone()))
    .or(route_stock::routes(services.clone()))));
  // let routes = warp::any().and(balanced_or_tree!(
  //   welcome, login, profile, user, product, customer, invoice
  // ));

  return routes.boxed();
}
