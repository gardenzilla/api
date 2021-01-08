mod route_customer;
mod route_login;
mod route_pricing;
mod route_product;
mod route_profile;
mod route_sku;
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

  // /**
  //  * Invoice routes
  //  */
  // let invoice_new = warp::path!("new")
  //   .and(warp::post())
  //   .and(auth())
  //   .and(with_db(client_invoice.clone()))
  //   .and(warp::body::json())
  //   .and_then(handler::invoice::new_invoice);

  // let invoice = warp::path!("invoice" / ..).and(balanced_or_tree!(invoice_new));

  // /*
  //  * Product routes
  //  */
  // let product_get_all = warp::path!("all")
  //   .and(warp::get())
  //   .and(auth())
  //   .and(with_db(client_product.clone()))
  //   .and_then(handler::product::get_all);

  // let product_get_by_id = warp::path::param()
  //   .and(warp::get())
  //   .and(auth())
  //   .and(with_db(client_product.clone()))
  //   .and_then(handler::product::get_by_id);

  // let product_new = warp::path!("new")
  //   .and(warp::post())
  //   .and(auth())
  //   .and(with_db(client_product.clone()))
  //   .and(warp::body::json())
  //   .and_then(handler::product::create_new);

  // let product_update = warp::path::param()
  //   .and(warp::put())
  //   .and(auth())
  //   .and(with_db(client_product.clone()))
  //   .and(warp::body::json())
  //   .and_then(handler::product::update);

  // let product = warp::path!("product" / ..).and(balanced_or_tree!(product_get_all
  //   .or(product_get_by_id)
  //   .or(product_new)
  //   .or(product_update),));

  // /*
  //  * Customer routes
  //  */
  // let customer_get_all = warp::path!("all")
  //   .and(warp::get())
  //   .and(auth())
  //   .and(with_db(client_customer.clone()))
  //   .and_then(handler::customer::get_all);

  // let customer_get_by_id = warp::path::param()
  //   .and(warp::get())
  //   .and(auth())
  //   .and(with_db(client_customer.clone()))
  //   .and_then(handler::customer::get_by_id);

  // let customer_new = warp::path!("new")
  //   .and(warp::post())
  //   .and(auth())
  //   .and(with_db(client_customer.clone()))
  //   .and(warp::body::json())
  //   .and_then(handler::customer::create_new);

  // let customer_update = warp::path::param()
  //   .and(warp::put())
  //   .and(auth())
  //   .and(with_db(client_customer.clone()))
  //   .and(warp::body::json())
  //   .and_then(handler::customer::update);

  // let customer = warp::path!("customer" / ..).and(balanced_or_tree!(customer_get_all
  //   .or(customer_get_by_id)
  //   .or(customer_new)
  //   .or(customer_update),));

  // Compose routes
  let routes = warp::any().and(balanced_or_tree!(welcome
    .or(route_login::routes(services.clone()))
    .or(route_profile::routes(services.clone()))
    .or(route_user::routes(services.clone()))
    .or(route_product::routes(services.clone()))
    .or(route_sku::routes(services.clone()))
    .or(route_upl::routes(services.clone()))
    .or(route_customer::routes(services.clone()))
    .or(route_pricing::routes(services.clone()))));
  // let routes = warp::any().and(balanced_or_tree!(
  //   welcome, login, profile, user, product, customer, invoice
  // ));

  return routes.boxed();
}
