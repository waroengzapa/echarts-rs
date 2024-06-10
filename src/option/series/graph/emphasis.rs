pub mod focus;

use serde::Serialize;
use self::focus::Focus;

#[derive(Serialize, Clone, Copy)]
pub struct Emphasis {
    pub focus: Focus
}