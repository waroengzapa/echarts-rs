use super::AxisLine;

#[derive(Default)]
pub struct AxisLineBuilder {
    show: Option<bool>
}

impl AxisLineBuilder {
    pub fn build(self) -> AxisLine {
        AxisLine {
            show: self.show
        }
    }
    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }
}