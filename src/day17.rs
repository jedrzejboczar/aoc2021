use std::io::Read;

use regex::Regex;

#[derive(Debug)]
struct Target {
    x_start: i32,
    x_end: i32,
    y_start: i32,
    y_end: i32,
}

impl Target {
    fn contains(&self, pos: &(i32, i32)) -> bool {
        pos.0 >= self.x_start && pos.0 <= self.x_end
            && pos.1 >= self.y_start && pos.1 <= self.y_end
    }

    fn try_shoot(&self, mut velocity: (i32, i32)) -> (bool, Vec<(i32, i32)>) {
        let mut pos = (0, 0);
        let mut path = vec![];
        let x_dir = velocity.0.signum();

        loop {
            path.push(pos);

            // check if we hit the target
            if self.contains(&pos) {
                return (true, path);
            }

            // we stop when we overshoot the target in y or x
            let y_overshoot = pos.1 < self.y_end;
            let x_overshoot = if x_dir > 0 {
                pos.0 > self.x_end
            } else if x_dir < 0 {
                pos.0 < self.x_start
            } else {
                false
            };
            if x_overshoot || (velocity.0 == 0 && y_overshoot) {
                break;
            }

            // increment position
            pos = (pos.0 + velocity.0, pos.1 + velocity.1);

            // decrease velocity
            let new_x = (velocity.0.abs() - 1) * velocity.0.signum();
            let new_y = velocity.1 - 1;
            velocity = (new_x, new_y);
        }

        (false, path)
    }
}

fn load_data(mut input: impl Read) -> Target {
    let mut s = String::new();
    input.read_to_string(&mut s).unwrap();
    let re = Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();
    let captures = re.captures(&s).unwrap();
    let x1: i32 = captures[1].parse().unwrap();
    let x2: i32 = captures[2].parse().unwrap();
    let y1: i32 = captures[3].parse().unwrap();
    let y2: i32 = captures[4].parse().unwrap();
    Target {
        x_start: x1.min(x2),
        x_end: x1.max(x2),
        y_start: y1.min(y2),
        y_end: y1.max(y2),
    }
}

fn display(target: &Target, path: &[(i32, i32)]) {
    let min_x = target.x_start.min(*path.iter().map(|(x, _)| x).min().unwrap_or(&target.x_start));
    let max_x = target.x_end.max(*path.iter().map(|(x, _)| x).max().unwrap_or(&target.x_end));
    let min_y = target.y_start.min(*path.iter().map(|(_, y)| y).min().unwrap_or(&target.y_start));
    let max_y = target.y_end.max(*path.iter().map(|(_, y)| y).max().unwrap_or(&target.y_end));

    for y in (min_y ..= max_y).rev() {
        for x in min_x ..= max_x {
            let pos = (x, y);
            let c = if pos == (0, 0) {
                "S"
            } else if path.contains(&pos) {
                "#"
            } else if target.contains(&pos) {
                "T"
            } else {
                "."
            };
            print!("{}", c);
        }
        println!();
    }
}

fn highest_point(path: &[(i32, i32)]) -> i32 {
    path.iter()
        .map(|(_x, y)| *y)
        .max()
        .unwrap()
}

pub fn part_1(input: impl Read, verbose: bool) -> usize {
    let target = load_data(input);

    // Examples
    // for vel in [(7, 2), (6, 3), (9, 0), (17, -4)] {
    //     let (ok, path) = target.try_shoot(vel);
    //     println!("shoot {:?} -> {} : {:?}", vel, ok, path);
    //     display(&target, &path);
    // }

    // X "range" is a sum of an arithmetic series (x,x-1,x-2,...) with (x+1) elements
    // xr = N * (x + 0) / 2
    let x_range = |x: i32| (x + 1) * (x + 0) / 2;
    let xs = (0..)
        .skip_while(|x| x_range(*x) < target.x_start)
        .take_while(|x| x_range(*x) <= target.x_end);

    // Test all Ys starting from the one that would reach bottom of target in 1 iteration,
    // and doing this <how long?>
    let mut highest = i32::MIN;
    let mut best_path = vec![];
    for y in target.y_start.. {
        for x in xs.clone() {
            let (ok, path) = target.try_shoot((x, y));
            if ok {
                highest = highest.max(highest_point(&path));
                best_path = path;
            }
        }

        // brute force!
        if y > 1000 {
            break;
        }
    }

    // if verbose {
    //     display(&target, &best_path);
    // }

    highest as usize
}

pub fn part_2(input: impl Read, verbose: bool) -> usize {
    let target = load_data(input);

    // and built in --release mode!
    const BRUTE_FORCE: i32 = 5000;

    // X "range" is a sum of an arithmetic series (x,x-1,x-2,...) with (x+1) elements
    // xr = N * (x + 0) / 2
    let x_range = |x: i32| (x + 1) * (x + 0) / 2;
    let xs = (0..)
        .skip_while(|x| x_range(*x) < target.x_start)
        .take(BRUTE_FORCE as usize);

    // Test all Ys starting from the one that would reach bottom of target in 1 iteration,
    // and doing this <how long?>
    let mut highest = i32::MIN;
    let mut best_path = vec![];
    let mut vels = vec![];
    for y in target.y_start..BRUTE_FORCE {
        for x in xs.clone() {
            let (ok, path) = target.try_shoot((x, y));
            if ok {
                highest = highest.max(highest_point(&path));
                best_path = path;
                vels.push((x, y));
            }
        }
    }

    if verbose {
        println!("Velocites:\n{:?}", vels);
    }

    vels.len()
}
