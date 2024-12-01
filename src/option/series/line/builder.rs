use super::{data::Data, end_label::EndLabel, Line};

#[derive(Default)]
pub struct LineBuilder {
    name: Option<String>,
    y_axis_index: Option<usize>,
    end_label: Option<EndLabel>,
    data: Vec<Data>
}
impl LineBuilder {
    pub fn build(self) -> Line {
        Line { 
            type_: "line", 
            name: self.name,
            y_axis_index: self.y_axis_index, 
            end_label: self.end_label,
            data: self.data
        }
    }
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
    pub fn y_axis_index(mut self, index: usize) -> Self {
        self.y_axis_index = Some(index);
        self
    }
    pub fn end_label(mut self, end_label: EndLabel) -> Self {
        self.end_label = Some(end_label);
        self
    }
    pub fn data(mut self, data: Vec<Data>) -> Self {
        self.data = data;
        self
    }    
}