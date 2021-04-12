use crate::{
  handler,
  routes::{add, auth},
  services::Services,
};
use warp::{Filter, Reply};

pub fn routes(services: Services) -> warp::filters::BoxedFilter<(impl Reply,)> {
  let new_account = warp::path!("new")
    .and(warp::post())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::loyalty::new_account);

  let get_by_customer_id = warp::path!("customer" / ..)
    .and(warp::path::param())
    .and(warp::get())
    .and(auth())
    .and(add(services.clone()))
    .and_then(handler::loyalty::get_by_customer_id);

  let get_by_card_id = warp::path!("card" / ..)
    .and(warp::path::param())
    .and(warp::get())
    .and(auth())
    .and(add(services.clone()))
    .and_then(handler::loyalty::get_by_card_id);

  let get_by_query = warp::path!("query")
    .and(warp::post())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::loyalty::get_by_query);

  let get_transactions = warp::path!("transactions" / ..)
    .and(warp::path::param())
    .and(warp::get())
    .and(auth())
    .and(add(services.clone()))
    .and_then(handler::loyalty::get_transactions);

  let set_card = warp::path!("set_card")
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::loyalty::set_card);

  let set_loyalty_level = warp::path!("set_loyalty_level")
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::loyalty::set_loyalty_level);

  let set_birthdate = warp::path!("set_birthdate")
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::loyalty::set_birthdate);

  warp::path!("loyalty" / ..)
    .and(combine!(
      new_account,
      get_by_customer_id,
      get_by_card_id,
      get_by_query,
      get_transactions,
      set_card,
      set_loyalty_level,
      set_birthdate
    ))
    .boxed()
}
