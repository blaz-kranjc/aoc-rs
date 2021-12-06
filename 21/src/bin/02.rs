use anyhow::{bail, Result};
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, PartialEq, Eq, EnumString)]
enum Direction {
    #[strum(serialize = "forward")]
    Forward,
    #[strum(serialize = "up")]
    Up,
    #[strum(serialize = "down")]
    Down,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    amount: i32,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();
        if parts.len() != 2 {
            bail!("Invalid number of parts");
        }
        let direction = parts[0].parse::<Direction>()?;
        let amount = parts[1].parse::<i32>()?;
        Ok(Instruction { direction, amount })
    }
}

#[derive(Debug, Default)]
struct AimPosition {
    position: i32,
    depth: i32,
    aim: i32,
}

fn follow(current: AimPosition, i: &Instruction) -> AimPosition {
    match i.direction {
        Direction::Forward => AimPosition {
            position: current.position + i.amount,
            depth: current.depth + i.amount * current.aim,
            ..current
        },
        Direction::Down => AimPosition {
            aim: current.aim + i.amount,
            ..current
        },
        Direction::Up => AimPosition {
            aim: current.aim - i.amount,
            ..current
        },
    }
}

fn main() {
    let instructions = aoc::get_input(21, 2)
        .split('\n')
        .map(Instruction::from_str)
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    let end_pos = instructions.iter().fold(AimPosition::default(), follow);
    println!("Part 1: {}", end_pos.aim * end_pos.position);
    println!("Part 2: {}", end_pos.depth * end_pos.position);
}
