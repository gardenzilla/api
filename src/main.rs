#[macro_use]
mod balance;
mod handler;
mod routes;

use protos;
use std::error::Error;
use warp::*;

mod error;
mod prelude;
use error::*;
mod login;
use login::UserId;
use tokio::{signal, sync::oneshot};

#[tokio::main(max_threads = 10_000)]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create shutdown channel
    let (tx, rx) = oneshot::channel();

    // Init server
    let (addr, server) = warp::serve(
        warp::any()
            .and(routes::get_all().await)
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
