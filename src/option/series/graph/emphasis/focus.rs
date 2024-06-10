use serde::Serialize;

#[derive(Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Focus {
    None,
    #[serde(rename = "self")]
    Self_,
    Series,
    Adjacency
}