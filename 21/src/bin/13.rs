use std::str::FromStr;

use anyhow::{bail, Context};

#[derive(Debug)]
enum Fold {
    Vertical(i32),
    Horizontal(i32),
}

impl FromStr for Fold {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let prefix = "fold along ";
        if !s.starts_with(prefix) {
            bail!("Unknown format");
        }
        let mut parts = s[prefix.len()..].split('=');
        let dir = parts.next().context("Missing direction")?;
        let column = parts.next().context("Missing column")?.parse()?;
        match dir {
            "x" => Ok(Fold::Horizontal(column)),
            "y" => Ok(Fold::Vertical(column)),
            _ => bail!("Unknown direction"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts.next().context("Missing x")?.parse()?;
        let y = parts.next().context("Missing y")?.parse()?;
        if parts.next().is_some() {
            bail!("Too many parts provided");
        }
        Ok(Point { x, y })
    }
}

fn apply_fold_single(p: &Point, fold: &Fold) -> Point {
    let x = match fold {
        &Fold::Horizontal(amount) => {
            if p.x > amount {
                2 * amount - p.x
            } else {
                p.x
            }
        }
        _ => p.x,
    };
    let y = match fold {
        &Fold::Vertical(amount) => {
            if p.y > amount {
                2 * amount - p.y
            } else {
                p.y
            }
        }
        _ => p.y,
    };
    Point { x, y }
}

fn apply(points: &[Point], fold: &Fold) -> Vec<Point> {
    points.iter().fold(vec![], |mut acc, p| {
        let point = apply_fold_single(p, fold);
        if !acc.contains(&point) {
            acc.push(point)
        }
        acc
    })
}

fn limits(points: &[Point]) -> (i32, i32) {
    let mut x_max = 0;
    let mut y_max = 0;
    for &Point { x, y } in points {
        if x > x_max {
            x_max = x;
        }
        if y > y_max {
            y_max = y;
        }
    }
    (x_max, y_max)
}

fn display(points: &[Point]) {
    let limits = limits(points);
    for y in 0..=limits.1 {
        for x in 0..=limits.0 {
            if points.contains(&Point { x, y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn main() {
    let input = aoc::get_input(21, 13)
        .trim()
        .split('\n')
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let split_position = input
        .iter()
        .position(|s| s.is_empty())
        .expect("invalid input");
    let points = &input[0..split_position]
        .iter()
        .map(|s| Point::from_str(s))
        .collect::<anyhow::Result<Vec<_>>>()
        .expect("Invalid input");
    let folds = &input[split_position + 1..]
        .iter()
        .map(|s| Fold::from_str(s))
        .collect::<anyhow::Result<Vec<_>>>()
        .expect("Invalid input");

    let after_first_fold = apply(points, &folds[0]);
    println!("Part 1: {}", after_first_fold.len());
    println!("Part 2:");
    let code = folds
        .iter()
        .skip(1)
        .fold(after_first_fold, |v, f| apply(&v, f));
    display(&code);
}
