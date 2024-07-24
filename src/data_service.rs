use log::debug;
use polars::prelude::*;
use crate::entities::APIError;

pub const COL_APP_OWNER_INVENTORY_FILE: &str = "Project";
pub const COL_TOPIC_NAME_INVENTORY_FILE: &str = "Topic_Name_Kafka";


pub fn read_csv(file: &String) -> PolarsResult<DataFrame> {
    // Prefer `from_path` over `new` as it is faster.
    CsvReadOptions::default()
        .with_has_header(true)
        .with_infer_schema_length(None)
        .try_into_reader_with_file_path(Some(file.into()))?
        .finish()
}

pub fn get_app_list(ds: &DataFrame) -> Result<Vec<String>,APIError> {
    let mut app_list: Vec<String> = Vec::new();
    let ds = ds.clone();
    let ds = ds.lazy().group_by([col(COL_APP_OWNER_INVENTORY_FILE)]).agg(
          [col(COL_TOPIC_NAME_INVENTORY_FILE).count().alias("Count")]
    ).sort([COL_APP_OWNER_INVENTORY_FILE], SortMultipleOptions::new().with_order_descending(false));
    let ds = ds.collect().map_err(|e| {
        debug!("Failed to group by app owner: {}", e);
        APIError::new("Failed to group by app owner")
    })?;
    for row in 0..ds.height() {
        let row = ds.get(row).unwrap();
        for (i, col) in row.iter().enumerate() {
            if i == 0 {
                app_list.push(col.to_string().replace("\"",""));
            }
        }
    }
    Ok(app_list)
}
pub fn get_topics(ds: &DataFrame, app_name: &String) -> Result<Vec<String>,APIError> {
    let mut topic_list: Vec<String> = Vec::new();
    let ds = ds.clone();
    let ds = ds.lazy().filter(col(COL_APP_OWNER_INVENTORY_FILE).eq(lit(app_name.as_str())))
    .sort([COL_TOPIC_NAME_INVENTORY_FILE], SortMultipleOptions::new().with_order_descending(false));
    let ds = ds.collect().map_err(|e| {
        debug!("Failed to filter by topic name: {}", e);
        APIError::new("Failed to group by topic name")
    })?;
    for row in 0..ds.height() {
        let row = ds.get(row).unwrap();
        for (i, col) in row.iter().enumerate() {
            if i == 1 {
                topic_list.push(col.to_string().replace("\"",""));
            }
        }
    }
    Ok(topic_list)
}