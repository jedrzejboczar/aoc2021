use std::{slice::SliceIndex, collections::{HashMap, HashSet}};

pub fn load_data<S: AsRef<str>>(lines: &[S]) -> Heights {
    let grid = lines.iter()
        .map(|line| {
            line.as_ref()
                .chars()
                .map(|c| c.to_string()
                     .parse()
                     .unwrap())
                .collect()
        }).collect();
    Heights { grid }
}

pub struct Heights {
    grid: Vec<Vec<u8>>,
}

#[derive(Debug, Clone, Copy)]
pub enum Side {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
    height: u8,
}

impl From<(usize, usize, u8)> for Point {
    fn from((x, y, height): (usize, usize, u8)) -> Self {
        Self { x, y, height }
    }
}

impl Heights {
    pub fn neighbours(&self, x: usize, y: usize) -> Vec<Point> {
        let sides = [Side::Left, Side::Right, Side::Up, Side::Down];
        sides.iter()
            .filter_map(|side| self.neighbour(x, y, *side))
            .collect()
    }

    fn neighbour(&self, x: usize, y: usize, side: Side) -> Option<Point> {
        let (xn, yn) = match side {
            Side::Left => x.checked_sub(1).map(|xsub| (xsub, y)),
            Side::Right => Some((x + 1, y)),
            Side::Up => y.checked_sub(1).map(|ysub| (x, ysub)),
            Side::Down => Some((x, y + 1)),
        }?;
        self.grid.get(xn)
            .and_then(|row| row.get(yn).copied())
            .map(|height| (xn, yn, height).into())
    }

    fn iter_all<'a>(&'a self) -> impl Iterator<Item=Point> + 'a {
        self.grid.iter()
            .enumerate()
            .flat_map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(y, val)| (x, y, *val).into())
                    // .inspect(|all| println!("{:?}", all))
            })
    }

    fn low_points<'a>(&'a self) -> impl Iterator<Item=Point> + 'a {
        self.iter_all()
            .filter(|p| {
                self.neighbours(p.x, p.y).iter()
                    .all(|n| p.height < n.height)
            })
    }

    fn basin(&self, x: usize, y: usize) -> Vec<Point> {
        // don't care for memory usage
        let mut visited = HashSet::new();
        let mut to_visit = Vec::new();
        to_visit.push((x, y));
        while let Some((x, y)) = to_visit.pop() {
            visited.insert((x, y));
            let height = self.grid[x][y];
            to_visit.extend(
                self.neighbours(x, y)
                    .iter()
                    .filter(|n| n.height >= height && n.height < 9)
                    .map(|n| (n.x, n.y))
                    .filter(|pos| visited.get(pos).is_none())
            );
        }
        visited.into_iter()
            .map(|(x, y)| (x, y, self.grid[x][y]).into())
            .collect()
    }

    fn show(&self, highlighted: &HashSet<(usize, usize)>) {
        for (x, row) in self.grid.iter().enumerate() {
            for (y, col) in row.iter().enumerate() {
                let bold = "\x1b[1m";
                let clear = "\x1b[0m";
                let maybe = |val| {
                    if highlighted.get(&(x, y)).is_some() {
                        val
                    } else {
                        ""
                    }
                };
                // let s = if *col == 9 {
                //     ".".to_string()
                // } else {
                //     col.to_string()
                // };
                let s = col.to_string();
                print!("{}{}{}", maybe(bold), s, maybe(clear));
            }
            println!();
        }
    }
}

pub fn part_1(heights: &Heights, verbose: bool) -> usize {
    if verbose {
        let highlighted = heights.low_points()
            .map(|p| (p.x, p.y))
            .collect();
        heights.show(&highlighted);
    }
    let risk_levels = heights.low_points()
        // .inspect(|(x, y, height)| {
        //     println!("low point: ({}, {}, {}) with neighbours: {}",
        //              x, y, height,
        //              heights.neighbours(*x, *y).iter()
        //                  .map(|h| h.to_string())
        //                  .collect::<Vec<_>>()
        //                  .join(" ")
        //             );
        // })
        .map(|point| (point.height + 1) as usize);
    risk_levels.sum()
}

pub fn part_2(heights: &Heights, verbose: bool) -> usize {
    let mut basins: Vec<_> = heights.low_points()
        .map(|p| heights.basin(p.x, p.y))
        .collect();
    basins.sort_unstable_by_key(|b| b.len());
    let largest3 = &basins[(basins.len() - 3)..];
    assert_eq!(largest3.len(), 3);
    if verbose {
        let mut highlighted = HashSet::new();
        for b in largest3 {
            highlighted.extend(b.iter().map(|p| (p.x, p.y)));
        }
        heights.show(&highlighted);
    }
    let result = largest3.iter()
        .map(|b| b.len())
        .product();
    result
}
