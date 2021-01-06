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

  let upl_get_bulk = warp::path!("bulk")
    .and(warp::post())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::upl::get_upl_bulk);

  warp::path!("upl" / ..)
    .and(balanced_or_tree!(upl_get_by_id.or(upl_get_bulk)))
    .boxed()
}
