pub fn load_data<S: AsRef<str>>(lines: &[S]) -> Vec<Lanternfish> {
    lines[0].as_ref()
        .split(',')
        .map(|num| Lanternfish { timer: num.parse().unwrap() })
        .collect()
}

#[derive(Debug, Clone)]
pub struct Lanternfish {
    timer: usize,
}

const REPRODUCTION_PERIOD: usize = 7;
const FIRST_CYCLE_INCR: usize = 2;

impl Lanternfish {
    pub fn new() -> Self {
        Self { timer: REPRODUCTION_PERIOD + FIRST_CYCLE_INCR - 1 }
    }

    pub fn next_day(&mut self) -> Option<Lanternfish> {
        match self.timer {
            0 => {
                self.timer = REPRODUCTION_PERIOD - 1;
                Some(Lanternfish::new())
            },
            _ => {
                self.timer -= 1;
                None
            }
        }
    }
}

pub fn simulate(mut fish: Vec<Lanternfish>, days: usize, verbose: bool) -> Vec<Lanternfish> {
    for day in 1..=days {
        let mut new = fish.iter_mut()
            .filter_map(Lanternfish::next_day)
            .collect();
        fish.append(&mut new);

        if verbose {
            let timers: Vec<_> = fish.iter().map(|f| f.timer.to_string()).collect();
            println!("After {:2} days: {}", day, timers.as_slice().join(","))
        }
    }

    fish
}

pub fn calculate_population(initial: &[Lanternfish], days: usize, verbose: bool) -> usize {
    // Instead of creating objects we can just keep track of counts of fish with given timer value.
    // `counts[n]` is the number of fish that have the `timer` value of `n`.
    let mut counts = [0usize; REPRODUCTION_PERIOD + 2];

    for fish in initial {
        counts[fish.timer] += 1;
    }

    if verbose {
        println!("Initial: {:?}", counts);
    }

    for day in 1..=days {
        // the ones with timer=0 will create offspring
        let offspring = counts[0];

        // Decrement the timers
        // Need to move the ones from last element because timer=0 goes to 6 not 8
        counts.rotate_left(1);
        counts[REPRODUCTION_PERIOD - 1] += counts[REPRODUCTION_PERIOD + FIRST_CYCLE_INCR - 1];

        // ...and add the offspring overwriting the already-moved fish
        counts[REPRODUCTION_PERIOD + FIRST_CYCLE_INCR - 1] = offspring;

        if verbose {
            println!("After day {} (+ {}) = {:?} #{}", day, offspring, counts, counts.iter().sum::<usize>());
        }
    }

    counts.iter().sum()
}
