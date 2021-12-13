use std::{ops::{Index, IndexMut}, collections::{HashSet, VecDeque}};

pub const BOLD: &str = "\x1b[1m";
pub const INVERSE: &str = "\x1b[0;30m\x1b[47m";
pub const CLEAR: &str = "\x1b[0m";

type Position = (usize, usize);

#[derive(Debug, Clone)]
pub struct GridPoint<T> {
    pub x: usize,
    pub y: usize,
    pub value: T,
}

#[derive(Debug)]
pub struct GridPointMut<'a, T> {
    pub x: usize,
    pub y: usize,
    pub value: &'a mut T,
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
    cells: Vec<T>,  // 2D grid, row-major order
    pub width: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum Side {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Side {
    const NON_DIAGONAL: [Side; 4] = [Side::N, Side::E, Side::S, Side::W];
    const ALL: [Side; 8] = [Side::N, Side::NE, Side::E, Side::SE, Side::S, Side::SW, Side::W, Side::NW];
}

impl<T> GridPoint<T> {
    pub fn pos(&self) -> Position {
        (self.x, self.y)
    }
}

impl<T> From<(usize, usize, T)> for GridPoint<T> {
    fn from((x, y, value): (usize, usize, T)) -> Self {
        Self { x, y, value }
    }
}

impl<T: Copy> Grid<T> {
    /// Create a grid from values, if values count is wrong returns the reminder count.
    pub fn from<I>(values: I, width: usize) -> Result<Self, usize>
        where I: IntoIterator<Item = T>
    {
        let cells: Vec<_> = values.into_iter().collect();
        let remaining = cells.len() % width;
        if remaining != 0 {
            Err(remaining)
        } else {
            Ok(Self { cells, width })
        }
    }

    pub fn height(&self) -> usize {
        self.cells.len() / self.width
    }

    pub fn linear_index(&self, (x, y): Position) -> usize {
        assert!(x < self.width && y < self.height());
        y * self.width + x
    }


    pub fn iter<'a>(&'a self) -> impl Iterator<Item=GridPoint<T>> + 'a {
        self.cells.iter()
            .enumerate()
            .map(|(i, v)| GridPoint { x: i % self.width, y: i / self.width, value: *v })
    }

    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item=GridPointMut<'a, T>> + 'a {
        self.cells.iter_mut()
            .enumerate()
            .map(|(i, v)| GridPointMut { x: i % self.width, y: i / self.width, value: v })
    }

    pub fn to_string<S, C, F>(&self, highlight: S, highlight_cond: C, fmt: F) -> String
        where
            S: AsRef<str>,
            C: Fn(Position) -> bool,
            F: Fn(&T) -> String,
    {
        let mut s = String::new();
        for y in 0..self.height() {
            if y != 0 {
                s += "\n";
            }
            for x in 0..self.width {
                let hl = highlight_cond((x, y));
                if hl {
                    s += highlight.as_ref();
                }
                s += &fmt(&self[(x, y)]);
                if hl {
                    s += CLEAR;
                }
            }
        }
        s
    }

    pub fn neighbours(&self, pos: Position, diagonal: bool) -> Vec<GridPoint<T>> {
        let sides = if diagonal {
            &Side::ALL[..]
        } else {
            &Side::NON_DIAGONAL[..]
        };
        sides.iter()
            .filter_map(|side| self.neighbour(pos, *side))
            .collect()
    }

    fn offset(side: Side) -> (isize, isize) {
        // north is up, but Y increases downwards (like printed lines)
        match side {
            Side::N  => ( 0, -1),
            Side::NE => ( 1, -1),
            Side::E  => ( 1,  0),
            Side::SE => ( 1,  1),
            Side::S  => ( 0,  1),
            Side::SW => (-1,  1),
            Side::W  => (-1,  0),
            Side::NW => (-1, -1),
        }
    }

    pub fn neighbour(&self, (x0, y0): Position, side: Side) -> Option<GridPoint<T>> {
        let (dx, dy) = Self::offset(side);
        let (x, y) = (x0 as isize + dx, y0 as isize + dy);
        if x < 0 || y < 0 {
            return None;
        }
        let (x, y) = (x as usize, y as usize);
        if x > self.width - 1 || y > self.height() - 1 {
            return None;
        }
        self.cells.get(self.linear_index((x, y)))
            .map(|v| GridPoint { x, y, value: *v })
    }

    /// Visit points starting from `pos` based on `condition`
    pub fn expand_from<F, C>(&mut self, pos: Position, diagonal: bool, mut visit: F, condition: C) -> Vec<GridPoint<T>>
        where
            F: FnMut(&mut Self, Position),
            C: Fn(Position, &GridPoint<T>) -> bool
    {
        // don't care for memory usage
        let mut visited = HashSet::new();
        let mut to_visit = Vec::new();

        to_visit.push(pos);
        while let Some(pos) = to_visit.pop() {
            visit(self, pos);
            visited.insert(pos);

            // let value = self[pos];
            to_visit.extend(
                self.neighbours(pos, diagonal)
                    .iter()
                    .filter(|p| condition(pos, p))
                    .map(|n| (n.x, n.y))
                    .filter(|pos| visited.get(pos).is_none())
            );
        }
        visited.into_iter()
            .map(|(x, y)| (x, y, self[(x, y)]).into())
            .collect()
    }

    pub fn visit_mut<S, F, I>(&mut self, seeds: S, mut visitor: F) -> HashSet<Position>
        where
            S: Iterator<Item = Position>,
            F: FnMut(&mut Self, Position) -> I,
            I: IntoIterator<Item = Position>
    {
        let mut visited = HashSet::new();
        let mut to_visit: VecDeque<_> = seeds.into_iter().collect();

        while let Some(pos) = to_visit.pop_front() {
            if !visited.insert(pos) {
                continue;
            }
            to_visit.extend(visitor(self, pos));
        }

        visited
    }
}

