#[macro_use]
mod balance;
mod error;
mod handler;
mod login;
mod prelude;
mod routes;
mod services;
// use error::*;
// use login::UserId;
use error::handle_rejection;
use std::error::Error;
use tokio::{signal, sync::oneshot};
use warp::Filter;
// use warp::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let services = services::Services::init().await;
  // Create shutdown channel
  let (tx, rx) = oneshot::channel();

  // Init server
  let (addr, server) = warp::serve(
    warp::any()
      .and(routes::get_all(services).await)
      .recover(handle_rejection),
  )
  .bind_with_graceful_shutdown(([127, 0, 0, 1], 3030), async {
    rx.await.ok();
  });

  println!("API is running at {}", addr);

  // Spawn the server into a runtime
  tokio::task::spawn(server);

  signal::ctrl_c().await?;

  println!("SIGTERM");

  // Send shutdown signal after SIGTERM received
  let _ = tx.send(());

  Ok(())
}
