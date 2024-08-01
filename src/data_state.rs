use polars::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppState {
    pub kafka_inventory: Option<DataFrame>,
    pub kafka_consumer: Option<DataFrame>,
    pub user_authentication : Option<DataFrame>
}