impl<T: Copy> Index<Position> for Grid<T> {
    type Output = T;

    fn index(&self, pos: Position) -> &Self::Output {
        let i = self.linear_index(pos);
        &self.cells[i]
    }
}

impl<T: Copy> IndexMut<Position> for Grid<T> {
    fn index_mut(&mut self, pos: Position) -> &mut Self::Output {
        let i = self.linear_index(pos);
        &mut self.cells[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid() -> Grid<u8> {
        // let vals = [
        //     [1, 2, 3],
        //     [4, 5, 6],
        //     [7, 8, 9u8]
        // ];
        // Grid::from(vals.into_iter().flatten(), 3).unwrap()
        let vals = [
            1, 2, 3,
            4, 5, 6,
            7, 8, 9u8
        ];
        Grid::from(vals.into_iter(), 3).unwrap()
    }

    fn values(neighbours: Vec<GridPoint<u8>>) -> Vec<u8> {
        neighbours.iter()
            .map(|n| n.value)
            .collect()
    }

    #[test]
    fn get_neighbour_n() {
        let g = grid();
        assert_eq!(g.neighbour((1, 1), Side::N).unwrap().value, 2);
    }

    #[test]
    fn get_neighbour_e() {
        let g = grid();
        assert_eq!(g.neighbour((1, 1), Side::E).unwrap().value, 6);
    }

    #[test]
    fn test_neighbours_mid_all() {
        let g = grid();
        let n = g.neighbours((1, 1), true);
        assert_eq!(values(n), vec![2, 3, 6, 9, 8, 7, 4, 1]);
    }

    #[test]
    fn test_neighbours_mid_diagonal() {
        let g = grid();
        let n = g.neighbours((1, 1), false);
        assert_eq!(values(n), vec![2, 6, 8, 4]);
    }

    #[test]
    fn test_neighbours_corner_all() {
        let g = grid();
        let n = g.neighbours((0, 2), true);
        assert_eq!(values(n), vec![4, 5, 8]);
    }


    #[test]
    fn test_neighbours_corner_diagonal() {
        let g = grid();
        let n = g.neighbours((0, 2), false);
        assert_eq!(values(n), vec![4, 8]);
    }
}

