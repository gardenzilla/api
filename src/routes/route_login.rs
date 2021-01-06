use crate::{handler, routes::add, services::Services};
use warp::{Filter, Reply};

pub fn routes(services: Services) -> warp::filters::BoxedFilter<(impl Reply,)> {
  let login_action = warp::path::end()
    .and(warp::post())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::login::login);

  let login_password_reset = warp::path!("reset_password")
    .and(warp::post())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::login::reset_password);

  warp::path!("login" / ..)
    .and(balanced_or_tree!(login_action.or(login_password_reset)))
    .boxed()
}
