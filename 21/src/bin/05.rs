use std::collections::HashSet;
use std::ops::{Add, Sub};
use std::str::FromStr;

use anyhow::{bail, Context};
use num_integer::gcd;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Point {
    fn normalize(&self) -> Point {
        let gcd = gcd(self.x, self.y);
        Point {
            x: self.x / gcd,
            y: self.y / gcd,
        }
    }
}

impl FromStr for Point {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let components = s.split(',').collect::<Vec<_>>();
        if components.len() != 2 {
            bail!("two integers required to parse a point")
        }
        let x = components[0].parse::<i32>()?;
        let y = components[1].parse::<i32>()?;
        Ok(Point { x, y })
    }
}

#[derive(Debug)]
struct Line(Point, Point);

struct PointIterator {
    current: Point,
    end: Point,
    step: Point,
    spent: bool,
}

impl PointIterator {
    fn new(begin: Point, end: Point) -> PointIterator {
        PointIterator {
            current: begin,
            end: end,
            step: (end - begin).normalize(),
            spent: false,
        }
    }
}

impl Iterator for PointIterator {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        if self.spent {
            None
        } else {
            if self.current == self.end {
                self.spent = true;
            }
            let result = self.current;
            self.current = self.current + self.step;
            Some(result)
        }
    }
}

impl Line {
    fn is_vertical(&self) -> bool {
        self.0.x == self.1.x
    }

    fn is_horizontal(&self) -> bool {
        self.0.y == self.1.y
    }

    fn follow(&self) -> impl Iterator<Item = Point> {
        return PointIterator::new(self.0, self.1);
    }
}

impl FromStr for Line {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let separator = " -> ";
        let separator_index = s.find(separator).context("No separator found")?;
        let origin = Point::from_str(&s[0..separator_index])?;
        let end = Point::from_str(&s[separator_index + separator.len()..])?;
        Ok(Line(origin, end))
    }
}

fn count_crossings<'a>(lines: impl Iterator<Item = &'a Line>) -> usize {
    let mut visited = HashSet::new();
    let mut duplicated = HashSet::new();
    for line in lines {
        for point in line.follow() {
            if visited.contains(&point) {
                duplicated.insert(point);
            } else {
                visited.insert(point);
            }
        }
    }

    duplicated.len()
}

fn main() {
    let lines = aoc::get_input(21, 5)
        .split('\n')
        .filter_map(|x| Line::from_str(x).ok())
        .collect::<Vec<_>>();

    println!(
        "Part 1: {}",
        count_crossings(
            lines
                .iter()
                .filter(|x| x.is_horizontal() || x.is_vertical())
        )
    );
    println!("Part 2: {}", count_crossings(lines.iter()));
}
