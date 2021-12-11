use std::str::FromStr;

use anyhow::{bail, Context};

#[derive(Debug)]
struct Grid {
    data: Vec<i8>,
    n_columns: usize,
}

impl Grid {
    fn neighbors(&self, i: usize) -> Vec<usize> {
        let mut result = vec![];
        let to_index = |row: usize, col: usize| row * self.n_columns + col;
        let (r, c) = (i / self.n_columns, i % self.n_columns);
        if r > 0 {
            result.push(to_index(r - 1, c));
        }
        if r < (self.data.len() / self.n_columns) - 1 {
            result.push(to_index(r + 1, c));
        }
        if r > 0 && c > 0 {
            result.push(to_index(r - 1, c - 1));
        }
        if r < (self.data.len() / self.n_columns) - 1 && c > 0 {
            result.push(to_index(r + 1, c - 1));
        }
        if c > 0 {
            result.push(to_index(r, c - 1));
        }
        if c < self.n_columns - 1 {
            result.push(to_index(r, c + 1));
        }
        if r > 0 && c < self.n_columns - 1 {
            result.push(to_index(r - 1, c + 1));
        }
        if r < (self.data.len() / self.n_columns) - 1 && c < self.n_columns - 1 {
            result.push(to_index(r + 1, c + 1));
        }
        result
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s
            .split('\n')
            .map(|r| {
                r.chars()
                    .map(|c| c.to_digit(10).map(|n| n as i8))
                    .collect::<Option<Vec<_>>>()
            })
            .collect::<Option<Vec<_>>>()
            .context("Invalid input characters")?;
        if rows.is_empty() {
            bail!("No rows supplied");
        }
        let n_columns = rows[0].len();
        if rows.iter().skip(1).any(|v| v.len() != n_columns) {
            bail!("Not all rows of the same size");
        }
        Ok(Grid {
            data: rows.into_iter().flatten().collect(),
            n_columns,
        })
    }
}

fn simulate_step(grid: &mut Grid) -> usize {
    let mut to_trigger = (0..grid.data.len()).collect::<Vec<_>>();
    let mut triggered = vec![false; grid.data.len()];
    while !to_trigger.is_empty() {
        let i = to_trigger.pop().unwrap();
        grid.data[i] += 1;
        if grid.data[i] > 9 && !triggered[i] {
            triggered[i] = true;
            to_trigger.append(&mut grid.neighbors(i));
        }
    }
    for i in 0..grid.data.len() {
        if grid.data[i] > 9 {
            grid.data[i] = 0;
        }
    }
    triggered.into_iter().filter(|&x| x).count()
}

fn main() {
    let mut octopuses = Grid::from_str(aoc::get_input(21, 11).trim()).expect("Invalid input");

    let mut synced = None;
    let mut flashes = 0;
    for i in 1..=100 {
        let result = simulate_step(&mut octopuses);
        flashes += result;
        if result == octopuses.data.len() {
            synced = Some(i);
        }
    }
    println!("Part 1: {}", flashes);

    for i in 101.. {
        if synced.is_some() {
            break;
        }
        let result = simulate_step(&mut octopuses);
        if result == octopuses.data.len() {
            synced = Some(i);
        }
    }
    println!("Part 2: {}", synced.unwrap());
}
