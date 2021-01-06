use crate::{
  handler,
  routes::{add, auth},
  services::Services,
};
use warp::{Filter, Reply};

pub fn routes(services: Services) -> warp::filters::BoxedFilter<(impl Reply,)> {
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

  warp::path!("sku" / ..)
    .and(balanced_or_tree!(sku_get_all
      .or(sku_new)
      .or(sku_get_by_id)
      .or(sku_get_bulk)
      .or(sku_update)
      .or(sku_find)
      .or(sku_set_divide)))
    .boxed()
}
