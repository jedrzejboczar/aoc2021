use std::{fmt::Display, collections::HashMap, str::FromStr};

#[derive(Debug)]
pub struct UnexpectedChar(char);

impl Display for UnexpectedChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unexpected character: {}", self.0)
    }
}

impl std::error::Error for UnexpectedChar {}


#[derive(Clone)]
pub struct BitInput(usize);

impl FromStr for BitInput {
    type Err = UnexpectedChar;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut value = 0;

        for (i, c) in s.chars().enumerate() {
            let mask = match c {
                '0' => 0,
                '1' =>  1 << i,  // bits in input are LSB first
                _ => return Err(UnexpectedChar(c)),
            };
            value |= mask;
        }

        Ok(BitInput(value))
    }
}

#[allow(dead_code)]
fn print_values(values: &[usize]) {
    println!("values:");
    for v in values.iter() {
        println!("  {:05b}", v);
    }
}

fn get_bit_counts(values: &[BitInput]) -> HashMap<usize, usize> {
    let mut bit_counts = HashMap::new();

    for val in values {
        // iterate over bits in the value, shifting the value to the right
        let mut val = val.0;
        let mut bit = 0;

        while val != 0 {
            // if current bit is 1 then update the count
            let count = bit_counts.entry(bit).or_insert(0);
            if (val & 1) != 0 {
                *count += 1;
            }
            // shift the value and move to the next bit
            val = val >> 1;
            bit += 1;
        }
    }

    bit_counts
}

pub fn calculate_gamma_epsilon(values: &[BitInput]) -> (usize, usize) {
    assert!(values.len() > 0, "`values` must be non-empty slice");

    // we now have counts of 1s for each bit position, number of 0s is total minus 1s
    let bit_counts = get_bit_counts(values);
    let total_count = values.len();
    let n_bits = bit_counts.keys().max().unwrap() + 1;

    let mut gamma = 0;
    for bit in 0..n_bits {
        let n_ones = bit_counts[&bit];
        let n_zeros = total_count - n_ones;
        assert!(n_ones != n_zeros, "What to do if there is the same number of 0s and 1s?");
        if n_ones > n_zeros {
            // gamma uses the most common bit, so if it's 1 than we need to OR
            // we again need to use reversed (LSB first order)
            gamma |= 1 << (n_bits - 1 - bit);
        }
    }

    // epsilon is just bitwise negation of gamma (masked to n_bits)
    let mask = (1 << n_bits) - 1;
    let epsilon = !gamma & mask;

    (gamma, epsilon)
}

pub trait BitCriteria {
    fn keep(&self, value: usize, tested_bit: usize, n_ones: usize, n_zeros: usize) -> bool;
}

pub fn calculate_rating(values: &[BitInput], bit_criteria: impl BitCriteria) -> usize
{
    // needed to reverse the number ...
    let n_bits = get_bit_counts(&values).keys().max().unwrap() + 1;

    let mut values: Vec<_> = values.iter().cloned().collect();
    let mut bit = 0;

    // "if you have one number left, stop"
    while values.len() > 1 {
        // println!("bit = {}", bit);

        let bit_counts = get_bit_counts(values.as_slice());
        let n_ones = *bit_counts.get(&bit).expect("Infinite loop?");
        let n_zeros = values.len() - n_ones;

        values = values.iter()
            .cloned()
            .filter(|val| bit_criteria.keep(val.0, bit, n_ones, n_zeros))
            .collect();

        // values = values.iter()
        //     .copied()
        //     .enumerate()
        //     .filter(|(i, val)| {
        //         let keep = bit_criteria.keep(*val, bit, n_ones, n_zeros);
        //         println!("  {:2}: {:05b} {}", i, val, if keep { "keep" } else { "remove" });
        //         keep
        //     })
        //     .map(|(i, val)| val)
        //     .collect();

        bit += 1;
    }

    // now we need to reverse bits...
    let mut result = 0;
    for bit in 0..n_bits {
        let bit_val = ((1 << bit) & values[0].0) != 0;
        if bit_val {
            result |= 1 << (n_bits - 1 - bit);
        }
    }

    result
}

pub struct OxygenGenerator;
pub struct CO2Scrubber;

impl BitCriteria for OxygenGenerator {
    fn keep(&self, value: usize, tested_bit: usize, n_ones: usize, n_zeros: usize) -> bool {
        let desired_bit_val = if n_ones >= n_zeros {
            1
        } else {
            0
        };
        // "keep values with that bit in that position"
        ((value >> tested_bit) & 1) == desired_bit_val
    }
}

impl BitCriteria for CO2Scrubber {
    fn keep(&self, value: usize, tested_bit: usize, n_ones: usize, n_zeros: usize) -> bool {
        let desired_bit_val = if n_ones >= n_zeros {
            0
        } else {
            1
        };
        // "keep values with that bit in that position"
        ((value >> tested_bit) & 1) == desired_bit_val
    }
}

