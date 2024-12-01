use serde::Serialize;

// #[derive(Serialize, Clone, Debug, Hash, PartialEq, Eq)]
#[derive(Serialize, Clone, Debug)]
pub struct Node {
    name: String,
    x: u32,
    y: u32
//     pub category: u8
}
impl Node {
    pub fn new(name: &str, x: u32, y: u32) -> Self {
        Self { 
            name: name.to_owned(), 
            x, 
            y 
        }
    }
}