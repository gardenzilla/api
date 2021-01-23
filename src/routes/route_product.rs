use crate::{
  handler,
  routes::{add, auth},
  services::Services,
};
use warp::{Filter, Reply};

pub fn routes(services: Services) -> warp::filters::BoxedFilter<(impl Reply,)> {
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

  let product_set_discontinued = warp::path!("set_discontinued")
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::product::product_set_discontinued);

  let product_set_perishable = warp::path!("set_perishable")
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::product::product_set_perishable);

  warp::path!("product" / ..)
    .and(balanced_or_tree!(product_get_all
      .or(product_new)
      .or(product_get_by_id)
      .or(product_get_bulk)
      .or(product_update)
      .or(product_find)
      .or(product_set_discontinued)
      .or(product_set_perishable)))
    .boxed()
}
