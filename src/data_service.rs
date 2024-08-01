use std::fmt::format;
use std::io::Cursor;

use actix_web::web::Data;
use log::debug;
use polars::prelude::*;

use crate::entities::{APIError, SearchKafkaRequest, SearchKafkaResponse};

// Inventory file
pub const COL_APP_OWNER_INVENTORY_FILE: &str = "Project";
pub const COL_TOPIC_NAME_INVENTORY_FILE: &str = "Topic_Name_Kafka";

const IDX_COL_APP_OWNER_INVENTORY_FILE: usize = 0;
const IDX_COL_TOPIC_NAME_INVENTORY_FILE: usize = 1;

// Consumer file
pub const COL_CONSUMER_APP_NAME_CONSUMER_FILE: &str = "Project";
pub const COL_CONSUMER_TOPIC_NAME_CONSUMER_FILE: &str = "Consume_Topic";
pub const COL_CONSUMER_GROUP_NAME_CONSUMER_FILE: &str = "Consumer_Group_Name";
pub const COL_CONSUMER_APP_NAME_2_CONSUMER_FILE: &str = "Consumer_App";
const IDX_COL_CONSUMER_APP_NAME_CONSUMER_FILE: usize = 0;

pub fn read_csv(file: &String) -> PolarsResult<DataFrame> {
    // Prefer `from_path` over `new` as it is faster.
    CsvReadOptions::default()
        .with_has_header(true)
        .with_infer_schema_length(None)
        .try_into_reader_with_file_path(Some(file.into()))?
        .finish()
}
pub fn read_csv_from_string(data: &String) -> PolarsResult<DataFrame> {
    // Prefer `from_path` over `new` as it is faster.
    // Create a cursor from the CSV string
    let cursor = Cursor::new(data);
    let option = CsvReadOptions::default()
        .with_has_header(true)
        .with_infer_schema_length(None);

    // Read the CSV string
    CsvReader::new(cursor).finish()
}
pub fn post_login(ds: &DataFrame, user_name: &String, password: &String) -> Result<bool, APIError> {
    debug!("Post login");

    let ds = ds.clone();
    let user_id = col("User_ID").eq(lit(user_name.as_str()));
    let password = col("Password").eq(lit(password.as_str()));

    let ds = ds
        .lazy().filter(user_id.and(password)).collect()
        .map_err(|e|{
            let message = format!("Failed to filter by username and password : [{:?}]",e.to_string());
            APIError::new(message.as_str())
        })?;
    Ok(ds.height() > 0)
}
pub fn get_app_list(ds: &DataFrame) -> Result<Vec<String>, APIError> {
    let mut app_list: Vec<String> = Vec::new();
    let ds = ds.clone();
    let ds = ds
        .lazy()
        .group_by([col(COL_APP_OWNER_INVENTORY_FILE)])
        .agg([col(COL_TOPIC_NAME_INVENTORY_FILE).count().alias("Count")])
        .sort(
            [COL_APP_OWNER_INVENTORY_FILE],
            SortMultipleOptions::new().with_order_descending(false),
        );
    let ds = ds.collect().map_err(|e| {
        debug!("Failed to group by app owner: {}", e);
        APIError::new("Failed to group by app owner")
    })?;
    map_single_result(&mut app_list, ds, IDX_COL_APP_OWNER_INVENTORY_FILE);
    Ok(app_list)
}
pub fn get_topic_list(ds: &DataFrame, app_name: &String) -> Result<Vec<String>, APIError> {
    let mut topic_list: Vec<String> = Vec::new();
    let ds = ds.clone();
    let ds = ds
        .lazy()
        .filter(col(COL_APP_OWNER_INVENTORY_FILE).eq(lit(app_name.as_str())))
        .sort(
            [COL_TOPIC_NAME_INVENTORY_FILE],
            SortMultipleOptions::new().with_order_descending(false),
        );
    let ds = ds.collect().map_err(|e| {
        debug!("Failed to filter by topic name: {}", e);
        APIError::new("Failed to group by topic name")
    })?;
    map_single_result(&mut topic_list, ds, IDX_COL_TOPIC_NAME_INVENTORY_FILE);
    Ok(topic_list)
}

