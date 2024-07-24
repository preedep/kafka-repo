use crate::entities::APIError;
use log::debug;
use polars::prelude::*;

pub const COL_APP_OWNER_INVENTORY_FILE: &str = "Project";
pub const COL_TOPIC_NAME_INVENTORY_FILE: &str = "Topic_Name_Kafka";

pub const COL_CONSUMER_APP_NAME_CONSUMER_FILE: &str = "Project";

const IDX_COL_APP_OWNER_INVENTORY_FILE: usize = 0;
const IDX_COL_TOPIC_NAME_INVENTORY_FILE: usize = 1;

const IDX_COL_CONSUMER_APP_NAME_CONSUMER_FILE: usize = 0;

pub fn read_csv(file: &String) -> PolarsResult<DataFrame> {
    // Prefer `from_path` over `new` as it is faster.
    CsvReadOptions::default()
        .with_has_header(true)
        .with_infer_schema_length(None)
        .try_into_reader_with_file_path(Some(file.into()))?
        .finish()
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
