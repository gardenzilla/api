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
    .and_then(handler::source::create_new);

  let get_all = warp::path!("all")
    .and(warp::get())
    .and(auth())
    .and(add(services.clone()))
    .and_then(handler::source::get_all);

  let get_by_id = warp::path::param()
    .and(warp::get())
    .and(auth())
    .and(add(services.clone()))
    .and_then(handler::source::get_by_id);

  let update = warp::path::param()
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::source::update);

  warp::path!("source" / ..)
    .and(balanced_or_tree!(get_all
      .or(create_new)
      .or(get_by_id)
      .or(update)))
    .boxed()
}
