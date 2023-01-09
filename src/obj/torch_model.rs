use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TorchModel {
    id: Option<i64>
}