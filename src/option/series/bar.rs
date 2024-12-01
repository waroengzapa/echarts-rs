use data::Data;
use serde::Serialize;

pub mod data;

#[derive(Serialize, Clone, Debug)]
pub struct Bar {
    #[serde(rename = "type")]
    type_: &'static str,
    data: Vec<Data>
}
impl Bar {
    pub fn new(data: Vec<Data>) -> Self {
        Self { 
            type_: "bar", 
            data
        }
    }
}