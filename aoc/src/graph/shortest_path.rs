use std::collections::{HashMap, HashSet};

use super::{Edge, Graph};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let graph = HashMap::from([
            (
                1,
                vec![
                    Edge {
                        target: 2,
                        weight: 7,
                    },
                    Edge {
                        target: 3,
                        weight: 9,
                    },
                    Edge {
                        target: 6,
                        weight: 14,
                    },
                ],
            ),
            (
                2,
                vec![
                    Edge {
                        target: 1,
                        weight: 7,
                    },
                    Edge {
                        target: 3,
                        weight: 10,
                    },
                    Edge {
                        target: 4,
                        weight: 15,
                    },
                ],
            ),
            (
                3,
                vec![
                    Edge {
                        target: 1,
                        weight: 9,
                    },
                    Edge {
                        target: 2,
                        weight: 10,
                    },
                    Edge {
                        target: 4,
                        weight: 11,
                    },
                    Edge {
                        target: 6,
                        weight: 2,
                    },
                ],
            ),
            (
                4,
                vec![
                    Edge {
                        target: 2,
                        weight: 15,
                    },
                    Edge {
                        target: 3,
                        weight: 11,
                    },
                    Edge {
                        target: 5,
                        weight: 6,
                    },
                ],
            ),
            (
                5,
                vec![
                    Edge {
                        target: 4,
                        weight: 6,
                    },
                    Edge {
                        target: 6,
                        weight: 9,
                    },
                ],
            ),
            (
                6,
                vec![
                    Edge {
                        target: 1,
                        weight: 14,
                    },
                    Edge {
                        target: 3,
                        weight: 2,
                    },
                    Edge {
                        target: 5,
                        weight: 9,
                    },
                ],
            ),
        ]);

        let (_, weight) = shortest_path(1, 5, graph);

        assert_eq!(20, weight);
    }
}
