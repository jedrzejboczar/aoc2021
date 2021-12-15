use std::{io::{Read, BufReader, BufRead}, collections::{BinaryHeap, HashMap, HashSet}};

use aoc::grid::{Grid, BOLD, Position};

fn load_grid(input: impl Read) -> Grid<u32> {
    let vals: Vec<_> = BufReader::new(input)
        .lines()
        .map(Result::unwrap)
        .map(|l| l.chars()
             .map(|c| c.to_digit(10).unwrap())
             .collect::<Vec<_>>())
        .collect();
    let width = vals[0].len();
    Grid::from(vals.into_iter().flatten(), width).unwrap()
}

struct PathNode {
    pos: Position,
    // from: Position,
    goal: Position,
}


impl PathNode {
    // fn distance(&self) -> usize {
    //     let abs_diff = |a, b| if a > b { a - b } else { b - a };
    //     let dx = abs_diff(self.pos.0, self.goal.0);
    //     let dy = abs_diff(self.pos.1, self.goal.1);
    //     dx + dy  // manhatan distance
    // }

    fn distance(&self) -> usize {
        let abs_diff = |a, b| if a > b { a - b } else { b - a };
        let dx = abs_diff(self.pos.0, self.goal.0);
        let dy = abs_diff(self.pos.1, self.goal.1);
        ((dx as f32).powi(2) + (dy as f32).powi(2)).sqrt().floor() as usize
    }
}

impl PartialEq for PathNode {
    fn eq(&self, other: &Self) -> bool {
        self.distance() == other.distance()
    }
}

impl Eq for PathNode {}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.distance().cmp(&other.distance()))
    }
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance().cmp(&other.distance())
    }
}

fn a_star(grid: &Grid<u32>, start: Position, goal: Position) -> Option<Vec<Position>> {
    let mut nodes = BinaryHeap::new();
    let mut nodes_set = HashSet::new();
    // nodes.push(Reverse(PathNode { pos: start, /* from: start, */ goal }));
    nodes.push(PathNode { pos: start, /* from: start, */ goal });
    nodes_set.insert(start);

    let mut predecessors: HashMap<Position, Position> = HashMap::new();
    let mut cumulative_risks: HashMap<Position, usize> = HashMap::new();
    cumulative_risks.insert(start, 0);

    while let Some(node) = nodes.pop() {
        // let node = node.0;  // remove Reverse wrapper
        nodes_set.remove(&node.pos);

        if node.pos == goal {
            // reconstruct path and return
            let mut path = Vec::new();
            path.push(goal);
            let mut current = goal;
            while current != start {
                current = predecessors[&current];
                path.push(current);
            }
            path.reverse();
            return Some(path);
        }

        for n in grid.neighbours(node.pos, false) {
            let risk = grid[n.pos()];  // risk of entering the neighbour
            // println!("risk {} {:?}", risk, n.pos());
            let cost = cumulative_risks.get(&node.pos).unwrap_or(&usize::MAX) + risk as usize;
            if cost < *cumulative_risks.get(&n.pos()).unwrap_or(&usize::MAX) {
                cumulative_risks.insert(n.pos(), cost);
                predecessors.insert(n.pos(), node.pos);
                if !nodes_set.contains(&n.pos()) {
                    nodes_set.insert(n.pos());
                    nodes.push(PathNode { pos: n.pos(), goal });
                }
            }
        }

        // let heap_str = nodes.iter().fold(String::new(), |s, node| s + &format!(" {:?}", node.0.pos));
        // println!("Heap ({:?}) {}", nodes.peek().map(|n| n.0.pos), heap_str);
    }

    None
}

fn solve(grid: Grid<u32>, verbose: bool) -> usize {
    let (start, goal) =  ((0, 0), (grid.width - 1, grid.width - 1));
    let path = a_star(&grid, start, goal).unwrap();
    let mut path_nodes = HashSet::new();
    path.iter().for_each(|pos| { path_nodes.insert(pos); });

    if verbose {
        println!("{}", grid.to_string(BOLD, |pos| path_nodes.contains(&pos), |val| val.to_string()));
    }

    // println!("Path: {:?}", path);
    let total_risk = path.iter()
        .skip(1)  // ignore start
        .map(|pos| grid[*pos] as usize)
        .sum();

    total_risk
}

pub fn part_1(input: impl Read, verbose: bool) -> usize {
    let grid = load_grid(input);
    // println!("{}", grid.to_string(BOLD, |_| false, |val| val.to_string()));

    solve(grid, verbose)
}


pub fn part_2(input: impl Read, verbose: bool) -> usize {
    let grid = load_grid(input);

    // expand grid
    let new_width = 5 * grid.width;
    let n = new_width * grid.height() * 5;
    let mut new = Grid::from([0].iter().cycle().take(n).copied(), new_width).unwrap();

    let points: Vec<_> = grid.iter()
        .map(|p| (p.x, p.y, p.value))
        .collect();

    for p in &points {
        // replicate the point module 10
        for i in 0..5 {
            for j in 0..5 {
                let pos = (p.0 + i * grid.width, p.1 + j * grid.width);
                let val = p.2 + (i + j) as u32;
                new[pos] = if val > 9 { val % 10 + 1 } else { val };
            }
        }
    }

    solve(new, verbose)
}
