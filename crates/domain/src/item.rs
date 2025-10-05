use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub tt_value_ped: f64,
    pub mu_percent: f64,
    pub mu_ped: f64,
}