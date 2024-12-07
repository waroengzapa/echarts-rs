pub mod bar;
pub mod candlestick;
pub mod line;
pub mod graph;

use bar::Bar;
use candlestick::Candlestick;
use graph::{edge::Edge, node::Node, Graph};
use line::Line;
use serde::Serialize;

#[derive(Clone)]
pub enum Series {
    Bar(Bar),
    Candlestick(Candlestick),
    Line(Line),
    Graph(Graph),
}

impl Serialize for Series {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        match self {
            Self::Bar(bar) => bar.serialize(serializer),
            Self::Candlestick(candlestick) => candlestick.serialize(serializer),
            Self::Line(line) => line.serialize(serializer),
            Self::Graph(graph) => graph.serialize(serializer),
        }
    }
}
impl Series {
    pub fn candlestick(data: Vec<candlestick::data::Data>) -> Self {
        Self::Candlestick(Candlestick::new(data))
    }
    pub fn graph(nodes: Vec<Node>, edges: Vec<Edge>) -> Self {
        Self::Graph(Graph::new(nodes, edges))
    }
    pub fn line(data: Vec<line::data::Data>) -> Self {
        Self::Line(Line::new(data))
    }
}