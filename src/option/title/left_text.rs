use serde::Serialize;

#[derive(Serialize, Clone, Copy, Debug)]
#[serde(rename_all = "lowercase")]
pub enum LeftText {
    Auto,
    Left,
    Center,
    Right
}