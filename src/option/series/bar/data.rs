pub mod number;

use number::Number;
use serde::Serialize;

#[derive(Serialize, Clone, Default, Debug)]
pub struct Data {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "value")]
    value_string_number: Option<(String, Number)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "value")]
    value_number_string: Option<(Number, String)>
}
impl Data {
    pub fn value_string_number(value_string_number: (String, Number)) -> Self {
        Self { 
            value_string_number: Some(value_string_number),
            ..Self::default()
        }
    }
    pub fn value_number_string(value_number_string: (Number, String)) -> Self {
        Self { 
            value_number_string: Some(value_number_string),
            ..Self::default()
        }
    }
}