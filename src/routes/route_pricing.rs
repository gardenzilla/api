use crate::{
  handler,
  routes::{add, auth},
  services::Services,
};
use warp::{Filter, Reply};

pub fn routes(services: Services) -> warp::filters::BoxedFilter<(impl Reply,)> {
  let set_price = warp::path!("new")
    .and(warp::post())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::pricing::create_new);

  let get_by_id = warp::path::param()
    .and(warp::get())
    .and(auth())
    .and(add(services.clone()))
    .and_then(handler::pricing::get_by_id);

  let get_bulk = warp::path!("bulk")
    .and(warp::post())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::pricing::get_bulk);

  let get_price_history = warp::path!("history" / ..)
    .and(warp::path::param())
    .and(warp::get())
    .and(auth())
    .and(add(services.clone()))
    .and_then(handler::pricing::get_price_history);

  let get_price_changes = warp::path!("changes")
    .and(warp::post())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::pricing::get_latest_price_changes);

  warp::path!("price" / ..)
    .and(combine!(
      set_price,
      get_by_id,
      get_bulk,
      get_price_history,
      get_price_changes
    ))
    .boxed()
}
