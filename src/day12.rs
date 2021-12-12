// use aoc::graph;
//
// type Graph = graph::Graph<Cave>;

use std::collections::VecDeque;
use std::fmt::Display;
use std::hash::Hash;
use std::{str::FromStr, collections::HashSet};

use petgraph::graph::UnGraph;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Cave {
    Start,
    End,
    Big(String),
    Small(String),
}

type G = UnGraph<Cave, ()>;

#[derive(Debug)]
pub struct Graph(G);

#[derive(Debug, Clone)]
struct Path {
    caves: Vec<Cave>,
}

#[derive(Debug, Clone)]
struct SearchPath<I> {
    nodes: Vec<I>,
    do_not_visit: HashSet<I>,
    // small_cells_count: HashMap<I, usize>,
    visited_small_cell_twice: Option<I>,
}

impl<I: Clone + Hash + Eq> SearchPath<I> {
    pub fn new(node: I) -> Self {
        Self {
            nodes: vec![node.clone()],
            do_not_visit: [node].into(),
            visited_small_cell_twice: None,
            // small_cells_count: HashMap::new(),
        }
    }
}

impl Graph {
    pub fn new(lines: &[String]) -> Self {
        let mut graph = UnGraph::new_undirected();

        for line in lines {
            let mut tokens = line.split("-");
            let first: Cave = tokens.next().unwrap().parse().unwrap();
            let second: Cave = tokens.next().unwrap().parse().unwrap();
            assert!(tokens.next().is_none(), "{}", line);

            let mut get_or_add = |cave| {
                graph.node_indices()
                    .find(|i| graph[*i] == cave)
                    .unwrap_or_else(|| {
                        graph.add_node(cave)
                    })
            };

            let first = get_or_add(first);
            let second = get_or_add(second);
            graph.add_edge(first, second, ());
        }

        Self(graph)
    }

    fn find_paths(&self, part_2: bool) -> Vec<Path> {
        // Store unfinshed paths in a queue
        let mut searches: VecDeque<SearchPath<_>> = VecDeque::new();
        let mut paths: Vec<Path> = Vec::new();

        let start = self.0.node_indices().find(|i| self.0[*i] == Cave::Start).unwrap();
        searches.push_back(SearchPath::new(start));

        while let Some(search) = searches.pop_front() {
            let end = search.nodes.last().unwrap();
            for nb in self.0.neighbors(*end) {
                // If we declared this node as do-not-visit then break the search
                if search.do_not_visit.contains(&nb) {
                    continue;
                }

                // Generate new search from this one
                let mut new = search.clone();

                // Don't go into small caves twice
                if let Cave::Small(_) = self.0[nb] {
                    if !part_2 {
                        new.do_not_visit.insert(nb);
                    } else {
                        // In part 2 we can visit one small cell twice
                        // If we visited this one alredy...
                        let already_visited = new.nodes.iter().filter(|n| **n == nb).count();
                        match already_visited {
                            // never visited, just go on
                            0 => (),
                            // visited once: if we visited any other twice, then cannot add...
                            1 => {
                                if new.visited_small_cell_twice.is_some() {
                                    continue;
                                }
                                // ...if not then we add it second time and mark this fact
                                new.visited_small_cell_twice = Some(nb);
                                new.do_not_visit.insert(nb);
                            },
                            // visited twice so we cannot add this one more
                            2 => {
                                continue;
                            },
                            _ => panic!("no way!"),
                        }

                    }
                }

                new.nodes.push(nb);

                // Decide if this is the final one or we need to continue searching
                if let Cave::End = self.0[nb] {
                    // Got final path
                    let caves = new.nodes.iter()
                        .map(|i| self.0[*i].clone())
                        .collect();
                    paths.push(Path { caves });
                } else {
                    // Still searching
                    searches.push_back(new);
                }
            }
        }

        paths
    }

    fn print_paths(&self, paths: &[Path]) {
        for path in paths {
            for (i, cave) in path.caves.iter().enumerate() {
                if i != 0 {
                    print!(",");
                }
                print!("{}", cave);
            }
            println!();
        }
    }

    pub fn part_1(&self, verbose: bool) -> usize {
        let paths = self.find_paths(false);
        if verbose {
            self.print_paths(&paths);
        }
        paths.len()
    }

