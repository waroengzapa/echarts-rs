use serde::Serialize;


#[derive(Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Layout {
    None,
    Circular,
    Force
}