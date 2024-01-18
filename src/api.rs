use actix_web::middleware::Logger;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

use crate::sim;
use crate::sim::cash::Account;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct ScenarioQuery {
    scenario: String,
}

#[derive(Debug, Serialize)]
struct ScenarioResponse {
    scenario: String,
    // Add more fields as needed
}

#[post("/results")]
async fn get_results(account: String) -> impl Responder {
    let account: Account = serde_json::from_str(&account).unwrap();
    let response = sim::run_simulation(account, None, false);
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    print!("Starting server...");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(index)
            .service(get_results)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
