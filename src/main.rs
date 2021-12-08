use std::{path::PathBuf, error::Error, io, fmt::Display};

use structopt::StructOpt;
use anyhow::Result;

mod utils;
mod columns;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

#[derive(Debug, StructOpt)]
struct Opt {
    day: usize,
    input: PathBuf,
    #[structopt(short, long)]
    verbose: bool,
}

#[derive(Debug)]
struct DayError(usize);

impl Display for DayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unimplemented day: {}", self.0)
    }
}

impl Error for DayError {}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    match opt.day {
        1 => {
            let nums = utils::load_from_file(opt.input)?;
            println!("Part 1: {}", day1::increasing_pairs_count(&nums));
            println!("Part 2: {}", day1::windowed_increasing_count(&nums));
        },
        2 => {
            let cmds = utils::load_from_file(opt.input)?;
            let pos = day2::move_by(&cmds, |pos, cmd| pos.update1(cmd));
            println!("Part 1:\n  position: {:?}\n  result: {}", pos, pos.result());
            let pos = day2::move_by(&cmds, |pos, cmd| pos.update2(cmd));
            println!("Part 2:\n  position: {:?}\n  result: {}", pos, pos.result());
        },
        3 => {
            let vals = utils::load_from_file(opt.input)?;
            println!("Part 1");
            let (gamma, epsilon) = day3::calculate_gamma_epsilon(&vals);
            println!("  gamma rate   = {0:3} = 0b{0:05b}", gamma);
            println!("  epsilon rate = {0:3} = 0b{0:05b}", epsilon);
            println!("  => power consumption = {}", gamma * epsilon);
            println!("Part 2");
            let oxygen = day3::calculate_rating(&vals, day3::OxygenGenerator);
            let co2_scrubber = day3::calculate_rating(&vals, day3::CO2Scrubber);
            println!("  oxygen generator rating = {}", oxygen);
            println!("  CO2 scrubber rating     = {}", co2_scrubber);
            println!("  => life support rating = {}", oxygen * co2_scrubber);
        }
        4 => {
            let lines: Vec<String> = utils::load_from_file(opt.input)?;
            let (nums, mut boards) = day4::parse_lines(&lines)?;
            println!("Winner score = {}", day4::play_to_win(&nums, &mut boards, opt.verbose).expect("Should finish"));
            println!("Winner score = {}", day4::play_to_loose(&nums, &mut boards, opt.verbose).expect("Should finish"));
        },
        5 => {
            let lines: Vec<day5::Line> = utils::load_from_file(opt.input)?;
            let show = |vents: day5::VentsCount| {
                if opt.verbose {
                    println!("Hydrothermal vents:\n{}", vents);
                }
                println!("Dangerous areas count = {}", vents.dangerous_area_count());
            };
            println!("Part 1:");
            show(day5::VentsCount::non_diagonal(&lines));
            println!("Part 2:");
            show(day5::VentsCount::all(&lines));
        },
        6 => {
            let fish_start = day6::load_data(&utils::load_lines(opt.input)?);
            let fish = day6::simulate(fish_start.clone(), 80, opt.verbose);
            println!("Part 1:");
            println!("There are {} Lanternfish after 80 days", fish.len());
            println!("Part 2:");
            for days in [80, 256] {
                let n = day6::calculate_population(&fish_start, days, opt.verbose);
                println!("There are {} Lanternfish after {} days", n, days);
            }

        }
        day => Err(io::Error::new(io::ErrorKind::InvalidData, DayError(day)))?,
    }

    Ok(())
}
