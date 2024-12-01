use super::{left_text::LeftText, Title};

#[derive(Default)]
pub struct TitleBuilder {
    text: String,
    subtext: Option<String>,
    left_text: Option<LeftText>
}

impl TitleBuilder {
    pub fn build(self) -> Title {
        Title { 
            text: self.text,
            subtext: self.subtext,
            left_text: self.left_text
        }
    }
    pub fn text(mut self, text: &str) -> TitleBuilder {
        self.text = text.to_string();
        self
    }
    pub fn subtext(mut self, subtext: &str) -> TitleBuilder {
        self.subtext = Some(subtext.to_string());
        self
    }
    pub fn left_text(mut self, left_text: LeftText) -> TitleBuilder {
        self.left_text = Some(left_text);
        self
    }
}