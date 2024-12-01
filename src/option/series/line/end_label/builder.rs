use super::EndLabel;

#[derive(Default)]
pub struct EndLabelBuilder {
    show: Option<bool>,
    formatter: Option<String>
}
impl EndLabelBuilder {
    pub fn build(self) -> EndLabel {
        EndLabel { 
            show: self.show, 
            formatter: self.formatter
        }
    }
    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }
    pub fn formatter(mut self, formatter: &str) -> Self {
        self.formatter = Some(formatter.to_string());
        self
    }
}