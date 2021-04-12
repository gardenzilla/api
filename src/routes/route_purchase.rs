use crate::{
  handler,
  routes::{add, auth},
  services::Services,
};
use warp::{Filter, Reply};

pub fn routes(services: Services) -> warp::filters::BoxedFilter<(impl Reply,)> {
  let get_by_id = warp::path::param()
    .and(warp::path::end())
    .and(warp::get())
    .and(auth())
    .and(add(services.clone()))
    .and_then(handler::purchase::purchase_get_by_id);

  let get_info_by_id = warp::path!("info")
    .and(warp::post())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::purchase::purchase_info_get_by_id);

  let get_receipt_by_id = warp::path!("receipt")
    .and(warp::post())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::purchase::get_receipt);

  let get_all = warp::path!("all")
    .and(warp::get())
    .and(warp::path::end())
    .and(auth())
    .and(add(services.clone()))
    .and_then(handler::purchase::purchase_get_all);

  let get_bulk = warp::path!("bulk")
    .and(warp::post())
    .and(warp::path::end())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::purchase::get_bulk);

  warp::path!("purchase" / ..)
    .and(combine!(
      get_info_by_id,
      get_by_id,
      get_receipt_by_id,
      get_all,
      get_bulk
    ))
    .boxed()
}
