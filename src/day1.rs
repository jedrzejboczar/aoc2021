// Part 1
pub fn increasing_pairs_count(nums: &[isize]) -> usize {
    nums.windows(2)
        .filter(|window| window[0] < window[1])
        .count()
}

// Part 2
pub fn windowed_increasing_count(nums: &[isize]) -> usize {
    let sums: Vec<_> = nums.windows(3)
        .map(|win| win.iter().sum())
        .collect();
    increasing_pairs_count(&sums)
}
