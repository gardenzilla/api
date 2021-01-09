use crate::{
  handler,
  routes::{add, auth},
  services::Services,
};
use warp::{Filter, Reply};

// [POST] transaction/new_purchase
// [POST] transaction/new_general
// [GET ] transaction/<ID>
// [GET ] transaction/balance
// [POST] transaction/bulk
// [POST] transaction/date_range

pub fn routes(services: Services) -> warp::filters::BoxedFilter<(impl Reply,)> {
  let new_purchase = warp::path!("new_purchase")
    .and(warp::post())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::cash::new_transaction_purchase);

  let new_general = warp::path!("new_general")
    .and(warp::post())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::cash::new_transaction_general);

  let get_by_id = warp::path::param()
    .and(warp::get())
    .and(auth())
    .and(add(services.clone()))
    .and_then(handler::cash::get_by_id);

  let get_bulk = warp::path!("bulk")
    .and(warp::post())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::cash::get_bulk);

  let get_balance = warp::path!("balance")
    .and(warp::get())
    .and(auth())
    .and(add(services.clone()))
    .and_then(handler::cash::get_balance);

  let get_by_date_range = warp::path!("date_range")
    .and(warp::post())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::cash::get_by_date_range);

  warp::path!("cash" / ..)
    .and(balanced_or_tree!(new_purchase
      .or(new_general)
      .or(get_by_id)
      .or(get_bulk)
      .or(get_balance)
      .or(get_by_date_range)))
    .boxed()
}
