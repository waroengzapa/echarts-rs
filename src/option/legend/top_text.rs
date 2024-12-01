use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TopText {
    Top,
    Middle,
    Bottom
}