use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Data<'a> {
    value: &'a str
}
impl <'a>Data<'a> {
    pub fn new(value: &'a str) -> Self {
        Self { 
            value
        }
    }
}