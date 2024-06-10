pub mod node;
pub mod edge;
pub mod layout;
pub mod graph_builder;
pub mod symbol;
pub mod label;
pub mod category;
pub mod emphasis;
pub mod line_style;

use serde::Serialize;
use self::{category::Category, edge::Edge, emphasis::Emphasis, graph_builder::GraphBuilder, label::Label, layout::Layout, line_style::LineStyle, node::Node, symbol::Symbol};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Graph {
    #[serde(rename = "type")]
    type_: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    layout: Option<Layout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roam: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    edge_symbol: Option<[Symbol;2]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line_style: Option<LineStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Label>,
    #[serde(skip_serializing_if = "Option::is_none")]
    emphasis: Option<Emphasis>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    categories: Vec<Category>,
    nodes: Vec<Node>,
    edges: Vec<Edge>
}
impl Default for Graph  {
    fn default() -> Self {
        Self {
            type_: "graph",
            layout: None,
            roam: None,
            edge_symbol: None,
            line_style: None,
            label: None,
            emphasis: None,
            categories: Vec::new(),
            nodes: Vec::new(),
            edges: Vec::new()
        }
    }
}
impl Graph {
    pub fn builder() -> GraphBuilder {
        GraphBuilder::default()
    }
}