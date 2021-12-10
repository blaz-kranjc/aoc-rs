use anyhow::{bail, Context};
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Point(usize, usize);

#[derive(Debug, Clone)]
struct Rectangle(Point, Point);

struct PointIterator {
    rectangle: Rectangle,
    current: Point,
    spent: bool,
}

impl PointIterator {
    fn new(rectangle: Rectangle) -> PointIterator {
        let current = rectangle.0.clone();
        PointIterator {
            rectangle,
            current,
            spent: false,
        }
    }
}

impl Iterator for PointIterator {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        if self.spent {
            None
        } else if self.current == self.rectangle.1 {
            self.spent = true;
            Some(self.current.clone())
        } else {
            let result = self.current.clone();
            if self.current.0 == self.rectangle.1 .0 {
                self.current.0 = self.rectangle.0 .0;
                self.current.1 += 1;
            } else {
                self.current.0 += 1;
            }
            Some(result)
        }
    }
}

impl Rectangle {
    fn points(&self) -> impl Iterator<Item = Point> {
        PointIterator::new(self.clone())
    }
}

#[derive(Debug)]
enum Instruction {
    TurnOn(Rectangle),
    TurnOff(Rectangle),
    Toggle(Rectangle),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref INSTRUCTION_REGEX: Regex = Regex::new(r"(?P<type>turn on|turn off|toggle) (?P<x1>\d+),(?P<y1>\d+) through (?P<x2>\d+),(?P<y2>\d+)").unwrap();
        }
        let parts = INSTRUCTION_REGEX
            .captures(s)
            .context("Invalid instruction layout")?;
        let rectangle = Rectangle(
            Point(parts["x1"].parse::<usize>()?, parts["y1"].parse::<usize>()?),
            Point(parts["x2"].parse::<usize>()?, parts["y2"].parse::<usize>()?),
        );
        match &parts["type"] {
            "turn on" => Ok(Instruction::TurnOn(rectangle)),
            "turn off" => Ok(Instruction::TurnOff(rectangle)),
            "toggle" => Ok(Instruction::Toggle(rectangle)),
            _ => bail!("Invalid instruction type"),
        }
    }
}

struct LightsBool([[bool; 1000]; 1000]);

impl LightsBool {
    fn new() -> LightsBool {
        LightsBool([[false; 1000]; 1000])
    }

    fn total_brightness(&self) -> usize {
        self.0
            .iter()
            .map(|row| row.iter().filter(|&&x| x).count())
            .sum()
    }

    fn apply(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::TurnOn(rect) => rect.points().for_each(|Point(x, y)| self.0[x][y] = true),
            Instruction::TurnOff(rect) => {
                rect.points().for_each(|Point(x, y)| self.0[x][y] = false)
            }
            Instruction::Toggle(rect) => rect
                .points()
                .for_each(|Point(x, y)| self.0[x][y] = !self.0[x][y]),
        }
    }
}

struct LightsVariable(Vec<i32>);

impl LightsVariable {
    fn new() -> LightsVariable {
        LightsVariable(vec![0; 1_000_000])
    }

    fn total_brightness(&self) -> i32 {
        self.0.iter().sum()
    }

    fn index(&self, row: usize, column: usize) -> usize {
        1000 * row + column
    }

    fn apply(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::TurnOn(rect) => rect.points().for_each(|Point(x, y)| {
                let index = self.index(x, y);
                self.0[index] += 1
            }),
            Instruction::TurnOff(rect) => rect.points().for_each(|Point(x, y)| {
                let index = self.index(x, y);
                self.0[index] = cmp::max(self.0[index] - 1, 0)
            }),
            Instruction::Toggle(rect) => rect.points().for_each(|Point(x, y)| {
                let index = self.index(x, y);
                self.0[index] += 2
            }),
        }
    }
}

fn main() {
    let instructions = aoc::get_input(15, 6)
        .split('\n')
        .map(Instruction::from_str)
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    let mut lights_bool = LightsBool::new();
    instructions.iter().for_each(|i| lights_bool.apply(i));
    println!("Part 1: {}", lights_bool.total_brightness());

    let mut lights_variable = LightsVariable::new();
    instructions.iter().for_each(|i| lights_variable.apply(i));
    println!("Part 2: {}", lights_variable.total_brightness());
}
