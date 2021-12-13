use std::{io::{Read, BufReader, BufRead}, ops::RangeBounds, collections::HashSet};

#[derive(Debug, Clone)]
struct Dots {
    dots: HashSet<(usize, usize)>,  // don't care for order, auto merge dots
    folds: Vec<Fold>,
}

#[derive(Debug, Clone, Copy)]
enum Fold {
    Y(usize),
    X(usize),
}

impl Dots {
    pub fn new(input: impl Read) -> Self {
        let lines: Vec<_> = BufReader::new(input)
            .lines()
            .map(|l| l.unwrap())
            .collect();
        let dots: HashSet<_> = lines.iter()
            .take_while(|l| !l.trim().is_empty())
            .map(|l| {
                let mut parts = l.trim().split(",");
                let x = parts.next().unwrap().parse::<usize>().unwrap();
                let y = parts.next().unwrap().parse::<usize>().unwrap();
                (x, y)
            }).collect();
        let folds = lines.iter()
            .rev()
            .take_while(|l| !l.trim().is_empty())
            .map(|l| {
                assert!(l.contains("fold along "), "{}", l);
                let mut parts = l.trim().split_whitespace()
                    .nth(2)
                    .unwrap()
                    .split('=');
                match parts.next().unwrap() {
                    "x" => Fold::X(parts.next().unwrap().parse().unwrap()),
                    "y" => Fold::Y(parts.next().unwrap().parse().unwrap()),
                    s => panic!("Unexpected: {}", s),
                }
            }).collect();

        Self { dots, folds }
    }

    pub fn folded(&self) -> Option<Self> {
        let mut new = self.clone();
        let fold = new.folds.pop()?;
        // mirror all the dots
        new.dots = new.dots.iter()
            .map(|(x, y)| {
                match fold {
                    Fold::Y(y0) => {
                        if *y > y0 {
                            (*x, y0 - (y - y0))
                        } else if *y < y0 {
                            (*x, *y)
                        } else {
                            panic!("Fold at dot: y = {}", y0);
                        }
                    },
                    Fold::X(x0) => {
                        if *x > x0 {
                            (x0 - (x - x0), *y)
                        } else if *x < x0 {
                            (*x, *y)
                        } else {
                            panic!("Fold at dot: y = {}", x0);
                        }
                    },
                }
            }).collect();
        Some(new)
    }

    pub fn count(&self) -> usize {
        self.dots.len()
    }

    pub fn print(&self) {
        let max_x = *self.dots.iter().map(|(x, _)| x).max().unwrap();
        let max_y = *self.dots.iter().map(|(_, y)| y).max().unwrap();
        for y in 0..=max_y {
            for x in 0..=max_x {
                let s = if self.dots.contains(&(x, y)) {
                    "#"
                } else {
                    "."
                };
                print!("{}", s);
            }
            println!();
        }
    }
}

pub fn part_1(input: impl Read) -> usize {
    let dots = Dots::new(input);
    // println!("-----");
    // dots.print();

    let dots = dots.folded().unwrap();
    // println!("-----");
    // dots.print();

    dots.count()
}

pub fn part_2(input: impl Read) -> usize {
    let mut dots = Dots::new(input);

    while let Some(new) = dots.folded() {
        dots = new;
    }

    dots.print();

    0
}
