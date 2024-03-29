use crate::{
  handler,
  routes::{add, auth},
  services::Services,
};
use warp::{Filter, Reply};

pub fn routes(services: Services) -> warp::filters::BoxedFilter<(impl Reply,)> {
  let new = warp::path!("new")
    .and(warp::post())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::cart::new_cart);

  let get_all = warp::path!("all")
    .and(warp::get())
    .and(auth())
    .and(add(services.clone()))
    .and_then(handler::cart::cart_get_all);

  let get_by_id = warp::path::param()
    .and(warp::get())
    .and(auth())
    .and(add(services.clone()))
    .and_then(handler::cart::cart_get_by_id);

  let get_bulk = warp::path!("bulk")
    .and(warp::post())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::cart::get_bulk);

  let add_customer = warp::path!("add_customer")
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::cart::cart_add_customer);

  let remove_customer = warp::path!("remove_customer")
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::cart::cart_remove_customer);

  let add_sku = warp::path!("add_sku")
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::cart::cart_add_sku);

  let set_sku_piece = warp::path!("set_sku_piece")
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::cart::cart_set_sku_piece);

  let remove_sku = warp::path!("remove_sku")
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::cart::cart_remove_sku);

  let add_upl = warp::path!("add_upl")
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::cart::cart_add_upl);

  let remove_upl = warp::path!("remove_upl")
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::cart::cart_remove_upl);

  let set_payment = warp::path!("set_payment")
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::cart::cart_set_payment);

  let add_payment = warp::path!("add_payment")
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::cart::cart_add_payment);

  let set_document = warp::path!("set_document")
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::cart::cart_set_need_invoice);

  let add_loyalty_card = warp::path!("add_loyalty_card")
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::cart::cart_add_loyalty_card);

  let remove_loyalty_card = warp::path!("remove_loyalty_card")
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::cart::cart_remove_loyalty_card);

  let remove_commitment = warp::path!("remove_commitment")
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::cart::cart_remove_commitment);

  let burn_points = warp::path!("burn_loyalty_points")
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::cart::cart_burn_loyalty_points);

  let close = warp::path!("close")
    .and(warp::put())
    .and(auth())
    .and(add(services.clone()))
    .and(warp::body::json())
    .and_then(handler::cart::cart_close);

  warp::path!("cart" / ..)
    .and(combine!(
      get_all,
      new,
      get_by_id,
      get_bulk,
      add_customer,
      remove_customer,
      remove_sku,
      add_sku,
      set_sku_piece,
      add_upl,
      remove_upl,
      set_payment,
      add_payment,
      set_document,
      add_loyalty_card,
      remove_loyalty_card,
      remove_commitment,
      burn_points,
      close
    ))
    .boxed()
}
