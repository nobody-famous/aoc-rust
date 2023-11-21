use std::collections::{HashMap, HashSet};

pub struct Edge<Pos, W> {
    pub target: Pos,
    pub weight: W,
}

pub type Graph<Pos, W> = HashMap<Pos, Vec<Edge<Pos, W>>>;

struct PathNode<'a, Pos, W> {
    pos: &'a Pos,
    path: Vec<&'a Pos>,
    weight: W,
}

struct State<'a, Pos, W> {
    visited: HashSet<&'a Pos>,
    frontier: HashMap<&'a Pos, PathNode<'a, &'a Pos, W>>,
}

fn add_edges<'a, Pos, W>(
    frontier: &HashMap<&'a Pos, PathNode<'a, &'a Pos, W>>,
    node: PathNode<Pos, W>,
    edges: &Vec<Edge<Pos, W>>,
) {
}

fn init_frontier<'a, Pos, W>(
    start: &Pos,
    graph: &Graph<Pos, W>,
) -> HashMap<&'a Pos, PathNode<'a, &'a Pos, W>>
where
    Pos: std::hash::Hash + std::cmp::Eq,
    W: Default,
{
    let frontier = HashMap::new();

    match graph.get(&start) {
        Some(edges) => {
            add_edges(
                &frontier,
                PathNode {
                    pos: start,
                    path: vec![],
                    weight: W::default(),
                },
                edges,
            );
            frontier
        }
        None => frontier,
    }
}

fn init_state<'a, Pos, W>(start: &'a Pos, graph: &'a Graph<Pos, W>) -> State<'a, Pos, W>
where
    Pos: std::hash::Hash + std::cmp::Eq,
    W: Default,
{
    State {
        visited: HashSet::from([start]),
        frontier: init_frontier(start, graph),
    }
}

pub fn shortest_path<Pos, W>(start: Pos, end: Pos, graph: Graph<Pos, W>) -> (Vec<Pos>, W)
where
    Pos: std::hash::Hash + std::cmp::Eq,
    W: Default,
{
    let state: State<Pos, W> = init_state(&start, &graph);
    (vec![], W::default())
}
