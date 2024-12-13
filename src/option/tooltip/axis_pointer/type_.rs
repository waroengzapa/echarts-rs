use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Line,
    Shadow,
    None,
    Cross
}