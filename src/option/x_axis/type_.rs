use serde::Serialize;

#[derive(Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Value,
    Category,
    Time,
    Log
}