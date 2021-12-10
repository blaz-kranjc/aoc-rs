use anyhow::Context;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

const COMPETITION_TIME: i32 = 2503;

struct Reindeer {
    speed: i32,
    run_time: i32,
    rest_time: i32,
}

impl FromStr for Reindeer {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REINDEER_REGEX: Regex =
                Regex::new(r"[A-Za-z]* can fly (?P<speed>\d+) km/s for (?P<run_time>\d+) seconds, but then must rest for (?P<rest_time>\d+) seconds.")
                    .unwrap();
        }
        let parts = REINDEER_REGEX
            .captures(s)
            .context("Invalid reindeer specification")?;
        Ok(Reindeer {
            speed: parts["speed"].parse()?,
            run_time: parts["run_time"].parse()?,
            rest_time: parts["rest_time"].parse()?,
        })
    }
}

fn distance_at(reindeer: &Reindeer, seconds: i32) -> i32 {
    let base = reindeer.run_time + reindeer.rest_time;
    let distance_base = (seconds / base) * reindeer.speed * reindeer.run_time;
    let remaining = seconds % base;
    if remaining > reindeer.run_time {
        distance_base + reindeer.speed * reindeer.run_time
    } else {
        distance_base + reindeer.speed * remaining
    }
}

fn max_at<T>(values: &[T]) -> Option<usize>
where
    T: Ord,
{
    values
        .iter()
        .enumerate()
        .max_by(|(_, l), (_, r)| l.cmp(r))
        .map(|(index, _)| index)
}

fn points_at(reindeers: &[Reindeer], seconds: i32) -> Vec<i32> {
    if reindeers.is_empty() {
        return vec![];
    }

    enum State {
        Resting(i32),
        Running(i32),
    }
    let mut scores = vec![0; reindeers.len()];
    let mut distances = vec![0; reindeers.len()];
    let mut states = reindeers
        .iter()
        .map(|r| State::Running(r.run_time))
        .collect::<Vec<_>>();
    for _ in 0..seconds {
        for i in 0..reindeers.len() {
            match states[i] {
                State::Resting(v) => {
                    if v > 1 {
                        states[i] = State::Resting(v - 1);
                    } else {
                        states[i] = State::Running(reindeers[i].run_time)
                    }
                }
                State::Running(v) => {
                    distances[i] += reindeers[i].speed;
                    if v > 1 {
                        states[i] = State::Running(v - 1);
                    } else {
                        states[i] = State::Resting(reindeers[i].rest_time)
                    }
                }
            }
        }
        scores[max_at(&distances).unwrap()] += 1;
    }
    scores
}

fn main() {
    let reindeers = aoc::get_input(15, 14)
        .split('\n')
        .filter_map(|s| Reindeer::from_str(s).ok())
        .collect::<Vec<_>>();
    println!(
        "Part 1: {}",
        reindeers
            .iter()
            .map(|r| distance_at(r, COMPETITION_TIME))
            .max()
            .expect("No reindeer provided")
    );
    println!(
        "Part 2: {}",
        points_at(&reindeers, COMPETITION_TIME)
            .iter()
            .max()
            .expect("No reindeer provided")
    );
}
