use actix_web::web;
use crate::handlers::{auth, ocr, transactions, users};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/auth")
                    .route("/register", web::post().to(auth::register))
                    .route("/login", web::post().to(auth::login))
            )
            .service(
                web::scope("/ocr")
                    .route("/process", web::post().to(ocr::process_image))
                    .route("/process/engine", web::post().to(ocr::process_image_with_engine))
            )
            .service(
                web::scope("/transactions")
                    .route("", web::get().to(transactions::get_transactions))
                    .route("", web::post().to(transactions::create_transaction))
                    .route("/{id}", web::get().to(transactions::get_transaction))
                    .route("/{id}", web::put().to(transactions::update_transaction))
                    .route("/{id}", web::delete().to(transactions::delete_transaction))
            )
            .service(
                web::scope("/categories")
                    .route("", web::get().to(transactions::get_categories))
            )
            .service(
                web::scope("/users")
                    .route("/profile", web::get().to(users::get_profile))
                    .route("/profile", web::put().to(users::update_profile))
            )
    );
} 