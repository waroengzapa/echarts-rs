pub mod graph;

use serde::Serialize;

use self::graph::Graph;

// #[derive(Serialize)]
pub enum Series {
    Graph(Graph)
}
impl Serialize for Series {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
    
        match self {
            Series::Graph(wee) => wee.serialize(serializer),
        }
    }
}
