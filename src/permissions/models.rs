use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Permission {
    pub id: Option<i32>,
    pub name: String,
    pub description: String,
}