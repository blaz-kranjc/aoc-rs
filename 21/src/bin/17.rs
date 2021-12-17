use std::str::FromStr;

use anyhow::Context;
use lazy_static::lazy_static;
use num_integer::Roots;
use regex::Regex;

#[derive(Debug)]
struct Target {
    xs: (i64, i64),
    ys: (i64, i64),
}

impl FromStr for Target {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RANGE_REGEX: Regex =
                Regex::new(r"target area: x=(?P<x0>-?[0-9]+)..(?P<x1>-?[0-9]+), y=(?P<y0>-?[0-9]+)..(?P<y1>-?[0-9]+)").unwrap();
        }
        let parts = RANGE_REGEX
            .captures(s)
            .context("Invalid reindeer specification")?;
        Ok(Target {
            xs: (parts["x0"].parse()?, parts["x1"].parse()?),
            ys: (parts["y0"].parse()?, parts["y1"].parse()?),
        })
    }
}

fn find_highest_and_count(t: &Target) -> (i64, i64) {
    let mut max_height = 0;
    let mut count = 0;
    for vx in (2 * t.xs.0).sqrt()..=t.xs.1 {
        for vy in t.ys.0..=(-t.ys.0) {
            let mut vx = vx;
            let mut vy = vy;
            let mut x = 0;
            let mut y = 0;
            let mut current_max = 0;
            loop {
                current_max = std::cmp::max(current_max, y);
                if y < t.ys.0 || x > t.xs.1 {
                    break;
                }
                if (t.xs.0..=t.xs.1).contains(&x) && (t.ys.0..=t.ys.1).contains(&y) {
                    count += 1;
                    max_height = std::cmp::max(max_height, current_max);
                    break;
                }
                x += std::cmp::max(vx, 0);
                y += vy;
                vx -= 1;
                vy -= 1;
            }
        }
    }
    (max_height, count)
}

fn main() {
    let target = Target::from_str(aoc::get_input(21, 17).trim()).expect("Invalid input");
    let (max, count) = find_highest_and_count(&target);

    println!("Part 1: {}", max);
    println!("Part 2: {}", count);
}
