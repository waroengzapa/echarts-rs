mod name_location;
pub mod type_;
pub mod data;
mod builder;

use builder::XAxisBuilder;
// use data::Data;
// use name_location::NameLocation;
use serde::Serialize;
use type_::Type;


#[derive(Serialize, Default)]    
pub struct XAxis {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     name_location: Option<NameLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    type_: Option<Type>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // name: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // data: Option<Vec<String>>,
}

impl XAxis {
    pub fn builder() -> XAxisBuilder {
        XAxisBuilder::default()
    }
}