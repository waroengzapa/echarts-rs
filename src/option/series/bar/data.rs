use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct Data {
    value: f64
}
impl Data {
    pub fn new(value: f64) -> Self {
        Self { 
            value
        }
    }
}