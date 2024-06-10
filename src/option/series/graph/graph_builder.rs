use super::{category::Category, edge::Edge, emphasis::Emphasis, label::Label, layout::Layout, line_style::LineStyle, node::Node, symbol::Symbol, Graph};

pub struct GraphBuilder {
    type_: &'static str,
    layout: Option<Layout>,
    roam: Option<bool>,
    edge_symbol: Option<[Symbol; 2]>,
    line_style: Option<LineStyle>,
    label: Option<Label>,
    emphasis: Option<Emphasis>,
    categories: Vec<Category>,
    nodes: Vec<Node>,
    edges: Vec<Edge>
}
impl Default for GraphBuilder  {
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
impl GraphBuilder {
    pub fn build(&self) -> Graph {
        Graph { 
            type_: self.type_, 
            layout: self.layout,
            roam: self.roam,
            edge_symbol: self.edge_symbol,
            line_style: self.line_style,
            label: self.label,
            emphasis: self.emphasis,
            categories: self.categories.clone(),
            nodes: self.nodes.clone(),
            edges: self.edges.clone()
        }
    }
    pub fn layout(mut self, layout: Layout) -> GraphBuilder {
        self.layout = Some(layout);
        self
    }
    pub fn roam(mut self, roam: bool) -> GraphBuilder {
        self.roam = Some(roam);
        self
    }
    pub fn line_style(mut self, line_style: LineStyle) -> GraphBuilder {
        self.line_style = Some(line_style);
        self
    }
    pub fn label(mut self, label: Label) -> GraphBuilder {
        self.label = Some(label);
        self
    }
    pub fn edge_symbol(mut self, edge_symbol: [Symbol;2]) -> GraphBuilder {
        self.edge_symbol = Some(edge_symbol);
        self
    }
    pub fn emphasis(mut self, emphasis: Emphasis) -> GraphBuilder {
        self.emphasis = Some(emphasis);
        self
    }
    pub fn push_category(mut self, category: Category) -> GraphBuilder {
        self.categories.push(category);
        self
    }
    pub fn add_nodes(mut self, nodes: Vec<Node>) -> GraphBuilder {
        self.nodes.extend(nodes);
        self
    }
    pub fn push_node(mut self, node: Node) -> GraphBuilder {
        self.nodes.push(node);
        self
    }
    pub fn add_edges(mut self, edges: Vec<Edge>) -> GraphBuilder {
        self.edges.extend(edges);
        self
    }
    pub fn push_edge(mut self, edge: Edge) -> GraphBuilder {
        self.edges.push(edge);
        self
    }
}