use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Position {
    Left,
    Right
}