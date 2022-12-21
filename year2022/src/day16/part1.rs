use std::collections::{HashMap, HashSet};

use super::utils::{build_map, parse, Valve, FILE_NAME};

const CORRECT_ANSWER: usize = 0;

pub fn solve() -> Result<(), String> {
    core::do_work(FILE_NAME, CORRECT_ANSWER, get_answer, |a, b| a == b)
}

// #[derive(Debug, PartialEq, Eq)]
// struct ToVisit {
//     pub name: String,
//     pub remaining: HashSet<String>,
//     pub steps: usize,
//     pub dist: usize,
//     pub path: Vec<String>,
// }

// impl Ord for ToVisit {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         other.dist.cmp(&self.dist)
//     }
// }

// impl PartialOrd for ToVisit {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         other.dist.partial_cmp(&self.dist)
//     }
// }

fn print_dists(dists: &HashMap<String, HashMap<String, usize>>) {
    let mut dist_vec: Vec<(&String, &HashMap<String, usize>)> = dists.iter().map(|e| e).collect();

    dist_vec.sort_by(|(n1, _), (n2, _)| n1.cmp(n2));

    dist_vec.iter().for_each(|(name, dist)| {
        let mut v: Vec<(&String, &usize)> = dist.iter().map(|e| e).collect();

        v.sort_by(|(n1, _), (n2, _)| n1.cmp(n2));

        print!("{:?}", name);
        v.iter().for_each(|(_, s)| print!(" {:?}", s));
        println!();
    });
}

struct Config {
    valves: HashMap<String, Valve>,
    dist_map: HashMap<String, HashMap<String, usize>>,
    to_open: Vec<String>,
    to_mask: HashMap<String, usize>,
    from_mask: HashMap<usize, String>,
}

#[derive(Debug)]
struct Node {
    cur: String,
    seen: HashSet<String>,
    time: usize,
}

// struct TargetInfo {
//     mask: usize,
//     rate: usize,
//     dist: usize,
// }

// type Cache = HashMap<Node, usize>;

fn get_answer(lines: Vec<String>) -> usize {
    let valves = parse(lines);
    let dist_map = build_map(&valves);
    let max_flow = valves.iter().fold(0, |acc, (_, valve)| acc + valve.rate) * 30;
    let to_open = valves
        .iter()
        .filter(|(_, v)| v.rate > 0)
        .map(|(name, _)| name.clone())
        .collect::<Vec<String>>();
    let (to_mask, from_mask) = assign_masks(&to_open);
    let cfg = Config {
        valves,
        dist_map,
        to_open,
        to_mask,
        from_mask,
    };

    let dist = walk(
        &cfg,
        Node {
            cur: String::from("AA"),
            seen: HashSet::new(),
            time: 0,
        },
    );

    max_flow - dist
}

fn get_dist(
    dists_map: &HashMap<String, HashMap<String, usize>>,
    from: &String,
    to: &String,
) -> usize {
    match dists_map.get(from) {
        Some(dists) => match dists.get(to) {
            Some(dist) => *dist,
            None => {
                println!("No distance for {:?} -> {:?}", from, to);
                todo!()
            }
        },
        None => {
            println!("No distance for {:?} -> {:?}", from, to);
            todo!()
        }
    }
}

fn assign_masks(names: &Vec<String>) -> (HashMap<String, usize>, HashMap<usize, String>) {
    let mut shift = 0;
    let mut to = HashMap::new();
    let mut from = HashMap::new();

    names.iter().for_each(|name| {
        to.insert(name.clone(), 1 << shift);
        from.insert(1 << shift, name.clone());
        shift += 1;
    });

    (to, from)
}

fn walk(cfg: &Config, node: Node) -> usize {
    let mut to_visit: Vec<Node> = get_to_visit(cfg, &node);

    while to_visit.len() > 0 {
        // println!("TIMES: {:?}", to_visit.iter().map(|n| n.time).collect::<Vec<usize>>());

        let mut next_nodes: Vec<Node> = vec![];

        for item in to_visit {
            let mut new_to_visit = get_to_visit(cfg, &item);
            next_nodes.append(&mut new_to_visit);
        }

        to_visit = next_nodes;
    }

    0
}

fn to_node(cfg: &Config, node: &Node, target: &String) -> Node {
    let mut new_seen = node.seen.clone();
    let dist = get_dist(&cfg.dist_map, &node.cur, target);

    new_seen.insert(String::from(target));

    Node {
        cur: String::from(target),
        seen: new_seen,
        time: node.time + dist + 1,
    }
}

fn get_to_visit(cfg: &Config, node: &Node) -> Vec<Node> {
    cfg.to_open
        .iter()
        .filter(|target| !node.seen.contains(*target))
        .map(|target| String::from(target))
        .map(|target| to_node(cfg, node, &target))
        .filter(|node| node.time < 30)
        .collect()
}

