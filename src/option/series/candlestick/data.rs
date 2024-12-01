use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    value: [f64;4],
}
impl Data {
    pub fn new(open:f64, high:f64, low:f64, close: f64) -> Self {
        Self { 
            value: [open, close, low, high],
        }
    }
    pub fn builder() -> DataBuilder {
        DataBuilder::default()
    }
}

#[derive(Default)]
pub struct DataBuilder {
    value: [f64;4],
}
impl DataBuilder {
    pub fn build(self) -> Data {
        Data {
            value: self.value,
        }
    }   
    pub fn value(mut self, open: f64, high: f64, low: f64, close: f64) -> Self {
        self.value = [open, close, low, high];
        self
    }
}