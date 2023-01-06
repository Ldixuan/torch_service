use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    id: Option<i64>,
    data_type: Option<String>,
    name: Option<String>,
}
