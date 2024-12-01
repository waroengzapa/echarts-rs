use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LeftText {
    Auto,
    Left,
    Center,
    Right
}