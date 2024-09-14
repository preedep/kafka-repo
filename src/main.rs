use std::sync::Arc;

use actix_cors::Cors;
use actix_files as fs;
use actix_rate_limiter::backend::memory::MemoryBackendProvider;
use actix_rate_limiter::limit::LimitBuilder;
use actix_rate_limiter::limiter::RateLimiterBuilder;
use actix_rate_limiter::middleware::RateLimiterMiddlewareFactory;
use actix_rate_limiter::route::RouteBuilder;
use actix_web::dev::Service;
use actix_web::http::header;
use actix_web::middleware::{DefaultHeaders, Logger};
use actix_web::web::Data;
use actix_web::{middleware, web, App};
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::data_service::read_csv;
use crate::data_utils::fetch_dataset_az_blob;
use crate::entities_ai::AISearchIndex;


use std::fs as file_system;
use async_openai::config::AzureConfig;

mod apis;
mod azure_ai_apis;
mod data_service;
mod data_state;
mod data_utils;
mod entities;
mod entities_ai;
mod export;
mod jwt_middleware;

#[derive(Debug, Serialize, Deserialize)]
struct MQTopicDescription {
    #[serde(rename = "business_module")]
    business_module: String,
    #[serde(rename = "topic_name")]
    topic_name: String,
    #[serde(rename = "publisher")]
    publisher: String,
    #[serde(rename = "remark")]
    remark: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct MQDataDescription {
    #[serde(rename = "mq_data_background")]
    mq_descriptions: String,
    #[serde(rename = "mq_data_current_state")]
    mq_data_current_state: String,
    #[serde(rename = "mq_technology")]
    mq_technology: String,
    #[serde(rename = "mq_pub_sub_topics")]
    mq_pub_sub_topics: Vec<MQTopicDescription>,
}

fn is_allowed_origin(origin: &str) -> bool {
    // List of allowed origins
    let allowed_origins = vec![
        "http://localhost:8888",
        "https://kafka-repo-dev001.azurewebsites.net",
    ];

    allowed_origins.contains(&origin)
}

fn load_mq_knowledge(file_path: &str) -> String {
    let file_content = file_system::read_to_string(file_path).expect("Failed to read JSON file");
    let parsed_json: MQDataDescription =
        serde_json::from_str(&file_content).expect("Failed to parse JSON");

    debug!("Parsed JSON: {:?}", parsed_json);

    let mut knowledge = String::new();
    knowledge.push_str("Here is the knowledge about Message sync MQ Pub/Sub :\n");
    knowledge.push_str(&parsed_json.mq_descriptions);
    knowledge.push_str("\n");
    knowledge.push_str("Here is the knowledge about Message sync MQ Pub/Sub Current State :\n");
    knowledge.push_str(&parsed_json.mq_data_current_state);
    knowledge.push_str("\n");
    knowledge.push_str("Here is the knowledge about Message sync MQ Pub/Sub Technology :\n");
    knowledge.push_str(&parsed_json.mq_technology);
    knowledge.push_str("\n");
    knowledge.push_str("Here is the knowledge about Message sync MQ Pub/Sub Topics :\n");
    for topic in parsed_json.mq_pub_sub_topics {
        knowledge.push_str("Business Module: ");
        knowledge.push_str(&topic.business_module);
        knowledge.push_str("\n");
        knowledge.push_str("Topic Name or Topic String: ");
        knowledge.push_str(&topic.topic_name);
        knowledge.push_str("\n");
        knowledge.push_str("Publisher: ");
        knowledge.push_str(&topic.publisher);
        knowledge.push_str("\n");
        knowledge.push_str("Remark: ");
        knowledge.push_str(&topic.remark);
        knowledge.push_str("\n");
    }
    knowledge.push_str("\n");
    knowledge
}

fn create_openai(open_ai_url: &str, open_ai_key: &str) -> AzureConfig {
    debug!("open_ai_url: {}", open_ai_url);

    let azure_config = AzureConfig::default()
        .with_api_base(open_ai_url)
        .with_api_key(open_ai_key)
        .with_api_version("2023-03-15-preview")
        .with_deployment_id("gpt-4");

    azure_config
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    dotenv::dotenv().ok();

    // Read environment variables
    let kafka_inventory_file =
        std::env::var("KAFKA_INVENTORY_FILE").expect("KAFKA_INVENTORY_FILE must be set");
    let kafka_consumer_file =
        std::env::var("KAFKA_CONSUMER_FILE").expect("KAFKA_CONSUMER must be set");
    // User Authentication
    let user_authentication_file =
        std::env::var("USER_AUTHENTICATION_FILE").expect("USER_AUTHENTICATION_FILE must be set");
    // Azure Blob Storage for Datasets Kafka Inventory and Kafka Consumer and User Authentication
    let azure_blob_account_name =
        std::env::var("STORAGE_ACCOUNT").expect("AZURE_BLOB_ACCOUNT_NAME must be set");
    let azure_blob_container_name =
        std::env::var("STORAGE_CONTAINER").expect("AZURE_BLOB_CONTAINER_NAME must be set");

    // JWT Secret Key
    let jwt_secret_key = std::env::var("JWT_SECRET_KEY").expect("JWT_SECRET must be set");
    // Azure AI Search
    let ai_search_api_url =
        std::env::var("AI_SEARCH_SERVICE_URL").expect("AI_SEARCH_URL must be set");
    let ai_search_api_key = std::env::var("AI_SEARCH_KEY").expect("AI_SEARCH_KEY must be set");

    let ai_search_indexes =
        std::env::var("AI_SEARCH_SERVICE_INDEXES").expect("AI_SEARCH_SERVICE_INDEXES must be set");

    let use_semantics = std::env::var("AI_SEARCH_WITH_SEMANTIC")
        .expect("AI_SEARCH_WITH_SEMANTIC must be set")
        .parse::<bool>()
        .expect("AI_SEARCH_WITH_SEMANTIC must be a boolean");

    // Open AI
    let open_ai_url = std::env::var("OPEN_AI_SERVICE_URL").expect("OPENAI_URL must be set");
    let open_api_key = std::env::var("OPEN_AI_KEY").expect("OPENAI_KEY must be set");

    debug!("Reading kafka inventory file: {}", kafka_inventory_file);
    debug!("Reading kafka consumer file: {}", kafka_consumer_file);
    debug!("Azure Blob Storage account: {}", azure_blob_account_name);
    debug!(
        "Azure Blob Storage container: {}",
        azure_blob_container_name
    );
    debug!("AI Search URL : {}", ai_search_api_url);
    debug!("Open AI Search URL : {}", open_ai_url);

    let knowledge = load_mq_knowledge("dataset/mq_data.json");

    debug!("AI Search Indexes: {}", ai_search_indexes);
    let azure_index = serde_json::from_str::<Vec<AISearchIndex>>(&ai_search_indexes)
        .expect("Failed to parse index");
    let mut data_state = data_state::AppState {
        kafka_inventory: None,
        kafka_consumer: None,
        user_authentication: None,
        jwt_secret: jwt_secret_key.clone(),
        // Azure AI Search
        azure_ai_search_url: Some(ai_search_api_url),
        azure_ai_search_key: Some(ai_search_api_key),
        //azure_ai_search_indexes: Some(ai_search_indexes.split(",").map(|s| s.trim().to_string()).collect()),
        azure_ai_search_indexes: Some(azure_index),
        azure_ai_search_use_semantics: use_semantics,

        // static knowledge
        knowledge: Some(knowledge),
        // Open AI
        open_ai_config: create_openai(&open_ai_url, &open_api_key),
    };

    // Fetch the dataset from Azure Blob Storage
    let ds_inventory = fetch_dataset_az_blob(
        &azure_blob_account_name,
        &azure_blob_container_name,
        &kafka_inventory_file,
    )
    .await;

    // Fetch the dataset from Azure Blob Storage
    let ds_consumer = fetch_dataset_az_blob(
        &azure_blob_account_name,
        &azure_blob_container_name,
        &kafka_consumer_file,
    )
    .await;

    // Fetch the dataset from Azure Blob Storage
    let ds_user_authentication = fetch_dataset_az_blob(
        &azure_blob_account_name,
        &azure_blob_container_name,
        &user_authentication_file,
    )
    .await;

    // Check if the dataset was fetched successfully
    match ds_inventory {
        Ok(ds) => {
            data_state.kafka_inventory = Some(ds);
        }
        Err(e) => {
            //panic!("Failed to fetch kafka inventory from Azure Blob Storage: {}", e);
            error!(
                "Failed to fetch kafka inventory from Azure Blob Storage: {}",
                e
            );
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
            error!(
                "Failed to fetch kafka consumer from Azure Blob Storage: {}",
                e
            );
            let ds_kafka_consumer =
                read_csv(&kafka_consumer_file).expect("Failed to read kafka consumer file");
            data_state.kafka_consumer = Some(ds_kafka_consumer);
        }
    }

    // Check if the dataset was fetched successfully
    match ds_user_authentication {
        Ok(ds) => {
            data_state.user_authentication = Some(ds);
        }
        Err(e) => {
            //panic!("Failed to fetch kafka consumer from Azure Blob Storage: {}", e);
            panic!(
                "Failed to fetch user authentication from Azure Blob Storage: {}",
                e
            );
        }
    }

    // Rate Limit
    let limit = LimitBuilder::new().set_ttl(10).set_amount(20).build();
    // Rate Limiter
    let limiter = RateLimiterBuilder::new()
        .add_route(
            RouteBuilder::new()
                .set_path("/api/v1/ai_search")
                .set_method("POST")
                .build(),
            limit.clone(),
        )
        .add_route(
            RouteBuilder::new()
                .set_path("/api/v1/search")
                .set_method("POST")
                .build(),
            limit.clone(),
        )
        .add_route(
            RouteBuilder::new()
                .set_path("/api/v1/render")
                .set_method("POST")
                .build(),
            limit.clone(),
        )
        .add_route(
            RouteBuilder::new()
                .set_path("/api/v1/apps")
                .set_method("GET")
                .build(),
            limit.clone(),
        )
        .add_route(
            RouteBuilder::new()
                .set_path("/api/v1/apps/{appName}/topics")
                .set_method("GET")
                .build(),
            limit.clone(),
        )
        .add_route(
            RouteBuilder::new()
                .set_path("/api/v1/consumers")
                .set_method("GET")
                .build(),
            limit.clone(),
        )
        .build();

    let backend = MemoryBackendProvider::default();
    let rate_limiter = RateLimiterMiddlewareFactory::new(limiter, Arc::new(Mutex::new(backend)));

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
            .wrap(
                Cors::default()
                    .allowed_origin_fn(|origin, _req_head| {
                        debug!("Origin: {:?}", origin);
                        is_allowed_origin(origin.to_str().unwrap())
                    })
                    .allowed_methods(vec!["GET", "POST"]),
            )
            .wrap(rate_limiter.clone())
            .wrap(jwt_middleware::JwtMiddleware::new(jwt_secret_key.clone()))
            .wrap(
                DefaultHeaders::new()
                    .add(("X-Content-Type-Options", "nosniff"))
                    .add(("X-Frame-Options", "DENY"))
                    .add(("X-XSS-Protection", "1; mode=block"))
                    .add((
                        "Strict-Transport-Security",
                        "max-age=31536000; includeSubDomains",
                    )),
            )
            .service(
                web::scope("/api/v1")
                    .route("/apps", web::get().to(apis::get_apps))
                    .route("/apps/{appName}/topics", web::get().to(apis::get_topics))
                    .route("/consumers", web::get().to(apis::get_consumers))
                    .route("/search", web::post().to(apis::post_search_kafka))
                    .route("/ai_search", web::post().to(apis::post_ai_search))
                    .route(
                        "/render",
                        web::post().to(apis::post_topic_kafka_relation_render),
                    ),
            )
            .service(
                web::scope("/api/authenticate/v1").route("/login", web::post().to(apis::login)),
            )
            .service(
                fs::Files::new("/", "./statics")
                    .index_file("login.html")
                    .use_last_modified(true)
                    .use_etag(true),
            )
    })
    .bind(("0.0.0.0", 8888))?
    .run()
    .await
}
