use warp::*;

use crate::handler;
use crate::login;
use crate::prelude::*;
use crate::{error::*, services::Services};

fn auth() -> impl Filter<Extract = (u32,), Error = Rejection> + Copy {
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

fn add<T>(s: T) -> impl Filter<Extract = (T,), Error = std::convert::Infallible> + Clone
where
  T: Clone + Send,
{
  warp::any().map(move || s.clone())
}

pub async fn get_all(services: Services) -> warp::filters::BoxedFilter<(impl Reply,)> {
  let welcome = warp::path::end().map(|| format!("Welcome to Gardenzilla API"));
  /*
   * Login routes
   */
  let login_action = warp::path::end()
    .and(warp::post())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::login::login);

  let login_password_reset = warp::path!("reset_password")
    .and(warp::post())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::login::reset_password);

  let login =
    warp::path!("login" / ..).and(balanced_or_tree!(login_action.or(login_password_reset)));

  let profile_new_password = warp::path!("new_password")
    .and(warp::post())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::user::new_password);

  let profile_get = warp::path::end()
    .and(warp::get())
    .and(auth())
    .and(add(services.clone()))
    .and_then(handler::user::get_profile);

  let profile_update = warp::path::end()
    .and(warp::post())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::user::update_profile);

  let profile = warp::path!("profile" / ..).and(balanced_or_tree!(profile_new_password
    .or(profile_get)
    .or(profile_update)));

  let user_get_all = warp::path!("all")
    .and(warp::get())
    .and(auth())
    .and(add(services.clone()))
    .and_then(handler::user::get_all);

  let user_get_by_id = warp::path::param()
    .and(warp::get())
    .and(auth())
    .and(add(services.clone()))
    .and_then(handler::user::get_by_id);

  let user_new = warp::path!("new")
    .and(warp::post())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::user::create_new);

  let user = warp::path!("user" / ..).and(balanced_or_tree!(user_get_all
    .or(user_get_by_id)
    .or(user_new)));

  let product_new = warp::path!("new")
    .and(warp::post())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::product::create_product);

  let product_get_all = warp::path!("all")
    .and(warp::get())
    .and(auth())
    .and(add(services.clone()))
    .and_then(handler::product::get_product_all);

  let product_get_by_id = warp::path::param()
    .and(warp::get())
    .and(auth())
    .and(add(services.clone()))
    .and_then(handler::product::get_product_by_id);

  let product_get_bulk = warp::path!("bulk")
    .and(warp::post())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::product::get_product_bulk);

  let product_update = warp::path::param()
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::product::update_product);

  let product_find = warp::path!("find")
    .and(warp::post())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::product::find_product);

  let product = warp::path!("product" / ..).and(balanced_or_tree!(product_get_all
    .or(product_new)
    .or(product_get_by_id)
    .or(product_get_bulk)
    .or(product_update)
    .or(product_find)));

  let sku_new = warp::path!("new")
    .and(warp::post())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::product::create_sku);

  let sku_get_all = warp::path!("all")
    .and(warp::get())
    .and(auth())
    .and(add(services.clone()))
    .and_then(handler::product::get_sku_all);

  let sku_get_by_id = warp::path::param()
    .and(warp::get())
    .and(auth())
    .and(add(services.clone()))
    .and_then(handler::product::get_sku_by_id);

  let sku_get_bulk = warp::path!("bulk")
    .and(warp::post())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::product::get_sku_bulk);

  let sku_update = warp::path::param()
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::product::update_sku);

  let sku_find = warp::path!("find")
    .and(warp::post())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::product::find_sku);

  let sku_set_divide = warp::path!("set_divide")
    .and(warp::post())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::product::sku_set_divide);

  let sku = warp::path!("sku" / ..).and(balanced_or_tree!(sku_get_all
    .or(sku_new)
    .or(sku_get_by_id)
    .or(sku_get_bulk)
    .or(sku_update)
    .or(sku_find)
    .or(sku_set_divide)));

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
  let routes = warp::any().and(balanced_or_tree!(
    welcome, login, profile, user, product, sku
  ));
  // let routes = warp::any().and(balanced_or_tree!(
  //   welcome, login, profile, user, product, customer, invoice
  // ));

  return routes.boxed();
}