pub fn get_consumer_list(ds: &DataFrame) -> Result<Vec<String>, APIError> {
    let mut consumer_list: Vec<String> = Vec::new();
    let ds = ds.clone();

    let ds = ds
        .lazy()
        .group_by([col(COL_CONSUMER_APP_NAME_CONSUMER_FILE)])
        .agg([col(COL_CONSUMER_APP_NAME_CONSUMER_FILE)
            .count()
            .alias("Count")])
        .sort(
            [COL_CONSUMER_APP_NAME_CONSUMER_FILE],
            SortMultipleOptions::new().with_order_descending(false),
        );
    let ds = ds.collect().map_err(|e| {
        debug!("Failed to group by consumer group: {}", e);
        APIError::new("Failed to group by consumer group")
    })?;

    map_single_result(
        &mut consumer_list,
        ds,
        IDX_COL_CONSUMER_APP_NAME_CONSUMER_FILE,
    );
    Ok(consumer_list)
}

fn map_single_result(topic_list: &mut Vec<String>, ds: DataFrame, idx: usize) {
    debug!("Mapping result : {} ", ds);
    for row in 0..ds.height() {
        let row = ds.get(row).unwrap();
        for (i, col) in row.iter().enumerate() {
            if i == idx {
                topic_list.push(col.to_string().replace("\"", ""));
            }
        }
    }
}

pub fn search(
    ds_inventory: &DataFrame,
    ds_consumer: &DataFrame,
    search_request: &SearchKafkaRequest,
) -> Result<Vec<SearchKafkaResponse>, APIError> {
    let mut result: Vec<SearchKafkaResponse> = Vec::new();

    let mut expr = col(COL_CONSUMER_APP_NAME_2_CONSUMER_FILE).is_not_null();

    if let Some(app_owner) = &search_request.app_owner {
        expr = expr.and(col(COL_APP_OWNER_INVENTORY_FILE).eq(lit(app_owner.as_str())));
    }
    if let Some(topic_name) = &search_request.topic_name {
        expr = expr.and(col(COL_TOPIC_NAME_INVENTORY_FILE).eq(lit(topic_name.as_str())));
    }
    if let Some(consumer_app) = &search_request.consumer_app {
        expr = expr.and(col(COL_CONSUMER_APP_NAME_2_CONSUMER_FILE).eq(lit(consumer_app.as_str())));
    }

    // Rename column
    let ds_consumer = ds_consumer
        .clone()
        .lazy()
        .with_column(
            col(COL_CONSUMER_APP_NAME_CONSUMER_FILE).alias(COL_CONSUMER_APP_NAME_2_CONSUMER_FILE),
        )
        .collect()
        .map_err(|e| {
            debug!("Failed to rename column: {}", e);
            APIError::new("Failed to rename column")
        })?;

    debug!("Renamed column: {}", ds_consumer);

    // join dataframes between inventory and consumer
    let joined = ds_inventory
        .join(
            &ds_consumer,
            [COL_TOPIC_NAME_INVENTORY_FILE],
            [COL_CONSUMER_TOPIC_NAME_CONSUMER_FILE],
            JoinArgs::new(JoinType::Left),
        )
        .map_err(|e| {
            debug!("Failed to join dataframes: {}", e);
            APIError::new("Failed to join dataframes")
        })?;

    let joined = joined.lazy().filter(expr).collect().map_err(|e| {
        debug!("Failed to filter dataframes: {}", e);
        APIError::new("Failed to filter dataframes")
    })?;

    debug!("Joined dataframe: {}", joined);
    // map result
    for row in 0..joined.height() {
        let mut search_kafka_response = SearchKafkaResponse::default();
        let row = joined.get(row).unwrap();
        for (i, col) in row.iter().enumerate() {
            if i == 0 {
                search_kafka_response.app_owner = col.to_string().replace("\"", "");
            } else if i == 1 {
                search_kafka_response.topic_name = col.to_string().replace("\"", "");
            } else if i == 3 {
                search_kafka_response.consumer_group_id = col.to_string().replace("\"", "");
            } else if i == 5 {
                search_kafka_response.consumer_app = col.to_string().replace("\"", "");
            }
        }
        result.push(search_kafka_response);
    }

    Ok(result)
}
