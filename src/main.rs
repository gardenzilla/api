#[macro_use]
mod balance;
mod handler;

use protos;
use protos::product::*;
use protos::{customer::customer_client::CustomerClient, user::*};
use warp::*;

mod prelude;
use prelude::*;
mod error;
use error::*;
mod login;
use login::UserId;
use product_client::ProductClient;
use user_client::UserClient;

fn auth() -> impl Filter<Extract = (UserId,), Error = Rejection> + Copy {
    warp::header::optional::<String>("Token").and_then(|n: Option<String>| async move {
        if let Some(token) = n {
            Ok(login::verify_token(&token).map_err(|err| ApiError::from(err))?)
        } else {
            Err(reject::custom(ApiRejection::new(
                warp::http::StatusCode::UNAUTHORIZED,
                "".into(),
            )))
        }
    })
}

fn with_db<T>(db: T) -> impl Filter<Extract = (T,), Error = std::convert::Infallible> + Clone
where
    T: Clone + Send,
{
    warp::any().map(move || db.clone())
}

#[tokio::main(max_threads = 10_000)]
async fn main() {
    // Service init
    let client = UserClient::connect("http://[::1]:50051").await.unwrap();
    let client_product = ProductClient::connect("http://[::1]:50054").await.unwrap();
    let client_customer = CustomerClient::connect("http://[::1]:50055").await.unwrap();

    let welcome = warp::path::end().map(|| format!("Welcome to Gardenzilla API"));

    /*
     * Login routes
     */
    let login_action = warp::path::end()
        .and(warp::post())
        .and(with_db(client.clone()))
        .and(warp::body::json())
        .and_then(handler::login::login);

    let login_password_reset = warp::path!("reset_password")
        .and(warp::post())
        .and(with_db(client.clone()))
        .and(warp::body::json())
        .and_then(handler::login::reset_password);

    let login = warp::path!("login" / ..).and(login_action.or(login_password_reset));

    /*
     * Profile routes
     */
    let profile_new_password = warp::path!("new_password")
        .and(warp::post())
        .and(auth())
        .and(with_db(client.clone()))
        .and(warp::body::json())
        .and_then(handler::user::new_password);

    let profile_get = warp::path::end()
        .and(warp::get())
        .and(auth())
        .and(with_db(client.clone()))
        .and_then(handler::user::get_profile);

    let profile_update = warp::path::end()
        .and(warp::post())
        .and(auth())
        .and(with_db(client.clone()))
        .and(warp::body::json())
        .and_then(handler::user::update_profile);

    let profile =
        warp::path!("profile" / ..).and(profile_new_password.or(profile_get).or(profile_update));

    /*
     * User routes
     */

    let user_get_all = warp::path!("all")
        .and(warp::get())
        .and(auth())
        .and(with_db(client.clone()))
        .and_then(handler::user::get_all);

    let user_get_by_id = warp::path::param()
        .and(warp::get())
        .and(auth())
        .and(with_db(client.clone()))
        .and_then(handler::user::get_by_id);

    let user_new = warp::path!("new")
        .and(warp::post())
        .and(auth())
        .and(with_db(client.clone()))
        .and(warp::body::json())
        .and_then(handler::user::create_new);

    let user = warp::path!("user" / ..).and(user_get_all.or(user_get_by_id).or(user_new));

    /*
     * Product routes
     */

    let product_get_all = warp::path!("all")
        .and(warp::get())
        .and(auth())
        .and(with_db(client_product.clone()))
        .and_then(handler::product::get_all);

    let product_get_by_id = warp::path::param()
        .and(warp::get())
        .and(auth())
        .and(with_db(client_product.clone()))
        .and_then(handler::product::get_by_id);

    let product_new = warp::path!("new")
        .and(warp::post())
        .and(auth())
        .and(with_db(client_product.clone()))
        .and(warp::body::json())
        .and_then(handler::product::create_new);

    let product_update = warp::path::param()
        .and(warp::put())
        .and(auth())
        .and(with_db(client_product.clone()))
        .and(warp::body::json())
        .and_then(handler::product::update);

    let product = warp::path!("product" / ..).and(
        product_get_all
            .or(product_get_by_id)
            .or(product_new)
            .or(product_update),
    );

    /*
     * Customer routes
     */

    let customer_get_all = warp::path!("all")
        .and(warp::get())
        .and(auth())
        .and(with_db(client_customer.clone()))
        .and_then(handler::customer::get_all);

    let customer_get_by_id = warp::path::param()
        .and(warp::get())
        .and(auth())
        .and(with_db(client_customer.clone()))
        .and_then(handler::customer::get_by_id);

    let customer_new = warp::path!("new")
        .and(warp::post())
        .and(auth())
        .and(with_db(client_customer.clone()))
        .and(warp::body::json())
        .and_then(handler::customer::create_new);

    let customer_update = warp::path::param()
        .and(warp::put())
        .and(auth())
        .and(with_db(client_customer.clone()))
        .and(warp::body::json())
        .and_then(handler::customer::update);

    let customer = warp::path!("customer" / ..).and(
        customer_get_all
            .or(customer_get_by_id)
            .or(customer_new)
            .or(customer_update),
    );

    // Compose routes
    let routes = warp::any().and(balanced_or_tree!(
        welcome, login, profile, user, product, customer
    ));
    // let routes = warp::any().and(welcome.or(login).or(profile).or(user).or(product));

    // Init server
    warp::serve(warp::any().and(routes).recover(handle_rejection))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
