pub fn load_data<S: AsRef<str>>(lines: &[S]) -> Vec<usize> {
    lines[0].as_ref()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect()
}

#[allow(dead_code)]
fn median(nums: &[usize]) -> usize {
    let mut nums = nums.to_vec();
    nums.sort();
    let n = nums.len();
    let mid = &nums[n / 2 ..= (n + 1) / 2];
    let n = mid.len();
    let sum = mid.into_iter()
        .map(|v| *v as f32)
        .sum::<f32>();
    (sum / n as f32).round() as usize
}

#[derive(Debug)]
pub struct BestResult {
    pos: usize,
    cost: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum FuelCost {
    Constant,
    Increasing,
}

fn fuel_cost(crabs: &[usize], pos: usize, rate: FuelCost) -> usize {
    let abs_diff = |a, b| if a > b { a - b } else { b - a };
    let diffs = crabs.iter().map(|p| abs_diff(pos, *p));
    match rate {
        FuelCost::Constant => diffs.sum(),
        FuelCost::Increasing => diffs
            // .map(|diff| (1..=diff).sum::<usize>())
            .map(|diff| diff * (diff + 1) / 2)  // analytical
            .sum()
    }
}

fn best_position_brute_force(crabs: &[usize], rate: FuelCost) -> usize {
    let (min, max) = crabs.iter()
        .fold((usize::MAX, usize::MIN),
              |(min, max), pos| (min.min(*pos), max.max(*pos)));
    (min..=max)
        .min_by_key(|pos| fuel_cost(crabs, *pos, rate))
        .unwrap()
}

#[allow(dead_code)]
fn best_position_analytical(crabs: &[usize]) -> usize {
    // let sum: usize = crabs.iter().sum();
    // let mean = (sum as f32 / crabs.len() as f32).round() as usize;
    // let median = median(crabs);
    // println!("sum={}, mean={}, median={}", sum, mean, median);
    // median
    median(crabs)
}

pub fn best_position(crabs: &[usize], rate: FuelCost) -> BestResult {
    let brute_force = best_position_brute_force(crabs, rate);
    // let analytical = best_position_analytical(crabs);
    // assert_eq!(analytical, brute_force);
    let pos = brute_force;
    BestResult { pos, cost: fuel_cost(crabs, pos, rate) }
}
