use super::{data::Data, item_style::ItemStyle, Candlestick};

#[derive(Default)]
pub struct CandlestickBuilder {
    item_style: Option<ItemStyle>,
    data: Vec<Data>
}

impl CandlestickBuilder {
    pub fn build(self) -> Candlestick {
        Candlestick { 
            type_: "candlestick", 
            item_style: self.item_style,
            data: self.data
        }
    }
    pub fn data(mut self, data: Vec<Data>) -> Self {
        self.data = data;
        self
    }
    pub fn item_style(mut self, item_style: ItemStyle) -> Self {
        self.item_style = Some(item_style);
        self
    }
}