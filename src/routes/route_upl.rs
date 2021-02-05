use crate::{
  handler,
  routes::{add, auth},
  services::Services,
};
use warp::{Filter, Reply};

pub fn routes(services: Services) -> warp::filters::BoxedFilter<(impl Reply,)> {
  let upl_get_by_id = warp::path::param()
    .and(warp::get())
    .and(auth())
    .and(add(services.clone()))
    .and_then(handler::upl::get_upl_by_id);

  let upl_get_by_id_archive = warp::path!("archive")
    .and(warp::path::param())
    .and(warp::get())
    .and(auth())
    .and(add(services.clone()))
    .and_then(handler::upl::get_upl_by_id_archive);

  let upl_get_bulk = warp::path!("bulk")
    .and(warp::post())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::upl::get_upl_bulk);

  let split_upl = warp::path!("split")
    .and(warp::post())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::upl::split_upl);

  let divide_upl = warp::path!("divide")
    .and(warp::post())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::upl::divide_upl);

  let open = warp::path!("open")
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::upl::open);

  let close = warp::path!("close")
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::upl::close);

  let merge_back = warp::path!("merge_back")
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::upl::merge_back);

  let get_by_sku_stock = warp::path!("get_by_sku_stock")
    .and(warp::post())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::upl::get_by_sku_stock);

  let get_location_info = warp::path!("get_location_info")
    .and(warp::post())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::upl::get_location_info);

  let get_location_info_bulk = warp::path!("get_location_info_bulk")
    .and(warp::post())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::upl::get_location_info_bulk);

  warp::path!("upl" / ..)
    .and(balanced_or_tree!(upl_get_by_id
      .or(upl_get_by_id_archive)
      .or(upl_get_bulk)
      .or(get_by_sku_stock)
      .or(split_upl)
      .or(divide_upl)
      .or(open)
      .or(close)
      .or(get_location_info)
      .or(get_location_info_bulk)
      .or(merge_back)))
    .boxed()
}
