use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use protos;
use tokio::sync::Mutex;
// use tonic::{transport::Server, Request, Response, Status};

const DEFAULT_PORT: u32 = 7000;

struct AppState {
    client: Mutex<protos::user::user_client::UserClient<tonic::transport::Channel>>,
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn hello(client: web::Data<AppState>) -> HttpResponse {
    let all = client
        .client
        .lock()
        .await
        .get_all(())
        .await
        .unwrap()
        .into_inner();
    let v = all
        .users
        .iter()
        .map(|u| u.id.to_string())
        .collect::<Vec<String>>();
    HttpResponse::Ok().json(v)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let port = match std::env::var("API_PORT") {
        Ok(val) => {
            let _val = val.parse::<u32>().expect("API_PORT ENV must be u32");
            println!("Running at port {}", _val);
            _val
        }
        Err(_) => {
            println!("THERE IS NO API_PORT ENV, so set it to {}", DEFAULT_PORT);
            DEFAULT_PORT
        }
    };
    let client = protos::user::user_client::UserClient::connect("http://[::1]:50051")
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                client: Mutex::new(client.clone()),
            })
            .route("/", web::get().to(greet))
            .route("/hello", web::get().to(hello))
            .route("/{name}", web::get().to(greet))
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}
