use serde::Serialize;

#[derive(Serialize)]
pub struct Title {
    pub text: &'static str
}