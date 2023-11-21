mod shortest_path;

use std::collections::HashMap;

pub use shortest_path::shortest_path;

pub struct Edge<Pos, W> {
    pub target: Pos,
    pub weight: W,
}

pub type Graph<Pos, W> = HashMap<Pos, Vec<Edge<Pos, W>>>;
