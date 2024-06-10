use serde::Serialize;

#[derive(Serialize, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Node {
    pub name: String,
    pub category: u8
}