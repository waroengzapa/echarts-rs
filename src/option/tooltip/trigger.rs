use serde::Serialize;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Trigger {
    Item,
    Axis,
    None
}