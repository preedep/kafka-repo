use std::sync::Arc;

use actix_files as fs;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{middleware, web, App};
use log::{debug, error, info};

use crate::data_service::read_csv;
use actix_web::dev::Service;
use actix_web::http::header;
use crate::data_utils::fetch_dataset_az_blob;

mod apis;
mod data_service;
mod data_state;
mod data_utils;
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

    let azure_blob_account_name =
        std::env::var("STORAGE_ACCOUNT").expect("AZURE_BLOB_ACCOUNT_NAME must be set");
    let azure_blob_container_name
        = std::env::var("STORAGE_CONTAINER").expect("AZURE_BLOB_CONTAINER_NAME must be set");


    debug!("Reading kafka inventory file: {}", kafka_inventory_file);
    debug!("Reading kafka consumer file: {}", kafka_consumer_file);
    debug!("Azure Blob Storage account: {}", azure_blob_account_name);
    debug!("Azure Blob Storage container: {}", azure_blob_container_name);


    let mut data_state = data_state::AppState {
        kafka_inventory: None,
        kafka_consumer: None,
    };

    // Fetch the dataset from Azure Blob Storage
    let ds_inventory = fetch_dataset_az_blob(
        &azure_blob_account_name,
        &azure_blob_container_name,
        &kafka_inventory_file,
    ).await;

    // Fetch the dataset from Azure Blob Storage
    let ds_consumer = fetch_dataset_az_blob(
        &azure_blob_account_name,
        &azure_blob_container_name,
        &kafka_consumer_file,
    ).await;

    // Check if the dataset was fetched successfully
    match ds_inventory {
        Ok(ds) => {
            data_state.kafka_inventory = Some(ds);
        }
        Err(e) => {
            //panic!("Failed to fetch kafka inventory from Azure Blob Storage: {}", e);
            error!("Failed to fetch kafka inventory from Azure Blob Storage: {}", e);
            let ds_kafka_inventory =
                read_csv(&kafka_inventory_file).expect("Failed to read kafka inventory file");
            data_state.kafka_inventory = Some(ds_kafka_inventory);
        }
    }
    // Check if the dataset was fetched successfully
    match ds_consumer {
        Ok(ds) => {
            data_state.kafka_consumer = Some(ds);
        }
        Err(e) => {
            //panic!("Failed to fetch kafka consumer from Azure Blob Storage: {}", e);
            error!("Failed to fetch kafka consumer from Azure Blob Storage: {}", e);
            let ds_kafka_consumer =
                read_csv(&kafka_consumer_file).expect("Failed to read kafka consumer file");
            data_state.kafka_consumer = Some(ds_kafka_consumer);
        }
    }

    /*
    let ds_kafka_inventory =
        read_csv(&kafka_inventory_file).expect("Failed to read kafka inventory file");
    let ds_kafka_consumer =
        read_csv(&kafka_consumer_file).expect("Failed to read kafka consumer file");


    let app_state = Arc::new(data_state::AppState {
        kafka_inventory: Some(ds_kafka_inventory),
        kafka_consumer: Some(ds_kafka_consumer),
    });
     */
    let app_state = Arc::new(data_state);
    info!("Starting server...");
    actix_web::HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(middleware::DefaultHeaders::new().add(("X-Version", "0.2")))
            .app_data(Data::new(app_state.clone()))
            .wrap_fn(|req, srv| {
                let fut = srv.call(req);
                async {
                    let mut res = fut.await?;
                    res.headers_mut().insert(
                        header::CACHE_CONTROL,
                        "no-store,no-cache,must-revalidate,proxy-validate,max-age=0"
                            .parse()
                            .unwrap(),
                    );
                    res.headers_mut()
                        .insert(header::EXPIRES, "0".parse().unwrap());
                    res.headers_mut()
                        .insert(header::PRAGMA, "no-cache".parse().unwrap());
                    Ok(res)
                }
            })
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
            .service(
                fs::Files::new("/", "./statics")
                    .index_file("index.html")
                    .use_last_modified(true)
                    .use_etag(true),
            )
    })
    .bind(("0.0.0.0", 8888))?
    .run()
    .await
}
