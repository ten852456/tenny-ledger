mod config;
mod db;
mod routes;
mod error;
mod models;
mod handlers;
mod ocr;
mod fixtures;
mod schema;

use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer, web};
use dotenv::dotenv;
use std::env;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    
    // Check for command line args
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 && args[1] == "load-fixtures" {
        println!("Loading fixtures...");
        match fixtures::load_all_fixtures() {
            Ok(_) => println!("Fixtures loaded successfully!"),
            Err(e) => eprintln!("Error loading fixtures: {}", e),
        }
        return Ok(());
    }
    
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let server_url = format!("{}:{}", host, port);
    
    info!("Starting server at: {}", server_url);
    
    // Initialize database connection
    let pool = db::establish_connection();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
            
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .configure(routes::configure)
    })
    .bind(server_url)?
    .run()
    .await
} 