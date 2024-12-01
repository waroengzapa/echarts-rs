use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct AxisLabel {
    #[serde(skip_serializing_if = "Option::is_none")]
    formatter: Option<String>
}

impl AxisLabel {
    pub fn new(formatter: &str) -> Self {
        Self {
            formatter: Some(formatter.to_string())
        }
    }
}