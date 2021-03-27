use crate::{
  handler,
  routes::{add, auth},
  services::Services,
};
use warp::{Filter, Reply};

pub fn routes(services: Services) -> warp::filters::BoxedFilter<(impl Reply,)> {
  let add_new = warp::path::param()
    .and(warp::post())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::multipart::form().max_length(3_000_000))
    .and_then(handler::sku_image::add_new);

  let get_images = warp::path::param()
    .and(warp::get())
    .and(auth())
    .and(add(services.clone()))
    .and_then(handler::sku_image::get_images);

  let get_cover_bulk = warp::path!("cover_bulk")
    .and(warp::post())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::sku_image::get_cover_bulk);

  warp::path!("sku_image" / ..)
    .and(balanced_or_tree!(add_new.or(get_images).or(get_cover_bulk)))
    .boxed()
}
