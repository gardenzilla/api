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
    .and_then(handler::stock::create_new);

  let get_by_id = warp::path::param()
    .and(warp::get())
    .and(auth())
    .and(add(services.clone()))
    .and_then(handler::stock::get_by_id);

  let get_all = warp::path!("all")
    .and(warp::get())
    .and(auth())
    .and(add(services.clone()))
    .and_then(handler::stock::get_all);

  let update = warp::path::param()
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::stock::update);

  warp::path!("stock" / ..)
    .and(combine!(get_all, create_new, get_by_id, update))
    .boxed()
}
