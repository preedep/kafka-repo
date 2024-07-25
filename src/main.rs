use std::sync::Arc;

use actix_files as fs;
use actix_web::{App, middleware, web};
use actix_web::middleware::Logger;
use actix_web::web::{Data, route};
use log::info;

use crate::data_service::read_csv;

mod apis;
mod data_service;
mod data_state;
mod entities;
mod export;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    dotenv::dotenv().ok();

    let kafka_inventory_file =
        std::env::var("KAFKA_INVENTORY_FILE").expect("KAFKA_INVENTORY_FILE must be set");
    let kafka_consumer_file =
        std::env::var("KAFKA_CONSUMER_FILE").expect("KAFKA_CONSUMER must be set");

    info!("Reading kafka inventory file: {}", kafka_inventory_file);
    info!("Reading kafka consumer file: {}", kafka_consumer_file);

    let ds_kafka_inventory =
        read_csv(&kafka_inventory_file).expect("Failed to read kafka inventory file");
    let ds_kafka_consumer =
        read_csv(&kafka_consumer_file).expect("Failed to read kafka consumer file");

    let app_state = Arc::new(data_state::AppState {
        kafka_inventory: Some(ds_kafka_inventory),
        kafka_consumer: Some(ds_kafka_consumer),
    });

    info!("Starting server...");

    actix_web::HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(middleware::DefaultHeaders::new().add(("X-Version", "0.2")))
            .app_data(Data::new(app_state.clone()))
            .service(
                web::scope("/api/v1")
                    .route("/apps", web::get().to(apis::get_apps))
                    .route("/apps/{appName}/topics", web::get().to(apis::get_topics))
                    .route("/consumers", web::get().to(apis::get_consumers))
                    .route("/search", web::post().to(apis::post_search_kafka))
                    .route(
                        "/render",
                        web::post().to(apis::post_topic_kafka_relation_render),
                    ),
            )
            .service(fs::Files::new("/", "./statics").index_file("index.html"))
    })
    .bind(("0.0.0.0", 8888))?
    .run()
    .await
}
