use core::fmt;
use std::{str::FromStr, num::ParseIntError, collections::HashMap, fmt::Display};

use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Line {
    start: (usize, usize),
    end: (usize, usize),
}

impl Line {
    pub fn is_horizontal(&self) -> bool {
        self.delta().1 == 0
    }

    pub fn is_vertical(&self) -> bool {
        self.delta().0 == 0
    }

    pub fn is_diagonal(&self) -> bool {
        let (dx, dy) = self.delta();
        dx == dy
    }

    fn delta(&self) -> (usize, usize) {
        let dx = self.start.0 as isize - self.end.0 as isize;
        let dy = self.start.1 as isize - self.end.1 as isize;
        (dx.abs() as usize, dy.abs() as usize)
    }

    pub fn points(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        // let in_order = |(a, b): (usize, usize)| (a.min(b), a.max(b));
        let (dx, dy) = self.delta();
        let n = dx.max(dy) + 1;

        let coord = |a: usize, b: usize| -> Box<dyn Iterator<Item = usize>> {
            if a < b {
                Box::new((a ..= b).cycle())
            } else {
                Box::new((b ..= a).rev().cycle())
            }
        };

        let x = coord(self.start.0, self.end.0);
        let y = coord(self.start.1, self.end.1);

        // println!("{:?}, (dx, dy) = {:?}, n = {}", self, (dx, dy), n);
        x.zip(y)
            // .inspect(|xy| println!(" -> {:?}", xy))
            .take(n)
    }
}

pub struct VentsCount {
    counts: HashMap<(usize, usize), usize>,
}

impl VentsCount {
    pub fn non_diagonal(lines: &[Line]) -> Self {
        let lines = lines.iter()
            .filter(|l| !l.is_diagonal());
        Self::from_lines(lines)
    }

    pub fn all(lines: &[Line]) -> Self {
        Self::from_lines(lines.iter())
    }

    fn from_lines<'a>(lines: impl Iterator<Item = &'a Line>) -> Self {
        let mut counts = HashMap::new();
        for line in lines {
            for (x, y) in line.points() {
                let count = counts.entry((x, y)).or_insert(0);
                *count += 1;
            }
        }
        Self { counts }
    }

    pub fn dangerous_area_count(&self) -> usize {
        self.counts.iter()
            .filter(|((_x, _y), n)| **n >= 2)
            .count()
    }
}

impl Display for VentsCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_x = *self.counts.keys().map(|(x, _y)| x).max().ok_or(fmt::Error)?;
        let max_y = *self.counts.keys().map(|(_x, y)| y).max().ok_or(fmt::Error)?;

        for y in 0 ..= max_y {
            if y != 0 {
                writeln!(f, "")?;
            }
            for x in 0 ..= max_x {
                let count = self.counts.get(&(x, y));
                let s =  count.map(|c| c.to_string()).unwrap_or(".".to_string());
                write!(f, "{}", s)?;
            }
        }

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum ParseLineError {
    #[error("Malformed line: {0}")]
    MalformedLine(String),
    #[error("Malformed line part: {0}")]
    MalformedPart(String),
    #[error("Invalid number")]
    ParseNumError(#[from] ParseIntError),
}

impl FromStr for Line {
    type Err = ParseLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("->");

        let start = parts.next().ok_or(ParseLineError::MalformedLine(s.to_string()))?;
        let end = parts.next().ok_or(ParseLineError::MalformedLine(s.to_string()))?;
        if parts.next().is_some() {
            return Err(ParseLineError::MalformedLine(s.to_string()));
        }

        let parse_part = |part: &str| -> Result<(usize, usize), ParseLineError> {
            let mut nums = part.trim().split(',')
                .map(|token| token.parse::<usize>().map_err(|e| ParseLineError::from(e)));
            let x = nums.next().ok_or(ParseLineError::MalformedPart(part.to_string()))??;
            let y = nums.next().ok_or(ParseLineError::MalformedPart(part.to_string()))??;
            Ok((x, y))
        };

        let start = parse_part(start)?;
        let end = parse_part(end)?;

        Ok(Self { start, end })
    }
}

