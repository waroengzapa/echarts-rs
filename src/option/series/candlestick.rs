mod builder;
pub mod data;
pub mod item_style;

use builder::CandlestickBuilder;
use data::Data;
use item_style::ItemStyle;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Candlestick {
    #[serde(rename = "type")]
    type_: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    item_style: Option<ItemStyle>,
    data: Vec<Data>
}

impl Candlestick {
    pub fn new(data: Vec<Data>) -> Self {
        Candlestick {
            type_: "candlestick",
            item_style: None,
            data
        }
    }
    pub fn builder() -> CandlestickBuilder {
        CandlestickBuilder::default()
    }
}