// fn new_walk(cfg: &Config, cache: &mut Cache, node: Node) -> usize {
//     let to_visit = get_to_visit(cfg, node.seen);

//     match cache.get(&node) {
//         Some(v) => *v,
//         _ => {
//             let least = to_visit.iter().fold(usize::MAX, |acc, target| {
//                 match cfg.dist_map.get(&node.name) {
//                     Some(dists) => {
//                         let to_end = process_kid(cfg, cache, &node, target, dists);

//                         if to_end < acc {
//                             to_end
//                         } else {
//                             acc
//                         }
//                     }
//                     None => todo!(),
//                 }
//             });

//             cache.insert(node.clone(), least);

//             least
//         }
//     }
// }

// fn get_info(cfg: &Config, target: &String, dists: &HashMap<String, usize>) -> TargetInfo {
//     match (
//         cfg.to_mask.get(target),
//         cfg.valves.get(target),
//         dists.get(target),
//     ) {
//         (Some(mask), Some(valve), Some(dist)) => TargetInfo {
//             mask: *mask,
//             rate: valve.rate,
//             dist: *dist,
//         },
//         _ => todo!(),
//     }
// }

// fn process_kid(
//     cfg: &Config,
//     cache: &mut Cache,
//     node: &Node,
//     target: &String,
//     dists: &HashMap<String, usize>,
// ) -> usize {
//     let info = get_info(cfg, target, dists);
//     let new_dist = node.dist + info.dist + 1;
//     let to_end = new_walk(
//         cfg,
//         cache,
//         Node {
//             name: target.clone(),
//             seen: node.seen | info.mask,
//             dist: new_dist,
//             path: node.path.clone(),
//         },
//     );

//     let mut value = new_dist * info.rate;

//     if to_end < usize::MAX {
//         value += to_end;
//     }

//     value
// }

// fn walk(cfg: &Config, node: Node) -> usize {
//     match cfg.dist_map.get(&node.name) {
//         Some(dists) => {
//             let to_visit = get_to_visit(cfg, node.seen);

//             to_visit.iter().fold(usize::MAX, |least, target| {
//                 match (
//                     cfg.to_mask.get(*target),
//                     dists.get(*target),
//                     cfg.valves.get(*target),
//                 ) {
//                     (Some(mask), Some(target_dist), Some(valve)) => {
//                         let new_seen = node.seen | mask;
//                         let new_dist = node.dist + target_dist + 1;
//                         let value = new_dist * valve.rate;
//                         let mut new_path = node.path.clone();

//                         new_path.push(String::from(*target));

//                         let subtotal = walk(
//                             cfg,
//                             Node {
//                                 name: String::from(*target),
//                                 seen: new_seen,
//                                 dist: new_dist,
//                                 path: new_path,
//                             },
//                         );

//                         let mut new_value = value;

//                         if subtotal < usize::MAX {
//                             new_value += subtotal;
//                         }

//                         if new_value < least {
//                             new_value
//                         } else {
//                             least
//                         }
//                     }
//                     _ => least,
//                 }
//             })
//         }
//         None => todo!(),
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let lines = vec![
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB".to_string(),
            "Valve BB has flow rate=13; tunnels lead to valves CC, AA".to_string(),
            "Valve CC has flow rate=2; tunnels lead to valves DD, BB".to_string(),
            "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE".to_string(),
            "Valve EE has flow rate=3; tunnels lead to valves FF, DD".to_string(),
            "Valve FF has flow rate=0; tunnels lead to valves EE, GG".to_string(),
            "Valve GG has flow rate=0; tunnels lead to valves FF, HH".to_string(),
            "Valve HH has flow rate=22; tunnel leads to valve GG".to_string(),
            "Valve II has flow rate=0; tunnels lead to valves AA, JJ".to_string(),
            "Valve JJ has flow rate=21; tunnel leads to valve II".to_string(),
        ];

        // 0 0 A
        // 0 0 A,D

        // A 0
        // B 13
        // C 2
        // D 20
        // E 3
        // F 0
        // G 0
        // H 22
        // I 0
        // J 21

        //   A B C D E F G H I J
        // A 0 1 2 1 2 3 4 5 1 2
        // B 1 0 1 2 3 4 5 6 2 3
        // C 2 1 0 1 2 3 4 5 3 4
        // D 1 2 1 0 1 2 3 4 2 3
        // E 2 3 2 1 0 1 2 3 3 4
        // F 3 4 3 2 1 0 1 2 4 5
        // G 4 5 4 3 2 1 0 1 5 6
        // H 5 6 5 4 3 2 1 0 6 7
        // I 1 2 3 2 3 4 5 6 0 1
        // J 2 3 4 3 4 5 6 7 1 0

        assert_eq!(get_answer(lines), 1651)
    }
}
