use crate::{
  handler,
  routes::{add, auth},
  services::Services,
};
use warp::{Filter, Reply};

pub fn routes(services: Services) -> warp::filters::BoxedFilter<(impl Reply,)> {
  let create_new = warp::path!("new")
    .and(warp::post())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::procurement::create_new);

  let get_by_id = warp::path::param()
    .and(warp::get())
    .and(auth())
    .and(add(services.clone()))
    .and_then(handler::procurement::get_by_id);

  let get_all = warp::path!("all")
    .and(warp::get())
    .and(auth())
    .and(add(services.clone()))
    .and_then(handler::procurement::get_all);

  let get_bulk = warp::path!("bulk")
    .and(warp::post())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::procurement::get_bulk);

  let remove = warp::path::param()
    .and(warp::delete())
    .and(auth())
    .and(add(services.clone()))
    .and_then(handler::procurement::remove);

  let set_delivery_date = warp::path!("set_delivery_date")
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::procurement::set_delivery_date);

  let set_reference = warp::path!("set_reference")
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::procurement::set_reference);

  let add_sku = warp::path!("add_sku")
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::procurement::add_sku);

  let remove_sku = warp::path!("remove_sku")
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::procurement::remove_sku);

  let set_sku_piece = warp::path!("set_sku_piece")
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::procurement::set_sku_piece);

  let set_sku_price = warp::path!("set_sku_price")
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::procurement::set_sku_price);

  let add_upl = warp::path!("add_upl")
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::procurement::add_upl);

  let remove_upl = warp::path!("remove_upl")
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::procurement::remove_upl);

  let set_status_ordered = warp::path!("set_status_ordered" / ..)
    .and(warp::path::param())
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and_then(handler::procurement::set_status_ordered);

  let set_status_arrived = warp::path!("set_status_arrived" / ..)
    .and(warp::path::param())
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and_then(handler::procurement::set_status_arrived);

  let set_status_processing = warp::path!("set_status_processing" / ..)
    .and(warp::path::param())
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and_then(handler::procurement::set_status_processing);

  let set_status_closed = warp::path!("set_status_closed" / ..)
    .and(warp::path::param())
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and_then(handler::procurement::set_status_closed);

  warp::path!("procurement" / ..)
    .and(balanced_or_tree!(get_all
      .or(create_new)
      .or(get_by_id)
      .or(get_bulk)
      .or(remove)
      .or(set_delivery_date)
      .or(set_reference)
      .or(add_sku)
      .or(remove_sku)
      .or(set_sku_piece)
      .or(set_sku_price)
      .or(add_upl)
      .or(remove_upl)
      .or(set_status_ordered)
      .or(set_status_arrived)
      .or(set_status_processing)
      .or(set_status_closed)))
    .boxed()
}