    pub fn part_2(&self, verbose: bool) -> usize {
        let paths = self.find_paths(true);
        if verbose {
            self.print_paths(&paths);
        }
        paths.len()
    }
}

impl FromStr for Cave {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cave = match s {
            "start" => Cave::Start,
            "end" => Cave::End,
            name if name.chars().all(|c| c.is_ascii_uppercase()) => Cave::Big(name.into()),
            name if name.chars().all(|c| c.is_ascii_lowercase()) => Cave::Small(name.into()),
            name => return Err(name.into()),
        };
        Ok(cave)
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cave::Start => write!(f, "start"),
            Cave::End => write!(f, "end"),
            Cave::Big(s) => write!(f, "{}", s),
            Cave::Small(s) => write!(f, "{}", s),
        }
    }
}

// use typed_arena::Arena;
//
// pub struct Graph<'a> {
//     nodes: Arena<Node<'a>>,
//     // start: Cave,
//     // end: Cave,
// }
//
// struct Node<'a> {
//     cave: Cave,
//     nodes: Vec<&'a Node<'a>>
// }
//
// impl<'a> Graph<'a> {
//     // pub fn new(lines: &[String]) -> Self {
//     //     let mut nodes = Arena::new();
//     //     // let mut start = None;
//     //     // let mut end = None;
//     //
//     //     let nodes_ref = &mut nodes;
//     //     for line in lines {
//     //         let mut tokens = line.split("-");
//     //         let first: Cave = tokens.next().unwrap().parse().unwrap();
//     //         let second: Cave = tokens.next().unwrap().parse().unwrap();
//     //         assert!(tokens.next().is_none(), "{}", line);
//     //
//     //         // try to find this node in the graph, if not found add it
//     //         let first = Self::maybe_add(nodes_ref, Node::new(first));
//     //         let second = Self::maybe_add(nodes_ref, Node::new(second));
//     //
//     //         first.nodes.push(second);
//     //         second.nodes.push(first);
//     //     }
//     //
//     //     Self { nodes /* , start: start.unwrap(), end: end.unwrap() */ }
//     // }
//     //
//     // fn maybe_add<'b, 'c, T>(arena: &'b mut Arena<T>, node: T) -> &'c mut T
//     //     where T: Eq
//     // {
//     //     let found = arena.iter_mut()
//     //         .find(|n| **n == node);
//     //     // if not found then add it and return
//     //     let new_node = if let None = found {
//     //         arena.alloc(node)
//     //     } else {
//     //         // find again cause else &mut/& lifetimes would overlap
//     //         arena.iter_mut().find(|n| **n == node).unwrap()
//     //     };
//     //     new_node
//     // }
//
//     pub fn new(lines: &[String]) -> Self {
//         let mut caves = Vec::new();
//         let mut connections = Vec::new();
//
//         for line in lines {
//             let mut tokens = line.split("-");
//             let first: Cave = tokens.next().unwrap().parse().unwrap();
//             let second: Cave = tokens.next().unwrap().parse().unwrap();
//             assert!(tokens.next().is_none(), "{}", line);
//
//             caves.push(first.clone());
//             caves.push(second.clone());
//             connections.push((first, second));
//         }
//
//         let nodes = Arena::new();
//         for cave in &caves {
//             nodes.alloc(Node::new(cave.clone()));
//         }
//
//         let mut nodes = nodes;
//         for (left, right) in &connections {
//             let mut to_connect: Vec<_> = nodes.iter_mut()
//                 .filter(|n| n.cave == *left || n.cave == *right)
//                 .collect();
//             assert_eq!(to_connect.len(), 2);
//             to_connect[0].nodes.push(to_connect[1]);
//
//             // let left_node = nodes.iter_mut()
//             //     .find(|n| n.cave == *left).unwrap();
//             // let right_node = nodes.iter_mut()
//             //     .find(|n| n.cave == *left).unwrap();
//             // left_node.nodes.push(&right_node);
//             // right_node.nodes.push(&left_node);
//         }
//
//         Self { nodes }
//     }
//
// }
//
// impl<'a> Node<'a> {
//     pub fn new(cave: Cave) -> Self {
//         Self { cave, nodes: Vec::new() }
//     }
// }
//
// impl<'a> PartialEq for Node<'a> {
//     fn eq(&self, other: &Self) -> bool {
//         self.cave == other.cave
//     }
// }
//
// impl<'a> Eq for Node<'a> {}
