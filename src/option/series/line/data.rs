pub mod number;

use number::Number;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct Data {
    value: (String, Number)
}
impl Data {
    pub fn new(value: (String, Number)) -> Self {
        Self { 
            value
        }
    }
}
