use crate::{
  handler,
  routes::{add, auth},
  services::Services,
};
use warp::{Filter, Reply};

pub fn routes(services: Services) -> warp::filters::BoxedFilter<(impl Reply,)> {
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

  warp::path!("profile" / ..)
    .and(combine!(profile_new_password, profile_get, profile_update))
    .boxed()
}
