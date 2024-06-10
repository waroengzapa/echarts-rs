use serde::Serialize;

use super::symbol::Symbol;

#[derive(Serialize, Clone, Copy)]
pub struct Category {
    pub name: &'static str,
    pub symbol: Symbol
}