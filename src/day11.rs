use std::{ops::{Deref, DerefMut}, collections::{HashSet, VecDeque}};

use aoc::grid::{Grid, GridPoint, INVERSE};

pub struct OctopusGrid(Grid<u8>);

impl Deref for OctopusGrid {
    type Target = Grid<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for OctopusGrid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl OctopusGrid {
    pub fn new(lines: &[String]) -> Self {
        let width = lines[0].len();
        let nums = lines.iter()
            .flat_map(|l| l.chars())
            .map(|c| c.to_string().parse().expect("Failed to parse a number"));
        let grid = Grid::from(nums, width).expect("Wrong number of cells");
        Self(grid)
    }

    pub fn step(&mut self, verbose: bool) -> HashSet<(usize, usize)> {
        let fmt = |val: &u8| if *val <= 9 {
            val.to_string()
        } else {
            "*".to_string()
        };

        // increase all octopuses' energy levels by 1
        self.iter_mut()
            .for_each(|p| *p.value += 1);
        if verbose {
            let hl = |pos| self[pos] > 9;
            println!("###\n{}", self.to_string(INVERSE, hl, fmt));
        }

        // ones with level >9 flash: level of all neighbours += 1 (also diagonal)
        // each can flash at most once
        let seeds = self.iter()
            .filter(|p| p.value > 9)
            .map(|p| p.pos());

        let mut to_flash = VecDeque::from_iter(seeds);
        let mut flashed = HashSet::new();

        while let Some(pos) = to_flash.pop_front() {
            // this one flashes if not already flashed
            if flashed.insert(pos) {
                // increase values of all neighbours
                for n in self.neighbours(pos, true) {
                    self[n.pos()] += 1;
                    // if a neighbour exceeded 9 it also will flash
                    if self[n.pos()] > 9 {
                        to_flash.push_back(n.pos());
                    }
                }
            }
        }

        // after flashing decrease levels of flashing ones to 0
        for p in self.iter_mut() {
            if *p.value > 9 {
                // assert!(flashed.contains(&(p.x, p.y)), "{:?}", p);
                *p.value = 0;
            }
        }

        flashed
    }

    pub fn part_1(&mut self, steps: usize, verbose: bool) -> usize {
        let mut flashes = 0;
        if verbose {
            let hl = |_| false;
            let fmt = |val: &u8| val.to_string();
            println!("Before:\n{}", self.to_string(INVERSE, hl, fmt));
        }
        for step in 1..=steps {
            let flashed = self.step(verbose);
            flashes += flashed.len();
            if verbose {
                let hl = |pos| flashed.contains(&pos);
                let fmt = |val: &u8| val.to_string();
                println!("\nAfter step {}:\n{}", step, self.to_string(INVERSE, hl, fmt));
            }
        }
        flashes
    }

    pub fn part_2(&mut self, verbose: bool) -> usize {
        let mut step = 1;
        loop {
            let flashes = self.step(verbose).len();
            if flashes == self.width * self.height() {
                return step;
            }
            step += 1;
        }
    }
}
