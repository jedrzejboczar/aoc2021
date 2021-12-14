use std::{collections::HashMap, io::{BufReader, BufRead, Read}};

#[derive(Debug)]
struct Polymerisator {
    polymer: String,
    rules: HashMap<(char, char), String>,
}

impl Polymerisator {
    pub fn new(input: impl Read) -> Self {
        let reader = BufReader::new(input);
        let mut lines = reader.lines().map(Result::unwrap);
        let template = lines.next().unwrap().trim().to_string();
        lines.next().unwrap();

        let mut n = 0;
        let rules: HashMap<_, _> = lines
            .inspect(|_| n += 1)
            .map(|line| {
                let mut parts = line.split("->");
                let from = parts.next().unwrap().trim();
                let to = parts.next().unwrap().trim();
                assert_eq!(from.len(), 2);
                assert_eq!(to.len(), 1);
                ((from.chars().nth(0).unwrap(), from.chars().nth(1).unwrap()), to.into())
            }).collect();

        assert_eq!(rules.len(), n);

        Self { polymer: template, rules }
    }

    pub fn grow(&mut self) {
        // avoid some initial relocations, but don't try to be too smart with size guessing
        let mut new = String::with_capacity(self.polymer.len());
        // add the first one
        new += &self.polymer[..1];
        // iterate over windows
        let pairs = self.polymer.chars().zip(self.polymer.chars().skip(1));
        for (prev, curr) in pairs {
            if let Some(insertion) = self.rules.get(&(prev, curr)) {
                // println!("Insert: {}-{}-{}", prev, insertion, curr);
                new += insertion;
            }
            new.push(curr);
        }
        self.polymer = new;
    }

    pub fn polymer(&self) -> &str {
        &self.polymer
    }

    pub fn counts(&self) -> HashMap<char, usize> {
        self.polymer.chars()
            .fold(HashMap::new(), |mut counts, c| {
                *counts.entry(c).or_insert(0) += 1;
                counts
            })
    }

    pub fn grow_and_get_counts(&self, steps: usize) -> HashMap<char, usize> {
        // Store counts of polymer pairs, we do not need to know the order
        let mut pair_counts: HashMap<(char, char), usize> = HashMap::new();
        let pairs = self.polymer.chars().zip(self.polymer.chars().skip(1));
        for pair in pairs {
            *pair_counts.entry(pair).or_insert(0) += 1;
        }

        // Also update counts while growing
        let mut counts = self.counts();

        // In each step iterate over exisiting paris, possibly grow and update counts
        let mut growth_step = |_i| {
            // Copy because we will be modifying pair_counts
            let previous = pair_counts.clone();
            let prev_pairs = previous.iter()
                .filter(|(_, count)| **count > 0)
                .map(|(pair, _)| *pair);
            for pair in prev_pairs {
                if let Some(insertion) = self.rules.get(&pair) {
                    // All counts are increased by the number of pairs of this type
                    let n = *previous.get(&pair).unwrap();
                    // for _ in 0..n {
                    //     println!("Insert: {}-{}-{}", pair.0, insertion, pair.1);
                    // }
                    // Transforming the current pair type into 2 new types
                    let c = insertion.chars().nth(0).unwrap();
                    let new_pairs = [(pair.0, c), (c, pair.1)];
                    // remove this pair
                    *pair_counts.get_mut(&pair).unwrap() -= n;
                    // add the new ones
                    for new in new_pairs {
                        *pair_counts.entry(new).or_insert(0) += n;
                    }
                    // add the new char to counts
                    *counts.entry(c).or_insert(0) += n;
                }
            }

            // let counts_str = counts.iter()
            //     .filter(|(_, n)| **n > 0)
            //     .fold(String::new(), |s, (c, n)| format!("{} {}={}", s, c, n));
            // let total_count: usize = counts.iter().map(|(_, n)| n).sum();
            // let pairs_str = pair_counts.iter()
            //     .filter(|(_, n)| **n > 0)
            //     .fold(String::new(), |s, (c, n)| format!("{} {}{}={}", s, c.0, c.1, n));
            // let total_pairs: usize = pair_counts.values().sum();
            // assert_eq!(total_pairs, total_count - 1);
            // println!("After  {}:\n  counts:{} ({})\n  pairs: {}", i, counts_str, total_count, pairs_str);

        };

        for step in 1 ..= steps {
            growth_step(step);
        }

        counts
    }
}

pub fn part_1(input: impl Read, verbose: bool) -> usize {
    let mut p = Polymerisator::new(input);

    if verbose {
        println!("Template: {}", p.polymer());
    }
    for step in 1..=10 {
        p.grow();
        if verbose {
            if p.polymer.len() > 100 {
                println!("After {:2}: {}... ({})", step, &p.polymer()[..100], p.polymer().len());
            } else {
                println!("After {:2}: {} ({})", step, p.polymer(), p.polymer().len());
            }
        }
    }

    let counts = p.counts();
    let most_common = counts.iter().map(|(_, n)| n).max().unwrap();
    let least_common = counts.iter().map(|(_, n)| n).min().unwrap();

    most_common - least_common
}


// Gotta be smarter than brute force now...
// find stable cycles?
pub fn part_2(input: impl Read, _verbose: bool) -> usize {
    let p = Polymerisator::new(input);

    let counts = p.grow_and_get_counts(40);

    let most_common = counts.iter().map(|(_, n)| n).max().unwrap();
    let least_common = counts.iter().map(|(_, n)| n).min().unwrap();

    most_common - least_common
}
