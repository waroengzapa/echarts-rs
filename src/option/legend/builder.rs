use super::{top_text::TopText, Legend};

#[derive(Default)]
pub struct LegendBuilder {
    top_text: Option<TopText>,
    data: Option<Vec<String>>
}
impl LegendBuilder {
    pub fn build(self) -> Legend {
        Legend {
            top_text: self.top_text,
            data: self.data
        }
    }
    pub fn top_text(mut self, top_text: TopText) -> Self {
        self.top_text = Some(top_text);
        self
    }
    pub fn data(mut self, data: Vec<&str>) -> Self {
        self.data = Some(data.iter().map(|item| item.to_string()).collect());
        self
    }
}