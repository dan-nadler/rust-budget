use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

use crate::database;
use crate::sim;


#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[get("/scenarios")]
async fn get_scenarios() -> impl Responder {
    let response = database::main().await.unwrap();
    HttpResponse::Ok().json(response)
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

#[get("/results")]
async fn get_results(web::Query(query): web::Query<ScenarioQuery>) -> impl Responder {
    // TODO: Read scenario from database
    println!("Scenario: {}", query.scenario);

    let config = std::fs::read_to_string("account.yaml").unwrap();
    let account: sim::cash::Account = serde_yaml::from_str(&config).unwrap();

    let response = sim::run_simulation(account);
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    print!("Starting server...");

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(get_scenarios)
            .service(get_results)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
