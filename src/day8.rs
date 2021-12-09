use std::{str::FromStr, fmt::Display, ops::RangeBounds};


/// 7-segment display segements' states
///
///  aaaa
/// b    c
/// b    c
///  dddd
/// e    f
/// e    f
///  gggg
#[derive(Debug, Default, Clone, Copy)]
pub struct Segments(u8);

impl Segments {
    fn a(&self) -> bool { (self.0 & (1 << 0)) != 0 }
    fn b(&self) -> bool { (self.0 & (1 << 1)) != 0 }
    fn c(&self) -> bool { (self.0 & (1 << 2)) != 0 }
    fn d(&self) -> bool { (self.0 & (1 << 3)) != 0 }
    fn e(&self) -> bool { (self.0 & (1 << 4)) != 0 }
    fn f(&self) -> bool { (self.0 & (1 << 5)) != 0 }
    fn g(&self) -> bool { (self.0 & (1 << 6)) != 0 }

    fn contains(&self, other: &Self) -> bool {
        (self.0 & other.0) == other.0
    }

    fn contained_in(&self, other: &Self) -> bool {
        other.contains(self)
    }

    fn count(&self) -> u32 {
        self.0.count_ones()
    }

    fn from_digit(digit: u8) -> Self {
        match digit {
            0 => "abcefg",   // 6
            1 => "cf",       // 2
            2 => "acdeg",    // 5
            3 => "acdfg",    // 5
            4 => "bcdf",     // 4
            5 => "abdfg",    // 5
            6 => "abdefg",   // 6
            7 => "acf",      // 3
            8 => "abcdefg",  // 7
            9 => "abcdfg",   // 6
            d => panic!("Must be a decimal digit 0-9: {}", d),
        }.parse().unwrap()
    }

    fn same_count_digits(&self) -> Vec<u8> {
        (0..=9)
            .filter(|d| Segments::from_digit(*d).count() == self.count())
            .collect()
    }
}

impl TryFrom<char> for Segments {
    type Error = char;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        let offset = match c {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            c => return Err(c),
        };
        Ok(Self(1 << offset))
    }
}

impl FromStr for Segments {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut segments = Self::default();
        for c in s.chars() {
            let this = Segments::try_from(c)
                .map_err(|c| format!("Unexpected char: {}", c))?;
            if segments.contains(&this) {
                return Err(format!("Field {} found more than once", c))
            }
            segments.0 |= this.0;
        }
        Ok(segments)
    }
}

impl Display for Segments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.a() { write!(f, "a")? }
        if self.b() { write!(f, "b")? }
        if self.c() { write!(f, "c")? }
        if self.d() { write!(f, "d")? }
        if self.e() { write!(f, "e")? }
        if self.f() { write!(f, "f")? }
        if self.g() { write!(f, "g")? }
        Ok(())
    }
}

pub struct InputRecord {
    pub observations: Vec<Segments>,
    pub task: Vec<Segments>,
}

pub fn load_data<S: AsRef<str>>(lines: &[S]) -> Vec<InputRecord> {
    lines.iter()
        .map(|line| {
            let sides = line.as_ref()
             .split('|')
             .map(str::trim)
             .map(|sides| sides.split_whitespace()
                  .map(|token| token.parse::<Segments>().unwrap())
                  .collect::<Vec<Segments>>()
              ).collect::<Vec<_>>();
             assert_eq!(sides.len(), 2, "There should be 2 sides delimited by |, but got {}", sides.len());
             InputRecord {
                 observations: sides[0].clone(),
                 task: sides[1].clone(),
             }
        }).collect()
}

impl InputRecord {
    // Returns digit corresponding to given segments configuration
    fn solve_one(&self, segments: &Segments) -> Result<u8, ()> {
        // find digits that have the same number of segments as this one
        let same_count = segments.same_count_digits();

        if same_count.len() == 0 || same_count.len() > 1 {
            Err(())
        } else {
            Ok(same_count[0])
        }
    }

    // Returns a list of digits that could be decoded
    fn solve(&self) -> Vec<Result<u8, ()>> {
        self.task.iter()
            .map(|d| self.solve_one(d))
            .collect()
    }
}

pub fn solve_part1(lines: &[InputRecord]) -> usize {
    let considered = [1, 4, 7, 8];
    lines.iter()
        .map(|line| {
            let solution = line.solve();
            solution.iter()
                .filter_map(|digit| digit.ok())
                .filter(|digit| considered.iter().find(|d| *d == digit).is_some())
                .count()

            // line.count_output_digits(&[1, 4, 7, 8])
        }).sum()
}

pub fn print_lines(lines: &[InputRecord]) {
    for line in lines {
        let fmt_part = |digits: &[Segments]| {
            digits.iter()
                .map(|segments| segments.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        };
        println!("{} | {}", fmt_part(&line.observations), fmt_part(&line.task));
    }
}
